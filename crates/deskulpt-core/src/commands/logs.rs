use std::fs;
use std::path::PathBuf;

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Runtime};

use crate::path::PathExt;

/// Information about a log file.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct LogFileInfo {
    /// Name of the log file
    pub name: String,
    /// Size in bytes
    pub size: u64,
    /// Last modified timestamp (ISO 8601)
    pub modified: String,
}

/// Log entry parsed from a log file.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct LogEntry {
    /// Timestamp of the log entry
    pub timestamp: String,
    /// Log level (DEBUG, INFO, WARN, ERROR)
    pub level: String,
    /// Log message
    pub message: String,
    /// Additional fields (structured logging)
    pub fields: Option<String>,
}

/// List all available log files.
pub fn list_logs<R: Runtime>(app: &AppHandle<R>) -> Result<Vec<LogFileInfo>> {
    let log_dir = app.persist_dir()?.join("logs");

    if !log_dir.exists() {
        return Ok(vec![]);
    }

    let mut logs = Vec::new();

    for entry in fs::read_dir(&log_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().map(|e| e == "log").unwrap_or(false) {
            let metadata = fs::metadata(&path)?;
            let modified = metadata
                .modified()?
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs();
            let datetime = time::OffsetDateTime::from_unix_timestamp(modified as i64)?;

            logs.push(LogFileInfo {
                name: path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string(),
                size: metadata.len(),
                modified: datetime.to_string(),
            });
        }
    }

    // Sort by modification time (newest first)
    logs.sort_by(|a, b| b.modified.cmp(&a.modified));

    Ok(logs)
}

/// Read log file content with optional filtering.
pub fn read_log<R: Runtime>(
    app: &AppHandle<R>,
    filename: String,
    limit: Option<u32>,
) -> Result<Vec<LogEntry>> {
    let log_dir = app.persist_dir()?.join("logs");
    let log_path = log_dir.join(&filename);

    // Security: Prevent directory traversal
    if !log_path.starts_with(&log_dir) {
        return Err(anyhow!("Invalid log file path"));
    }

    if !log_path.exists() {
        return Err(anyhow!("Log file not found: {}", filename));
    }

    let content = fs::read_to_string(&log_path)?;
    let lines: Vec<&str> = content.lines().collect();

    let limit = limit.unwrap_or(1000).min(10000) as usize; // Cap at 10k entries
    let start = if lines.len() > limit {
        lines.len() - limit
    } else {
        0
    };

    let mut entries = Vec::new();

    for line in lines[start..].iter() {
        if let Ok(entry) = parse_log_line(line) {
            entries.push(entry);
        }
    }

    Ok(entries)
}

/// Parse a JSON log line into a LogEntry.
fn parse_log_line(line: &str) -> Result<LogEntry> {
    let json: serde_json::Value = serde_json::from_str(line)?;
    let obj = json.as_object().ok_or(anyhow!("Not an object"))?;

    let timestamp = obj
        .get("timestamp")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string();

    let level = obj
        .get("level")
        .and_then(|v| v.as_str())
        .unwrap_or("INFO")
        .to_string();

    let message = obj
        .get("message")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    // Collect other fields
    let mut fields_obj = serde_json::json!({});
    for (k, v) in obj.iter() {
        if !["timestamp", "level", "message", "target", "name"].contains(&k.as_str()) {
            fields_obj[k] = v.clone();
        }
    }

    let fields = if fields_obj
        .as_object()
        .map(|o| !o.is_empty())
        .unwrap_or(false)
    {
        Some(fields_obj.to_string())
    } else {
        None
    };

    Ok(LogEntry {
        timestamp,
        level,
        message,
        fields,
    })
}

/// Get log statistics.
pub fn get_log_stats<R: Runtime>(app: &AppHandle<R>) -> Result<LogStats> {
    let logs = list_logs(app)?;

    let total_size: u64 = logs.iter().map(|l| l.size).sum();
    let file_count = logs.len();

    Ok(LogStats {
        file_count,
        total_size,
    })
}

/// Log statistics.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct LogStats {
    /// Number of log files
    pub file_count: u32,
    /// Total size in bytes
    pub total_size: u64,
}

/// Clear all log files.
pub fn clear_logs<R: Runtime>(app: &AppHandle<R>) -> Result<()> {
    let log_dir = app.persist_dir()?.join("logs");

    if !log_dir.exists() {
        return Ok(());
    }

    for entry in fs::read_dir(&log_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().map(|e| e == "log").unwrap_or(false) {
            fs::remove_file(&path)?;
        }
    }

    tracing::info!("Log files cleared");

    Ok(())
}
