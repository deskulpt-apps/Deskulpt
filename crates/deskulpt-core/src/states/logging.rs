//! State management for logging.

use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::mem::ManuallyDrop;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::Result;
use tauri::{App, AppHandle, Manager, Runtime};
use tracing::span::Entered;
use tracing::{Level, Span, info_span};
use tracing_appender::non_blocking::{NonBlockingBuilder, WorkerGuard};
use tracing_subscriber::filter::{LevelFilter, Targets};
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::layer::{Layer, SubscriberExt};
use tracing_subscriber::{Registry, fmt};
use uuid::Uuid;

use crate::logging::SpanContextLayer;
use crate::path::PathExt;

/// Maximum number of log files (including the active log) to retain.
const MAX_LOG_FILES: usize = 10;
/// Maximum size per log file before forcing a rotation.
const MAX_LOG_FILE_SIZE_BYTES: u64 = 10 * 1024 * 1024;
/// Time-to-live for rotated log files.
const MAX_LOG_FILE_AGE: Duration = Duration::from_secs(60 * 60 * 24 * 7);
/// Name of the active log file.
const ACTIVE_LOG_FILENAME: &str = "deskulpt.log";

/// Managed state for logging.
struct LoggingState {
    /// Guard that flushes the tracing worker on drop.
    _guard: WorkerGuard,
    /// Guard that keeps the root runtime span entered for the app lifetime.
    _runtime_span: RuntimeSpanGuard,
}

/// Extension trait for operations related to logging.
pub trait LoggingStateExt<R: Runtime>: Manager<R> + PathExt<R> {
    /// Initialize state management for logging.
    ///
    /// This sets up newline-delimited JSON logging with file rotation, enforces
    /// retention limits, and registers a panic hook so crashes are captured.
    fn manage_logging(&self) -> Result<()> {
        let logs_dir = self.logs_dir()?;
        cleanup_logs_dir(logs_dir)?;

        let appender = SizeCappedAppender::new(logs_dir, MAX_LOG_FILE_SIZE_BYTES)?;
        let (writer, guard) = NonBlockingBuilder::default().finish(appender);
        let fmt_layer = fmt::layer()
            .json()
            .with_ansi(false)
            .with_file(true)
            .with_line_number(true)
            .with_timer(UtcTime::rfc_3339())
            .flatten_event(true)
            .with_writer(writer);

        // Deskulpt crates default to INFO to avoid noisy third-party TRACE logs.
        let filter = Targets::new()
            .with_target("deskulpt", Level::INFO)
            .with_target("deskulpt_core", Level::INFO)
            .with_target("deskulpt_widgets", Level::INFO)
            .with_default(Level::WARN);

        #[cfg(debug_assertions)]
        let subscriber = {
            let console_layer = fmt::layer()
                .with_ansi(true)
                .with_target(true)
                .with_writer(std::io::stderr)
                .with_filter(LevelFilter::DEBUG);
            Registry::default()
                .with(filter)
                .with(SpanContextLayer::new())
                .with(fmt_layer)
                .with(console_layer)
        };
        #[cfg(not(debug_assertions))]
        let subscriber = Registry::default()
            .with(filter)
            .with(SpanContextLayer::new())
            .with(fmt_layer);
        tracing::subscriber::set_global_default(subscriber)?;

        let session_id = Uuid::new_v4();
        let build_profile = if cfg!(debug_assertions) {
            "debug"
        } else {
            "release"
        };
        let runtime_span = info_span!(
            "deskulpt_runtime",
            session_id = %session_id,
            stage = "startup",
            build = build_profile,
        );
        let runtime_guard = RuntimeSpanGuard::new(runtime_span);

        let previous_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic_info| {
            tracing_panic::panic_hook(panic_info);
            previous_hook(panic_info);
        }));

        self.manage(LoggingState {
            _guard: guard,
            _runtime_span: runtime_guard,
        });
        Ok(())
    }
}

impl<R: Runtime> LoggingStateExt<R> for App<R> {}
impl<R: Runtime> LoggingStateExt<R> for AppHandle<R> {}

/// A simple appender that rotates the active log file whenever it grows beyond
/// the configured size and enforces retention guarantees.
struct SizeCappedAppender {
    dir: PathBuf,
    max_bytes: u64,
    current_size: u64,
    file: Option<File>,
}

