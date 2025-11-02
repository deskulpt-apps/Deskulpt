use std::panic::PanicHookInfo;
use std::path::Path;

use anyhow::{Context, Result};
use once_cell::sync::OnceCell;
use tauri::{App, Runtime};
use tracing_appender::non_blocking::{self, NonBlocking, WorkerGuard};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry, fmt};

use crate::path::PathExt;

const MAX_LOG_FILES: usize = 10;

static INIT: OnceCell<()> = OnceCell::new();
static LOG_GUARD: OnceCell<WorkerGuard> = OnceCell::new();
static PANIC_HOOK: OnceCell<()> = OnceCell::new();

/// Initialize tracing for the Deskulpt backend.
pub fn init<R: Runtime>(app: &mut App<R>) -> Result<()> {
    if INIT.get().is_some() {
        return Ok(());
    }

    app.init_logs_dir()?;
    let logs_dir = app.logs_dir()?;
    let (writer, guard) = build_writer(logs_dir)?;
    install_subscriber(writer)?;
    retain_guard(guard);
    install_panic_hook();

    INIT.set(()).ok();
    Ok(())
}

fn build_writer(logs_dir: &Path) -> Result<(NonBlocking, WorkerGuard)> {
    let appender = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .max_log_files(MAX_LOG_FILES)
        .filename_prefix("app.log")
        .build(logs_dir)
        .context("failed to initialise rolling file appender")?;

    let (writer, guard) = non_blocking::NonBlockingBuilder::default()
        .lossy(false)
        .finish(appender);

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

fn retain_guard(guard: WorkerGuard) {
    let _ = LOG_GUARD.set(guard);
}

fn install_panic_hook() {
    if PANIC_HOOK.get().is_some() {
        return;
    }

    let previous_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        log_panic(panic_info);
        previous_hook(panic_info);
    }));

    PANIC_HOOK.set(()).ok();
}

fn log_panic(panic_info: &PanicHookInfo<'_>) {
    let backtrace = std::backtrace::Backtrace::force_capture();
    let message = panic_message(panic_info);

    if let Some(location) = panic_info.location() {
        tracing::error!(
            target: "panic",
            backtrace = %backtrace,
            panic.file = location.file(),
            panic.line = location.line(),
            "Unhandled panic: {message}"
        );
    } else {
        tracing::error!(target: "panic", backtrace = %backtrace, "Unhandled panic: {message}");
    }
}

fn panic_message(panic_info: &PanicHookInfo<'_>) -> String {
    if let Some(message) = panic_info.payload().downcast_ref::<&str>() {
        (*message).to_owned()
    } else if let Some(message) = panic_info.payload().downcast_ref::<String>() {
        message.clone()
    } else {
        "panic payload is not a string".to_owned()
    }
}
