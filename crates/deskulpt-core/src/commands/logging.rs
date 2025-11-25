use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use deskulpt_common::{SerResult, ser_bail};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Runtime, WebviewWindow, command};
use tracing::{debug, error, info, instrument, trace, warn};

use crate::path::PathExt;

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
pub enum LoggingLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

#[command]
#[specta::specta]
pub async fn log<R: Runtime>(
    window: WebviewWindow<R>,
    level: LoggingLevel,
    message: String,
    meta: serde_json::Value,
) -> SerResult<()> {
    match window.label() {
        "canvas" => match level {
            LoggingLevel::Trace => trace!(target: "frontend::canvas", %meta, message),
            LoggingLevel::Debug => debug!(target: "frontend::canvas", %meta, message),
            LoggingLevel::Info => info!(target: "frontend::canvas", %meta, message),
            LoggingLevel::Warn => warn!(target: "frontend::canvas", %meta, message),
            LoggingLevel::Error => error!(target: "frontend::canvas", %meta, message),
        },
        "manager" => match level {
            LoggingLevel::Trace => trace!(target: "frontend::manager", %meta, message),
            LoggingLevel::Debug => debug!(target: "frontend::manager", %meta, message),
            LoggingLevel::Info => info!(target: "frontend::manager", %meta, message),
            LoggingLevel::Warn => warn!(target: "frontend::manager", %meta, message),
            LoggingLevel::Error => error!(target: "frontend::manager", %meta, message),
        },
        _ => {},
    }
    Ok(())
}

#[command]
#[specta::specta]
#[instrument(skip(app_handle))]
pub async fn list_logs<R: Runtime>(app_handle: AppHandle<R>) -> SerResult<Vec<LogFileInfo>> {
    let mut files: Vec<_> = collect_log_files(&app_handle)?
        .into_iter()
        .filter_map(|(path, metadata)| {
            let modified = metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
            let name = path.file_name()?.to_string_lossy().into_owned();
            Some((
                modified,
                LogFileInfo {
                    name,
                    size: metadata.len().try_into().unwrap_or(u32::MAX),
                    modified: format_system_time(modified),
                },
            ))
        })
        .collect();

    files.sort_by(|a, b| b.0.cmp(&a.0));
    Ok(files.into_iter().map(|(_, info)| info).collect())
}

#[command]
#[specta::specta]
#[instrument(skip(app_handle))]
pub async fn read_log<R: Runtime>(
    app_handle: AppHandle<R>,
    filename: String,
    limit: u32,
) -> SerResult<Vec<LogEntry>> {
    ensure_single_component(&filename)?;

    let limit = limit.max(1) as usize;
    let logs_dir = app_handle.logs_dir()?;
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
pub async fn clear_logs<R: Runtime>(app_handle: AppHandle<R>) -> SerResult<()> {
    for (path, _) in collect_log_files(&app_handle)? {
        if let Err(e) = std::fs::remove_file(&path) {
            error!(error = ?e, path = %path.display(), "Failed to remove log file");
        }
    }
    Ok(())
}

#[command]
#[specta::specta]
#[instrument(skip(app_handle))]
pub async fn open_logs_dir<R: Runtime>(app_handle: AppHandle<R>) -> SerResult<()> {
    let logs_dir = app_handle.logs_dir()?;
    open::that_detached(logs_dir)?;
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

fn collect_log_files<R: Runtime>(
    app_handle: &AppHandle<R>,
) -> SerResult<Vec<(PathBuf, std::fs::Metadata)>> {
    let logs_dir = app_handle.logs_dir()?;
    let entries = match std::fs::read_dir(&logs_dir) {
        Ok(entries) => entries,
        Err(e) => {
            error!(error = ?e, directory = %logs_dir.display(), "Failed to read logs directory");
            return Err(e.into());
        },
    };

    let mut files = Vec::new();
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
            files.push((entry.path(), metadata));
        }
    }

    Ok(files)
}

fn format_system_time(time: SystemTime) -> String {
    match time.duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_secs().to_string(),
        Err(_) => "0".into(),
    }
}
