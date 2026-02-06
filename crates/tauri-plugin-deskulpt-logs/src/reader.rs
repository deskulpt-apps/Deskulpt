//! Logs reading, filtering, and pagination.

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::Level;

/// A page of log entries.
#[derive(Debug, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    /// Log entries in reverse chronological order (most recent first).
    pub entries: Vec<Entry>,
    /// Cursor for reading the next page of older log entries.
    ///
    /// If `None`, there are no more entries to read beyond this page.
    pub cursor: Option<Cursor>,
}

/// Cursor for log pagination.
#[derive(Debug, Deserialize, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Cursor {
    /// The index of the log file in the files list.
    pub file_idx: usize,
    /// The byte offset within the log file.
    ///
    /// When continuing from this cursor, reading resumes backwards from this
    /// offset. An offset of zero means this file has been fully read, and the
    /// reader should move to the next older file.
    pub offset: u64,
}

/// A single log entry.
#[derive(Debug, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    /// Timestamp of the log entry in RFC 3339 format.
    pub timestamp: String,
    /// The stringified logging level (e.g., "INFO", "ERROR").
    pub level: String,
    /// The log message text.
    pub message: String,
    /// The complete raw JSON object representing the log entry.
    pub raw: serde_json::Value,
}

/// Tail reader for rolling log files.
///
/// This reader processes log files in reverse order they are provided. Within
/// each file, it reads backwards from the end towards the beginning.
pub struct RollingTailReader {
    /// List of log files.
    ///
    /// The log files should be in reverse chronological order (most recent
    /// first). The log entries within each file should be in chronological
    /// order (most recent last).
    files: Vec<PathBuf>,
    /// The minimum logging level to filter entries.
    ///
    /// Entries with severity lower than this level are skipped when reading.
    min_level: Level,
    /// Reusable buffer for reading file blocks.
    ///
    /// This is to avoid repeated allocations when reading multiple blocks. The
    /// size is at least [`Self::BLOCK_SIZE`].
    buf: Vec<u8>,
}

impl RollingTailReader {
    /// Size of each block to read.
    ///
    /// This is set to 16KB to balance between performance and memory usage.
    /// Larger blocks reduce the number of read operations but increase memory
    /// consumption.
    const BLOCK_SIZE: u64 = 1 << 14;

    /// Create a new [`RollingTailReader`] instance.
    pub fn new(files: Vec<PathBuf>, min_level: Level) -> Self {
        Self {
            files,
            min_level,
            buf: vec![0u8; Self::BLOCK_SIZE as usize],
        }
    }

    /// Read a page of log entries.
    ///
    /// This returns up to `limit` log entries at or above the configured
    /// minimum severity level of the reader. Entries are returned in reverse
    /// chronological order (most recent first). If `cursor` is `None`, reading
    /// starts from the last log entry in the last log file and proceeds
    /// backwards. Otherwise, reading resumes from the specified cursor, which
    /// should have been obtained from a previous call to this method.
    pub fn read(&mut self, limit: usize, cursor: Option<Cursor>) -> Result<Page> {
        assert!(limit > 0, "Limit must be strictly positive");

        if self.files.is_empty() {
            return Ok(Page {
                entries: Vec::new(),
                cursor: None,
            });
        }

        let mut entries = vec![];
        let mut position = self.start_position(&cursor);

        while let Some((file_idx, end_offset)) = position {
            if entries.len() >= limit {
                break; // Reached the requested limit
            }

            // Sanity checks: don't read past EOF (if cursor is invalid), and
            // automatically move to the next file if offset is zero
            let file_len = self.files[file_idx].metadata()?.len();
            let effective_end = end_offset.min(file_len);
            if effective_end == 0 {
                position = self.next_file_position(file_idx + 1);
                continue;
            }

            let (mut file_entries, cursor_in_file) =
                self.read_file(file_idx, effective_end, limit - entries.len())?;

            entries.append(&mut file_entries);

            if let Some(next_offset) = cursor_in_file {
                // We have filled the quota while still within this file, so we
                // return a cursor pointing to where we left off
                let next_cursor = Cursor {
                    file_idx,
                    offset: next_offset,
                };
                return Ok(Page {
                    entries,
                    cursor: Some(next_cursor),
                });
            }

            // Finished reading this file without reaching quota, move to the
            // next and loop again
            position = self.next_file_position(file_idx + 1);
        }

        // Either ran out of files or reached the limit without more to read
        Ok(Page {
            entries,
            cursor: None,
        })
    }

