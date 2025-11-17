use std::collections::VecDeque;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::time::{SystemTime, UNIX_EPOCH};

use deskulpt_common::{SerResult, ser_bail};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::{AppHandle, Runtime, command};
use tracing::{Level, error, instrument, warn};

use crate::path::PathExt;

/// Helper to get the logs directory with consistent error handling.
fn get_logs_dir<R: Runtime>(app_handle: &AppHandle<R>) -> SerResult<std::path::PathBuf> {
    app_handle.logs_dir().map(|p| p.to_path_buf()).map_err(|e| {
        error!(error = ?e, "Failed to resolve logs directory");
        e.into()
    })
}

/// Metadata describing a log file on disk.
#[derive(Debug, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct LogFileInfo {
    pub name: String,
    pub size: u32,
    pub modified: String,
}

/// A single parsed log entry.
#[derive(Debug, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct LogEntry {
    pub timestamp: String,
    pub level: String,
    pub message: String,
    pub fields: Option<String>,
}

/// Log levels accepted from the frontend.
#[derive(Debug, Deserialize, specta::Type)]
#[serde(rename_all = "lowercase")]
pub enum FrontendLogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl From<FrontendLogLevel> for Level {
    fn from(level: FrontendLogLevel) -> Self {
        match level {
            FrontendLogLevel::Trace => Level::TRACE,
            FrontendLogLevel::Debug => Level::DEBUG,
            FrontendLogLevel::Info => Level::INFO,
            FrontendLogLevel::Warn => Level::WARN,
            FrontendLogLevel::Error => Level::ERROR,
        }
    }
}

#[command]
#[specta::specta]
#[instrument(skip(app_handle))]
pub fn log<R: Runtime>(
    app_handle: AppHandle<R>,
    level: FrontendLogLevel,
    message: String,
    fields: Option<Value>,
) -> SerResult<()> {
    let _ = app_handle;
    let level: Level = level.into();
    let fields = fields.unwrap_or(Value::Null);
    log_frontend_event(level, &message, &fields);
    Ok(())
}

fn log_frontend_event(level: Level, message: &str, fields: &Value) {
    macro_rules! emit {
        ($macro:ident) => {
            tracing::$macro!(
                target: "deskulpt::frontend",
                frontend_message = %message,
                frontend_fields = tracing::field::debug(fields),
            )
        };
    }

    match level {
        Level::TRACE => emit!(trace),
        Level::DEBUG => emit!(debug),
        Level::INFO => emit!(info),
        Level::WARN => emit!(warn),
        Level::ERROR => emit!(error),
    }
}

#[command]
#[specta::specta]
#[instrument(skip(app_handle))]
pub fn list_logs<R: Runtime>(app_handle: AppHandle<R>) -> SerResult<Vec<LogFileInfo>> {
    let logs_dir = get_logs_dir(&app_handle)?;
    let mut files = vec![];

    let entries = match fs::read_dir(&logs_dir) {
        Ok(entries) => entries,
        Err(e) => {
            error!(error = ?e, directory = %logs_dir.display(), "Failed to read logs directory");
            return Err(e.into());
        },
    };

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                error!(error = ?e, "Failed to read directory entry");
                continue;
            },
        };
        let metadata = match entry.metadata() {
            Ok(metadata) => metadata,
            Err(e) => {
                error!(error = ?e, path = %entry.path().display(), "Failed to read entry metadata");
                continue;
            },
        };
        if !metadata.is_file() {
            continue;
        }
        let modified = metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
        files.push((
            modified,
            LogFileInfo {
                name: entry.file_name().to_string_lossy().into_owned(),
                size: metadata.len().try_into().unwrap_or(u32::MAX),
                modified: format_system_time(modified),
            },
        ));
    }

    files.sort_by(|a, b| b.0.cmp(&a.0));
    Ok(files.into_iter().map(|(_, info)| info).collect())
}

#[command]
#[specta::specta]
#[instrument(skip(app_handle))]
pub fn read_log<R: Runtime>(
    app_handle: AppHandle<R>,
    filename: String,
    limit: u32,
) -> SerResult<Vec<LogEntry>> {
    ensure_single_component(&filename)?;

    let limit = limit.max(1) as usize;
    let logs_dir = get_logs_dir(&app_handle)?;
    let path = logs_dir.join(&filename);
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => {
            error!(error = ?e, path = %path.display(), "Failed to open log file");
            return Err(e.into());
        },
    };
    let reader = BufReader::new(file);

    let mut buffer = VecDeque::with_capacity(limit);
    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(e) => {
                error!(error = ?e, path = %path.display(), "Failed to read line from log file");
                continue;
            },
        };
        if buffer.len() >= limit {
            buffer.pop_front();
        }
        buffer.push_back(line);
    }

    Ok(buffer
        .into_iter()
        .filter_map(|line| parse_entry(&line))
        .collect())
}

#[command]
#[specta::specta]
#[instrument(skip(app_handle))]
pub fn clear_logs<R: Runtime>(app_handle: AppHandle<R>) -> SerResult<()> {
    let logs_dir = get_logs_dir(&app_handle)?;
    let entries = match fs::read_dir(&logs_dir) {
        Ok(entries) => entries,
        Err(e) => {
            error!(error = ?e, directory = %logs_dir.display(), "Failed to read logs directory");
            return Err(e.into());
        },
    };
    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                error!(error = ?e, "Failed to read directory entry");
                continue;
            },
        };
        let metadata = match entry.metadata() {
            Ok(metadata) => metadata,
            Err(e) => {
                error!(error = ?e, path = %entry.path().display(), "Failed to read entry metadata");
                continue;
            },
        };
        if metadata.is_file() {
            if let Err(e) = fs::remove_file(entry.path()) {
                error!(error = ?e, path = %entry.path().display(), "Failed to remove log file");
            }
        }
    }
    Ok(())
}

fn parse_entry(line: &str) -> Option<LogEntry> {
    let value: serde_json::Value = match serde_json::from_str(line) {
        Ok(value) => value,
        Err(e) => {
            warn!(error = ?e, "Failed to parse log entry line");
            return None;
        },
    };

    let get_str = |key: &str| {
        value
            .get(key)
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string()
    };

    let timestamp = get_str("timestamp");
    let level = get_str("level");
    let message = get_str("message");

    let fields = value.as_object().and_then(|object| {
        let mut rest = serde_json::Map::new();
        for (key, val) in object {
            if matches!(key.as_str(), "timestamp" | "level" | "message") {
                continue;
            }
            rest.insert(key.clone(), val.clone());
        }
        if rest.is_empty() {
            None
        } else {
            serde_json::to_string(&rest).ok()
        }
    });

    Some(LogEntry {
        timestamp,
        level,
        message,
        fields,
    })
}

fn ensure_single_component(filename: &str) -> SerResult<()> {
    if filename.is_empty() || filename.contains(['/', '\\']) {
        ser_bail!("Invalid log file name");
    }
    Ok(())
}
#[allow(dead_code)]
fn format_system_time(time: SystemTime) -> String {
    match time.duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_secs().to_string(),
        Err(_) => "0".into(),
    }
}
