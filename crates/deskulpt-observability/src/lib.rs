//! Observability, logging, and crash reporting for Deskulpt.
//!
//! This crate provides:
//! - Structured logging with `tracing`
//! - Log file rotation
//! - Panic capture and reporting
//! - Sentry integration for error tracking

use std::panic;
use std::path::PathBuf;
use std::sync::Once;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

pub mod log_file;

static INIT: Once = Once::new();

/// Configuration for observability.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityConfig {
    /// Directory to store log files.
    pub log_dir: PathBuf,
    /// Enable telemetry and crash reporting.
    pub enable_telemetry: bool,
    /// Sentry DSN for crash reporting.
    pub sentry_dsn: Option<String>,
    /// Application release version.
    pub release: String,
    /// Environment (e.g., "development", "production").
    pub environment: String,
}

impl ObservabilityConfig {
    /// Check if we're in development mode.
    pub fn is_dev(&self) -> bool {
        self.environment == "development"
    }

    /// Get the log level based on environment.
    pub fn log_level(&self) -> &'static str {
        if self.is_dev() {
            "deskulpt=debug,info"
        } else {
            "deskulpt=warn,error"
        }
    }
}

/// Initialize observability for the application.
///
/// This sets up:
/// - Structured logging to console and rotating files
/// - Panic handler for crash reporting
/// - Sentry integration (if DSN is provided)
///
/// This function is safe to call multiple times; it will only initialize once.
pub fn init(config: ObservabilityConfig) -> Result<()> {
    let mut error: Option<anyhow::Error> = None;

    INIT.call_once(|| {
        if let Err(e) = init_inner(config) {
            error = Some(e);
        }
    });

    if let Some(e) = error {
        return Err(e);
    }

    Ok(())
}

fn init_inner(config: ObservabilityConfig) -> Result<()> {
    let is_dev = config.is_dev();

    // Set up panic hook for crash reporting
    let config_clone = config.clone();
    let default_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        let msg = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            *s
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            s.as_str()
        } else {
            "Unknown panic"
        };

        let location = panic_info
            .location()
            .map(|l| format!("{}:{}", l.file(), l.line()))
            .unwrap_or_default();

        tracing::error!(
            panic = true,
            message = msg,
            location = %location,
            "Application panicked"
        );

        // Send to Sentry if enabled
        if config_clone.enable_telemetry {
            sentry::capture_message(
                &format!("Panic: {} at {}", msg, location),
                sentry::Level::Fatal,
            );
        }

        // Call the default hook to display the panic message
        default_hook(panic_info);
    }));

    // Create logs directory
    std::fs::create_dir_all(&config.log_dir)?;

    // Initialize log rotation manager in background
    let log_dir = config.log_dir.clone();
    std::thread::spawn(move || {
        let _ = log_file::start_rotation_monitor(log_dir);
    });

    // Set up tracing layers
    let env_filter =
        EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new(config.log_level()))?;

    let console_layer = tracing_subscriber::fmt::layer()
        .with_target(true)
        .with_thread_ids(is_dev)
        .with_level(true);

    // Use tracing_appender for file logging with rotation
    let file_appender = tracing_appender::rolling::daily(&config.log_dir, "deskulpt.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let file_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_writer(non_blocking)
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true);

    // Initialize Sentry if enabled and DSN is provided
    let _sentry_guard = if config.enable_telemetry {
        if let Some(dsn) = &config.sentry_dsn {
            let release = config.release.clone();
            let environment = config.environment.clone();
            let guard = sentry::init((
                dsn.as_str(),
                sentry::ClientOptions {
                    release: Some(release.into()),
                    environment: Some(environment.into()),
                    traces_sample_rate: if is_dev { 1.0 } else { 0.1 },
                    attach_stacktrace: true,
                    ..Default::default()
                },
            ));
            tracing::info!("Sentry crash reporting initialized");
            Some(guard)
        } else {
            None
        }
    } else {
        None
    };

    // Combine layers
    tracing_subscriber::registry()
        .with(env_filter)
        .with(console_layer)
        .with(file_layer)
        .init();

    tracing::info!(
        version = %config.release,
        environment = %config.environment,
        telemetry_enabled = config.enable_telemetry,
        "Deskulpt observability initialized"
    );

    Ok(())
}
