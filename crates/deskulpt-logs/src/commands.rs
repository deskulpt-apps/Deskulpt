use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use anyhow::Result;
use deskulpt_common::SerResult;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Runtime, WebviewWindow};
use tracing::{debug, error, info, trace, warn};

use crate::LogsExt;

/// Size of each log block to read.
///
/// This is set to 16 KiB to balance between performance and memory usage.
/// Larger blocks reduce the number of read operations but increase memory
/// consumption.
const BLOCK_SIZE: u64 = 1 << 14;

/// Logging levels supported.
///
/// They correspond to the [`tracing::Level`] variants.
#[derive(Debug, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum Level {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl From<Level> for tracing::Level {
    fn from(level: Level) -> Self {
        match level {
            Level::Trace => tracing::Level::TRACE,
            Level::Debug => tracing::Level::DEBUG,
            Level::Info => tracing::Level::INFO,
            Level::Warn => tracing::Level::WARN,
            Level::Error => tracing::Level::ERROR,
        }
    }
}

/// A page of log entries.
#[derive(Debug, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    /// Log entries in reverse chronological order, i.e., newest first.
    pub entries: Vec<Entry>,
    /// Cursor for fetching the next page of older log entries.
    pub cursor: Option<Cursor>,
    /// Whether there are more log entries available beyond this page.
    pub has_more: bool,
}

/// Cursor for log pagination.
#[derive(Debug, Deserialize, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Cursor {
    /// The rotating log file path.
    pub path: PathBuf,
    /// The byte offset within the log file.
    ///
    /// If offset is non-zero, there is older data in this file in [0, offset).
    /// If offset is zero, this file is fully consumed and the next older
    /// non-empty file should be read.
    pub offset: u64,
}

/// A single log entry.
#[derive(Debug, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    /// Timestamp of the log entry in RFC 3339 format.
    pub timestamp: String,
    /// The logging level, all capitals.
    pub level: String,
    /// The log message.
    pub message: String,
    /// The raw JSON representation of the log entry.
    pub raw: serde_json::Value,
}

impl Entry {
    /// Parse a log entry from bytes.
    ///
    /// If the bytes cannot be parsed as valid JSON or required log fields are
    /// missing, this returns `None`.
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let raw: serde_json::Value = serde_json::from_slice(bytes).ok()?;

        let timestamp = raw.get("timestamp")?.as_str()?.to_string();
        let level = raw.get("level")?.as_str()?.to_string();
        let message = raw.get("message")?.as_str()?.to_string();

        Some(Self {
            timestamp,
            level,
            message,
            raw,
        })
    }

    /// Check if the log entry meets the minimum logging level.
    fn meets_min_level(&self, min_level: &Level) -> bool {
        let min_level = match min_level {
            Level::Trace => tracing::Level::TRACE,
            Level::Debug => tracing::Level::DEBUG,
            Level::Info => tracing::Level::INFO,
            Level::Warn => tracing::Level::WARN,
            Level::Error => tracing::Level::ERROR,
        };

        match tracing::Level::from_str(&self.level) {
            // Tracing levels are ordered by verbosity but we order by severity
            Ok(self_level) => self_level <= min_level,
            Err(_) => false,
        }
    }
}

/// Log a message at the specified level from the frontend.
///
/// Optional metadata can be provided as a JSON value. If no metadata is needed,
/// pass null.
#[tauri::command]
#[specta::specta]
pub async fn log<R: Runtime>(
    window: WebviewWindow<R>,
    level: Level,
    message: String,
    meta: serde_json::Value,
) -> SerResult<()> {
    match window.label() {
        "canvas" => match level {
            Level::Trace => trace!(target: "frontend::canvas", %meta, message),
            Level::Debug => debug!(target: "frontend::canvas", %meta, message),
            Level::Info => info!(target: "frontend::canvas", %meta, message),
            Level::Warn => warn!(target: "frontend::canvas", %meta, message),
            Level::Error => error!(target: "frontend::canvas", %meta, message),
        },
        "manager" => match level {
            Level::Trace => trace!(target: "frontend::manager", %meta, message),
            Level::Debug => debug!(target: "frontend::manager", %meta, message),
            Level::Info => info!(target: "frontend::manager", %meta, message),
            Level::Warn => warn!(target: "frontend::manager", %meta, message),
            Level::Error => error!(target: "frontend::manager", %meta, message),
        },
        _ => {},
    }
    Ok(())
}