impl SizeCappedAppender {
    fn new(dir: &Path, max_bytes: u64) -> io::Result<Self> {
        if !dir.exists() {
            fs::create_dir_all(dir)?;
        }
        let active_path = dir.join(ACTIVE_LOG_FILENAME);
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&active_path)?;
        let current_size = file.metadata().map(|m| m.len()).unwrap_or(0);
        Ok(Self {
            dir: dir.to_path_buf(),
            max_bytes,
            current_size,
            file: Some(file),
        })
    }

    fn rotate(&mut self) -> io::Result<()> {
        if let Some(file) = self.file.as_mut() {
            file.flush()?;
        }
        // Close the current file handle before renaming on Windows.
        drop(self.file.take());

        let active_path = self.active_path();
        if active_path.exists() {
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            let rotated = self.dir.join(format!("deskulpt-{timestamp}.log"));
            fs::rename(&active_path, rotated)?;
        }

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&active_path)?;
        self.current_size = 0;
        self.file = Some(file);
        cleanup_logs_dir(&self.dir)?;
        Ok(())
    }

    fn active_path(&self) -> PathBuf {
        self.dir.join(ACTIVE_LOG_FILENAME)
    }

    fn file_mut(&mut self) -> io::Result<&mut File> {
        self.file
            .as_mut()
            .ok_or_else(|| io::Error::other("log file is not available"))
    }
}

impl Write for SizeCappedAppender {
    fn write(&mut self, mut buf: &[u8]) -> io::Result<usize> {
        let mut written_total = 0;
        while !buf.is_empty() {
            if self.current_size >= self.max_bytes {
                self.rotate()?;
            }

            let remaining = (self.max_bytes - self.current_size) as usize;
            if remaining == 0 {
                self.rotate()?;
                continue;
            }

            let chunk = remaining.min(buf.len());
            let written = self.file_mut()?.write(&buf[..chunk])?;
            self.current_size += written as u64;
            written_total += written;
            buf = &buf[written..];

            if written == 0 {
                break;
            }
        }

        Ok(written_total)
    }

    fn flush(&mut self) -> io::Result<()> {
        if let Some(file) = self.file.as_mut() {
            file.flush()
        } else {
            Ok(())
        }
    }
}

fn cleanup_logs_dir(dir: &Path) -> io::Result<()> {
    if !dir.exists() {
        return Ok(());
    }

    let now = SystemTime::now();
    let mut rotated = vec![];

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let Some(name) = path.file_name().and_then(|os| os.to_str()) else {
            continue;
        };

        if name == ACTIVE_LOG_FILENAME {
            continue;
        }
        if !name.starts_with("deskulpt-") || !name.ends_with(".log") {
            continue;
        }

        let metadata = entry.metadata()?;
        let modified = metadata.modified().unwrap_or(UNIX_EPOCH);
        if now.duration_since(modified).unwrap_or_default() > MAX_LOG_FILE_AGE {
            fs::remove_file(&path)?;
            continue;
        }

        rotated.push((modified, path));
    }

    rotated.sort_by(|a, b| b.0.cmp(&a.0));
    let max_rotated = MAX_LOG_FILES.saturating_sub(1);
    if rotated.len() > max_rotated {
        for (_, path) in rotated.into_iter().skip(max_rotated) {
            fs::remove_file(path)?;
        }
    }

    Ok(())
}

/// Keeps a span entered for the duration of the logging subsystem.
struct RuntimeSpanGuard {
    span_ptr: *mut Span,
    entered: ManuallyDrop<Entered<'static>>,
}

unsafe impl Send for RuntimeSpanGuard {}
unsafe impl Sync for RuntimeSpanGuard {}

impl RuntimeSpanGuard {
    fn new(span: Span) -> Self {
        let span_ptr = Box::into_raw(Box::new(span));
        // SAFETY: `span_ptr` points to a valid `Span` until this guard is dropped.
        let span_ref: &'static Span = unsafe { &*span_ptr };
        let entered = span_ref.enter();
        Self {
            span_ptr,
            entered: ManuallyDrop::new(entered),
        }
    }
}

impl Drop for RuntimeSpanGuard {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.entered);
            drop(Box::from_raw(self.span_ptr));
        }
    }
}
