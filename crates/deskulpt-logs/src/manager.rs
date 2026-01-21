//! Deskulpt logs manager and its APIs.

use std::path::{Path, PathBuf};

use anyhow::Result;
use tauri::{AppHandle, Manager, Runtime};
use tracing::Level;
use tracing_appender::non_blocking::{NonBlockingBuilder, WorkerGuard};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::filter::Targets;
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{Layer, Registry, fmt};

use crate::reader::{Cursor, Page, RollingTailReader};

/// Manager for Deskulpt logs.
pub struct LogsManager<R: Runtime> {
    /// The Tauri app handle.
    _app_handle: AppHandle<R>,
    /// The directory where log files are stored.
    dir: PathBuf,
    /// A guard that flushes pending logs when dropped.
    _guard: WorkerGuard,
}

impl<R: Runtime> LogsManager<R> {
    /// Initialize state management for logs.
    ///
    /// This will set up structured logging in newline-delimited JSON format
    /// with daily rotation. The lifetime of the logger is tied to the lifetime
    /// of the manager.
    pub fn new(app_handle: AppHandle<R>) -> Result<Self> {
        let dir = app_handle.path().app_log_dir()?;
        std::fs::create_dir_all(&dir)?;

        let appender = RollingFileAppender::builder()
            .rotation(Rotation::DAILY)
            .max_log_files(10)
            .filename_prefix("deskulpt")
            .filename_suffix("log")
            .build(&dir)?;

        let (writer, guard) = NonBlockingBuilder::default().finish(appender);

        let file_layer = fmt::layer()
            .json()
            .with_target(true)
            .with_file(true)
            .with_line_number(true)
            .with_timer(UtcTime::rfc_3339())
            .with_current_span(false)
            .with_span_list(true)
            .flatten_event(true)
            .with_writer(writer)
            .with_filter(
                Targets::new()
                    .with_target("deskulpt", Level::TRACE)
                    .with_target("frontend::canvas", Level::TRACE)
                    .with_target("frontend::manager", Level::TRACE),
            );

        let subscriber = Registry::default().with(file_layer);
        tracing::subscriber::set_global_default(subscriber)?;

        // Set up panic hook to log uncaught panics
        let previous_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic_info| {
            tracing_panic::panic_hook(panic_info);
            previous_hook(panic_info);
        }));

        Ok(Self {
            dir,
            _app_handle: app_handle,
            _guard: guard,
        })
    }

    /// Get the directory where log files are stored.
    pub fn dir(&self) -> &Path {
        &self.dir
    }

    /// Collect log files in most recent first order.
    fn collect(&self) -> Result<Vec<PathBuf>> {
        let mut files = std::fs::read_dir(&self.dir)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                let name = path.file_name()?.to_string_lossy();
                if name.starts_with("deskulpt.") && name.ends_with(".log") {
                    Some(path)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        // Here we assume that the filenames are timestamps, so sorting by
        // filename in descending order should correspond to most recent first
        files.sort_by(|a, b| b.file_name().cmp(&a.file_name()));
        Ok(files)
    }

    /// Read a page of log entries.
    ///
    /// This will filter up to `limit` entries that are at least the severity of
    /// `min_level`. If a cursor is provided, it will continue reading from the
    /// specified position. Otherwise, it will start from the latest entry.
    pub fn read(&self, limit: usize, min_level: Level, cursor: Option<Cursor>) -> Result<Page> {
        let files = self.collect()?;
        let mut reader = RollingTailReader::new(files, min_level);
        reader.read(limit, cursor)
    }

    /// Clear all log files and return the freed disk space in bytes.
    ///
    /// The latest log file is truncated instead of deleted to ensure that
    /// logging can continue without interruption. All older log files are
    /// deleted.
    ///
    /// The only possible failure is failure to collect the log files to clear
    /// (before actual operations begin). Failure to delete or truncate a log
    /// file will not result in an error, but will not contribute to the
    /// computed freed space.
    pub fn clear(&self) -> Result<u64> {
        let log_files = self.collect()?;

        let mut total_size: u64 = log_files
            .iter()
            .skip(1)
            .filter_map(|file| {
                let size = file.metadata().ok().map(|m| m.len());
                std::fs::remove_file(file).ok().and(size)
            })
            .sum();

        if let Some(latest_file) = log_files.first() {
            let size = latest_file.metadata().map(|m| m.len()).unwrap_or(0);
            if std::fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(latest_file)
                .is_ok()
            {
                total_size += size;
            }
        }

        Ok(total_size)
    }
}
