//! State management for logging.

use std::path::PathBuf;

use anyhow::Result;
use tauri::{App, AppHandle, Manager, Runtime};
use tracing::Level;
use tracing_appender::non_blocking::{NonBlockingBuilder, WorkerGuard};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::filter::Targets;
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{Layer, Registry, fmt};

use crate::path::PathExt;

/// Maximum number of log files to retain.
const MAX_LOG_FILES: usize = 10;

/// Managed state for logging.
struct LoggingState {
    /// Guard that flushes the tracing worker on drop.
    _guard: WorkerGuard,
}

/// Extension trait for operations related to logging.
pub trait LoggingStateExt<R: Runtime>: Manager<R> + PathExt<R> {
    /// Initialize state management for logging.
    ///
    /// This will set up structured logging in newline-delimited JSON format
    /// with daily rotation and maximum [`MAX_LOG_FILES`] log files retained.
    /// A panic hook is also set up to log uncaught panics.
    fn manage_logging(&self) -> Result<()> {
        let logs_dir = self.logs_dir()?;

        let appender = RollingFileAppender::builder()
            .rotation(Rotation::DAILY)
            .max_log_files(MAX_LOG_FILES)
            .filename_prefix("deskulpt")
            .filename_suffix("log")
            .build(logs_dir)?;
        let (writer, guard) = NonBlockingBuilder::default().finish(appender);

        let file_layer = fmt::layer()
            .json()
            .flatten_event(true)
            .with_timer(UtcTime::rfc_3339())
            .with_writer(writer.clone())
            .with_filter(
                Targets::new()
                    .with_target("deskulpt", Level::TRACE)
                    .with_target("frontend::canvas", Level::TRACE)
                    .with_target("frontend::manager", Level::TRACE),
            );

        let subscriber = Registry::default().with(file_layer);
        tracing::subscriber::set_global_default(subscriber)?;

        let previous_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic_info| {
            tracing_panic::panic_hook(panic_info);
            previous_hook(panic_info);
        }));

        self.manage(LoggingState { _guard: guard });
        Ok(())
    }

    /// Discover log files and return their paths by newest first.
    ///
    /// This looks for log files in the logs directory with names matching the
    /// pattern `deskulpt.*.log`, where `*` should be a timestamp though this is
    /// not verified here. The returned list is sorted by filename in descending
    /// order, which should correspond to most recent first if the `*`s are
    /// indeed timestamps.
    fn collect_logs(&self) -> Result<Vec<PathBuf>> {
        let logs_dir = self.logs_dir()?;

        let mut files = std::fs::read_dir(logs_dir)?
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

        files.sort_by(|a, b| b.file_name().cmp(&a.file_name()));
        Ok(files)
    }

    /// Clear all log files and return the freed disk space in bytes.
    ///
    /// The latest log file is truncated instead of deleted to ensure that
    /// logging can continue without interruption. All older log files are
    /// deleted. The total freed disk space is returned.
    ///
    /// Note that failure to delete or truncate a log file will not result in an
    /// error, but will not contribute to the computed freed space. Failure to
    /// discover the log files in the first place (before actual clearing
    /// begins), however, willl result in an error.
    fn clear_logs(&self) -> Result<u64> {
        let log_files = self.collect_logs()?;

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

impl<R: Runtime> LoggingStateExt<R> for App<R> {}
impl<R: Runtime> LoggingStateExt<R> for AppHandle<R> {}