/// Find the next non-empty log file starting from the given index.
///
/// If found, this returns `Ok(Some(...))` with the index in the given files
/// list and the length of the found file. If no non-empty file is found, this
/// returns `Ok(None)`. If any error occurs during file metadata retrieval, it
/// an error.
fn find_next_nonempty_file(files: &[PathBuf], start_idx: usize) -> Result<Option<(usize, u64)>> {
    let mut idx = start_idx;
    while idx < files.len() {
        let len = files[idx].metadata()?.len();
        if len > 0 {
            return Ok(Some((idx, len)));
        }
        idx += 1;
    }
    Ok(None)
}

/// Figure out where to start reading logs.
///
/// If no cursor is provided, this finds the newest non-empty log file. If a
/// cursor is provided, it locates the specified file and offset. If the offset
/// is zero, it finds the next non-empty file after that one.
///
/// If no suitable starting position is found, this returns `Ok(None)`, meaning
/// there are no logs to read. Otherwise, it returns `Ok(Some(...))` with the
/// file index and offset within that file. If any error occurs during the
/// process, it returns an error.
fn resolve_initial_position(
    files: &[PathBuf],
    cursor: &Option<Cursor>,
) -> Result<Option<(usize, u64)>> {
    match cursor {
        None => find_next_nonempty_file(files, 0),
        Some(c) => {
            let mut found_idx = None;
            for (i, p) in files.iter().enumerate() {
                if p == &c.path {
                    found_idx = Some(i);
                    break;
                }
            }

            let idx = found_idx.unwrap_or(0);
            if c.offset > 0 {
                Ok(Some((idx, c.offset)))
            } else {
                find_next_nonempty_file(files, idx + 1)
            }
        },
    }
}

/// Scan a log file backwards for matching log entries.
///
/// This reads the file in blocks from the given end offset towards the start,
/// extracting log entries that meet the minimum logging level until either
/// the start of the file is reached or the limit of remaining entries is met.
/// The buffer `buf` is used for reading file data and should be at least
/// [`BLOCK_SIZE`] bytes in length.
///
/// This function returns a vector of matching [`Entry`]s and an optional
/// byte offset indicating where to continue scanning in the next call. If the
/// entire file has been scanned, the offset is `None`.
fn scan_file_for_matches(
    path: &Path,
    mut end_offset: u64,
    limit_remaining: usize,
    min_level: &Level,
    buf: &mut [u8],
) -> Result<(Vec<Entry>, Option<u64>)> {
    let mut file = File::open(path)?;
    let mut matches = vec![];

    // Buffer to accumulate bytes for the current line, but because we read
    // backwards the bytes would be in reverse order
    let mut current_line_rev = vec![];

    while end_offset > 0 && matches.len() < limit_remaining {
        let block_start = end_offset.saturating_sub(BLOCK_SIZE);
        let block_len = (end_offset - block_start) as usize;

        file.seek(SeekFrom::Start(block_start))?;
        file.read_exact(&mut buf[..block_len])?;

        for i in (0..block_len).rev() {
            let byte = buf[i];
            let abs_pos = block_start + i as u64;

            if byte == b'\n' {
                if !current_line_rev.is_empty() {
                    // Reverse the accumulated line bytes to the correct order;
                    // we don't need to care about CR because trailing CR is
                    // still acceptable JSON
                    current_line_rev.reverse();
                    let line_bytes = std::mem::take(&mut current_line_rev);

                    if let Some(entry) = Entry::from_bytes(&line_bytes)
                        && entry.meets_min_level(min_level)
                    {
                        matches.push(entry);
                        if matches.len() >= limit_remaining {
                            // `abs_pos` is the position of the newline before
                            // this processed line, so the next read should
                            // start (backwards) from there
                            return Ok((matches, Some(abs_pos)));
                        }
                    }
                }
            } else {
                current_line_rev.push(byte);
            }

            if abs_pos == 0 {
                break; // Reached the start of the file
            }
        }

        end_offset = block_start;
    }

    // When we reach offset 0 and break out of the loop, the very first line of
    // the file may still be pending processing because it typically won't have
    // a preceding newline character
    if end_offset == 0 && !current_line_rev.is_empty() && matches.len() < limit_remaining {
        current_line_rev.reverse();
        let line_bytes = std::mem::take(&mut current_line_rev);

        if let Some(entry) = Entry::from_bytes(&line_bytes)
            && entry.meets_min_level(min_level)
        {
            matches.push(entry);
        }
    }

    Ok((matches, None)) // Entire file scanned without exceeding limit
}

