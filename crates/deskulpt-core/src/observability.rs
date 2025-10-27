//! Observability integration for Deskulpt.

use anyhow::Result;
use tauri::{AppHandle, Runtime};

use crate::path::PathExt;

/// Extension trait for initializing observability.
pub trait ObservabilityExt<R: Runtime>: PathExt<R> {
    /// Initialize observability with crash reporting and logging.
    fn init_observability(&self, enable_telemetry: bool) -> Result<()> {
        let persist_dir = self.persist_dir()?;
        let log_dir = persist_dir.join("logs");

        let config = deskulpt_observability::ObservabilityConfig {
            log_dir,
            enable_telemetry,
            sentry_dsn: option_env!("SENTRY_DSN").map(|s| s.to_string()),
            release: env!("CARGO_PKG_VERSION").to_string(),
            environment: if cfg!(debug_assertions) {
                "development".to_string()
            } else {
                "production".to_string()
            },
        };

        deskulpt_observability::init(config)?;

        tracing::info!(
            enable_telemetry,
            "Observability initialized with telemetry enabled: {}",
            enable_telemetry
        );

        Ok(())
    }
}

impl<R: Runtime> ObservabilityExt<R> for AppHandle<R> {}