    /// Parse and filter a log entry from a line of bytes.
    ///
    /// Returns `None` if the line cannot be parsed as valid JSON, is missing
    /// required fields (`timestamp`, `level`, `message`), or has a severity
    /// level below the configured minimum.
    fn parse_entry(&self, line: &[u8]) -> Option<Entry> {
        let raw: serde_json::Value = serde_json::from_slice(line).ok()?;

        // Filter by severity level (note: tracing levels are ordered by
        // verbosity, with TRACE > DEBUG > INFO > WARN > ERROR)
        let level = raw.get("level")?.as_str()?;
        if Level::from_str(level).ok()? > self.min_level {
            return None;
        }

        Some(Entry {
            timestamp: raw.get("timestamp")?.as_str()?.to_string(),
            level: level.to_string(),
            message: raw.get("message")?.as_str()?.to_string(),
            raw,
        })
    }

    /// Locate the position of the next log file to read from.
    ///
    /// This method should be called when the current file has been drained. It
    /// scans forward from `start_idx` (inclusive) to find the next (older)
    /// non-empty log file. If found, it returns the file index and its length
    /// in bytes (to indicate that we start reading from the end). Otherwise it
    /// returns `None`.
    fn next_file_position(&self, start_idx: usize) -> Option<(usize, u64)> {
        let mut idx = start_idx;
        while idx < self.files.len() {
            let len = self.files[idx].metadata().map_or(0, |m| m.len());
            if len > 0 {
                return Some((idx, len));
            }
            idx += 1;
        }
        None
    }

    /// Locate the start position to read from.
    ///
    /// If no cursor is provided, this locates the first (latest) non-empty log
    /// file and starts from its end. Otherwise, it resumes from the specified
    /// file and offset in the cursor.
    ///
    /// Specially, if the cursor's offset is zero, this method automatically
    /// moves to the end of the next (older) file. If the cursor points to an
    /// invalid file index, it is treated as if no cursor is provided.
    ///
    /// This method returns `None` if there are no more files to read.
    fn start_position(&self, cursor: &Option<Cursor>) -> Option<(usize, u64)> {
        match cursor {
            None => self.next_file_position(0),
            Some(c) => {
                if c.offset > 0 {
                    if c.file_idx < self.files.len() {
                        Some((c.file_idx, c.offset))
                    } else {
                        // Invalid file index in cursor, treat as no cursor
                        self.next_file_position(0)
                    }
                } else {
                    self.next_file_position(c.file_idx + 1)
                }
            },
        }
    }

    /// Read log entries backwards from a file, up to a limit.
    ///
    /// This method reads the specified file from `end_offset` backwards, and
    /// stops when either the file is drained or `limit_remaining` entries have
    /// been collected.
    ///
    /// This method returns the collected entries and an optional byte offset
    /// indicating where to resume reading on the next call. If the entire file
    /// has been read, the returned offset is `None`.
    fn read_file(
        &mut self,
        file_idx: usize,
        mut end_offset: u64,
        limit_remaining: usize,
    ) -> Result<(Vec<Entry>, Option<u64>)> {
        let mut file = File::open(&self.files[file_idx])?;
        let mut matches = vec![];

        // Buffer to accumulate bytes for the current line, but because we read
        // backwards the bytes would be in reverse order
        let mut current_line_rev = vec![];

        while end_offset > 0 && matches.len() < limit_remaining {
            let block_start = end_offset.saturating_sub(Self::BLOCK_SIZE);
            let block_len = (end_offset - block_start) as usize;

            file.seek(SeekFrom::Start(block_start))?;
            file.read_exact(&mut self.buf[..block_len])?;

            for i in (0..block_len).rev() {
                let byte = self.buf[i];
                let abs_pos = block_start + i as u64;

                if byte == b'\n' {
                    if !current_line_rev.is_empty() {
                        // Revert the accumulated line bytes to the correct
                        // order; we don't need to care about CR because
                        // trailing CR is still acceptable JSON
                        current_line_rev.reverse();
                        let line_bytes = std::mem::take(&mut current_line_rev);

                        if let Some(entry) = self.parse_entry(&line_bytes) {
                            matches.push(entry);
                            if matches.len() >= limit_remaining {
                                // `abs_pos` is the position of the newline
                                // before this processed line, so the next read
                                // should start (backwards) from there
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

        // When we reach offset 0 and break out of the loop, the very first line
        // of the file may still be pending processing because it typically
        // won't have a preceding newline character
        if end_offset == 0 && !current_line_rev.is_empty() && matches.len() < limit_remaining {
            current_line_rev.reverse();
            let line_bytes = std::mem::take(&mut current_line_rev);

            if let Some(entry) = self.parse_entry(&line_bytes) {
                matches.push(entry);
            }
        }

        Ok((matches, None)) // Entire file read without exceeding limit
    }
}