/// Fetch a page of log entries.
///
/// The limit specifies the maximum number of log entries to retrieve and must
/// be strictly positive. The cursor is for pagination. The first call should
/// pass `None` for the cursor to start from the newest log entries, and
/// subsequent calls should use the cursor returned from the previous call to
/// fetch older entries. `min_level` filters log entries to only those at or
/// above the specified logging level (ordered by severity).
///
/// ### Errors
///
/// - The limit is zero.
/// - Failed to retrieve metadata of log files.
/// - Failed to read log files.
#[tauri::command]
#[specta::specta]
pub async fn read<R: Runtime>(
    app_handle: AppHandle<R>,
    limit: usize,
    cursor: Option<Cursor>,
    min_level: Level,
) -> SerResult<Page> {
    assert!(limit > 0, "Limit must be strictly positive");

    let files = app_handle.logs().collect()?;
    if files.is_empty() {
        return Ok(Page {
            entries: Vec::new(),
            cursor: None,
            has_more: false,
        });
    }

    let mut entries = vec![];
    let mut scan_buf = vec![0u8; BLOCK_SIZE as usize]; // Reusable buffer
    let mut position = resolve_initial_position(&files, &cursor)?;

    while let Some((file_idx, end_offset)) = position {
        if entries.len() >= limit {
            break; // Reached the requested limit
        }

        let path = &files[file_idx];
        let file_len = path.metadata()?.len();

        // Sanity checks that we don't read past the end of file (if cursor is
        // somehow beyond file length), and we automatically move to the next
        // file if the cursor is already at 0
        let effective_end = end_offset.min(file_len);
        if effective_end == 0 {
            position = find_next_nonempty_file(&files, file_idx + 1)?;
            continue;
        }

        let (mut file_entries, cursor_in_file) = scan_file_for_matches(
            path,
            effective_end,
            limit - entries.len(),
            &min_level,
            &mut scan_buf,
        )?;

        entries.append(&mut file_entries);

        if let Some(next_offset) = cursor_in_file {
            // We have filled the quota while still within this file, so we
            // return a cursor pointing to where we left off
            let next_cursor = Cursor {
                path: path.clone(),
                offset: next_offset,
            };
            return Ok(Page {
                entries,
                cursor: Some(next_cursor),
                has_more: true,
            });
        }

        // Finished scanning this file without reaching quota, move to the next
        // and loop again
        position = find_next_nonempty_file(&files, file_idx + 1)?;
    }

    // We ran out of files or reached the limit, either case we declare no more
    // further entries
    Ok(Page {
        entries,
        cursor: None,
        has_more: false,
    })
}

/// Clear all log files and return the freed disk space in bytes.
///
/// ### Errors
///
/// - Error discovering log files.
#[tauri::command]
#[specta::specta]
pub async fn clear<R: Runtime>(app_handle: AppHandle<R>) -> SerResult<u64> {
    let cleared_size = app_handle.logs().clear()?;
    Ok(cleared_size)
}
