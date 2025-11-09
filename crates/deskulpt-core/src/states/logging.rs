//! State management for logging.

use std::mem::ManuallyDrop;

use anyhow::Result;
use tauri::{App, AppHandle, Manager, Runtime};
use tracing::span::Entered;
use tracing::{Level, Span, info_span};
use tracing_appender::non_blocking::{NonBlockingBuilder, WorkerGuard};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::filter::{LevelFilter, Targets};
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::layer::{Layer, SubscriberExt};
use tracing_subscriber::{Registry, fmt};
use uuid::Uuid;

use crate::logging::SpanContextLayer;
use crate::path::PathExt;

/// Maximum number of log files (including the active log) to retain.
const MAX_LOG_FILES: usize = 10;
/// Log filename prefix used by the rolling appender.
const LOG_FILE_PREFIX: &str = "deskulpt";

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
        let appender = RollingFileAppender::builder()
            .rotation(Rotation::DAILY)
            .filename_prefix(LOG_FILE_PREFIX)
            .filename_suffix("log")
            .max_log_files(MAX_LOG_FILES)
            .build(logs_dir)
            .map_err(|e| anyhow::anyhow!("failed to initialize rolling log file: {e}"))?;
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
