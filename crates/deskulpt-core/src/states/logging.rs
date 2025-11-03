//! State management for application logging.

use std::path::Path;

use anyhow::{Context, Result};
use tauri::{App, AppHandle, Manager, Runtime};
use tracing_appender::non_blocking::{self, NonBlocking, WorkerGuard};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_panic::panic_hook;
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry, fmt};

use crate::path::PathExt;

const MAX_LOG_FILES: usize = 10;

/// Managed state that keeps the tracing worker alive.
struct LoggingState {
    /// Guard responsible for ensuring the background logging worker stays
    /// alive.
    _guard: WorkerGuard,
}

/// Extension trait for operations related to logging.
pub trait LoggingStateExt<R: Runtime>: Manager<R> + PathExt<R> {
    /// Initialize tracing for the Deskulpt backend.
    fn manage_logging(&self) -> Result<()> {
        if self.try_state::<LoggingState>().is_some() {
            return Ok(());
        }

        let logs_dir = self.logs_dir()?;
        let (writer, guard) = build_writer(logs_dir)?;
        install_subscriber(writer)?;
        let previous_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic_info| {
            panic_hook(panic_info);
            previous_hook(panic_info);
        }));
        self.manage(LoggingState { _guard: guard });
        Ok(())
    }
}

impl<R: Runtime> LoggingStateExt<R> for App<R> {}
impl<R: Runtime> LoggingStateExt<R> for AppHandle<R> {}

fn build_writer(logs_dir: &Path) -> Result<(NonBlocking, WorkerGuard)> {
    let appender = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .max_log_files(MAX_LOG_FILES)
        .filename_prefix("deskulpt")
        .filename_suffix("log")
        .build(logs_dir)
        .context("failed to initialise rolling file appender")?;

    let (writer, guard) = non_blocking::NonBlockingBuilder::default().finish(appender);

    Ok((writer, guard))
}

fn install_subscriber(writer: NonBlocking) -> Result<()> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let fmt_layer = fmt::layer()
        .json()
        .with_ansi(false)
        .with_file(true)
        .with_line_number(true)
        .with_timer(UtcTime::rfc_3339())
        .flatten_event(true)
        .with_writer(writer);

    let subscriber = Registry::default().with(env_filter).with(fmt_layer);

    tracing::subscriber::set_global_default(subscriber)
        .context("failed to install tracing subscriber")
}
