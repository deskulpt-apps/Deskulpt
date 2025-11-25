//! State management for logging.

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

        let backend_file_layer = fmt::layer()
            .json()
            .with_ansi(false)
            .with_file(true)
            .with_line_number(true)
            .with_timer(UtcTime::rfc_3339())
            .flatten_event(true)
            .with_writer(writer.clone())
            .with_filter(Targets::new().with_target("deskulpt", Level::TRACE));

        let frontend_file_layer = fmt::layer()
            .json()
            .with_ansi(false)
            .with_timer(UtcTime::rfc_3339())
            .flatten_event(true)
            .with_writer(writer)
            .with_filter(
                Targets::new()
                    .with_target("frontend::canvas", Level::TRACE)
                    .with_target("frontend::manager", Level::TRACE),
            );

        let subscriber = Registry::default()
            .with(frontend_file_layer)
            .with(backend_file_layer);
        tracing::subscriber::set_global_default(subscriber)?;

        let previous_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic_info| {
            tracing_panic::panic_hook(panic_info);
            previous_hook(panic_info);
        }));

        self.manage(LoggingState { _guard: guard });
        Ok(())
    }
}

impl<R: Runtime> LoggingStateExt<R> for App<R> {}
impl<R: Runtime> LoggingStateExt<R> for AppHandle<R> {}
