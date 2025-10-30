# deskulpt-observability

Provides observability, logging, and crash reporting infrastructure for Deskulpt.

## Features

- **Structured Logging**: Uses `tracing` for structured, contextual logging
- **Log Rotation**: Automatic log file rotation by size and time
- **Crash Reporting**: Integrates with Sentry for error tracking
- **Panic Handling**: Captures and reports panics
- **Configurable Levels**: DEBUG for development, WARN+ for production
- **Privacy-First**: Excludes PII by default
- **Cross-Platform**: Works on Windows, macOS, and Linux

## Usage

Initialize observability early in your application:

```rust
let config = ObservabilityConfig {
    log_dir: log_directory,
    enable_telemetry: true,
    sentry_dsn: Some("your-sentry-dsn"),
    release: env!("CARGO_PKG_VERSION"),
    environment: if cfg!(debug_assertions) { "development" } else { "production" },
};

observability::init(config)?;
```

Then use standard `tracing` macros:

```rust
tracing::info!(widget_id = "my_widget", "Widget loaded");
tracing::error!(error = %err, context = "widget_update", "Failed to update widget");
```
