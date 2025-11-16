use std::collections::VecDeque;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::time::{SystemTime, UNIX_EPOCH};

use deskulpt_common::{SerResult, ser_bail};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::{AppHandle, Runtime, command};
use tracing::Level;

use crate::path::PathExt;

/// Metadata describing a log file on disk.
#[derive(Debug, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct LogFileInfo {
    pub name: String,
    pub size: u64,
    pub modified: String,
}

/// A single parsed log entry.
#[derive(Debug, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
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
pub fn log(level: FrontendLogLevel, message: String, fields: Option<Value>) -> SerResult<()> {
    let level: Level = level.into();
    let fields = fields.unwrap_or(Value::Null);
    tracing::event!(
        target: "deskulpt::frontend",
        level,
        frontend_message = %message,
        frontend_fields = tracing::field::debug(&fields),
    );
    Ok(())
}

#[allow(dead_code)]
#[command]
#[specta::specta]
pub fn list_logs<R: Runtime>(app_handle: AppHandle<R>) -> SerResult<Vec<LogFileInfo>> {
    let logs_dir = app_handle.logs_dir()?;
    let mut files = vec![];

    for entry in fs::read_dir(logs_dir)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        if !metadata.is_file() {
            continue;
        }
        let modified = metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
        files.push((
            modified,
            LogFileInfo {
                name: entry.file_name().to_string_lossy().into_owned(),
                size: metadata.len(),
                modified: format_system_time(modified),
            },
        ));
    }

    files.sort_by(|a, b| b.0.cmp(&a.0));
    Ok(files.into_iter().map(|(_, info)| info).collect())
}

#[allow(dead_code)]
#[command]
#[specta::specta]
pub fn read_log<R: Runtime>(
    app_handle: AppHandle<R>,
    filename: String,
    limit: usize,
) -> SerResult<Vec<LogEntry>> {
    ensure_single_component(&filename)?;

    let limit = limit.max(1);
    let path = app_handle.logs_dir()?.join(&filename);
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut buffer = VecDeque::with_capacity(limit);
    for line in reader.lines() {
        let line = line?;
        if buffer.len() == limit {
            buffer.pop_front();
        }
        buffer.push_back(line);
    }

    Ok(buffer
        .into_iter()
        .filter_map(|line| parse_entry(&line))
        .collect())
}

#[allow(dead_code)]
#[command]
#[specta::specta]
pub fn clear_logs<R: Runtime>(app_handle: AppHandle<R>) -> SerResult<()> {
    let logs_dir = app_handle.logs_dir()?;
    for entry in fs::read_dir(logs_dir)? {
        let entry = entry?;
        if entry.metadata()?.is_file() {
            fs::remove_file(entry.path())?;
        }
    }
    Ok(())
}

#[allow(dead_code)]
fn parse_entry(line: &str) -> Option<LogEntry> {
    let value: serde_json::Value = serde_json::from_str(line).ok()?;
    let timestamp = value
        .get("timestamp")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let level = value
        .get("level")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let message = value
        .get("message")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();

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

#[allow(dead_code)]
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
