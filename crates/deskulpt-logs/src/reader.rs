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
    /// Log entries in reverse chronological order, i.e., newest first.
    pub entries: Vec<Entry>,
    /// Cursor for reading the next page of older log entries.
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

/// Tail reader for rolling log files.
///
/// This reads the log files in the reverse order they are provided, and within
/// each file it reads backwards from the end towards the start.
pub struct RollingTailReader {
    /// List of log files to read from, ordered by most recent first.
    files: Vec<PathBuf>,
    /// The minimum logging level to filter entries.
    ///
    /// Entries with severity lower than this level are ignored when reading.
    min_level: Level,
    /// Reusable buffer for reading blocks of log contents.
    ///
    /// The size must be at least [`Self::BLOCK_SIZE`].
    buf: Vec<u8>,
}

impl RollingTailReader {
    /// Size of each block to read.
    ///
    /// This is set to 16KB to balance between performance and memory usage.
    /// Larger blocks reduce the number of read operations but increase memory
    /// consumption.
    const BLOCK_SIZE: u64 = 1 << 14;

    /// Create a new reader for the given files.
    ///
    /// The files should be ordered so that the reader starts by reading the
    /// last one and moves backwards till the first.
    pub fn new(files: Vec<PathBuf>, min_level: Level) -> Self {
        Self {
            files,
            min_level,
            buf: vec![0u8; Self::BLOCK_SIZE as usize],
        }
    }

    /// Read a page of log entries.
    ///
    /// The limit specifies the maximum number of log entries to retrieve and
    /// must be strictly positive. The cursor is for pagination. The first call
    /// should pass `None` for the cursor to start from the newest log entries,
    /// and subsequent calls should use the cursor returned from the previous
    /// call to read older entries. `min_level` filters log entries to only
    /// those at or above the specified logging level (ordered by severity).
    pub fn read(&mut self, limit: usize, cursor: Option<Cursor>) -> Result<Page> {
        assert!(limit > 0, "Limit must be strictly positive");

        if self.files.is_empty() {
            return Ok(Page {
                entries: Vec::new(),
                cursor: None,
                has_more: false,
            });
        }

        let mut entries = vec![];
        let mut position = self.resolve_initial_position(&cursor)?;

        while let Some((file_idx, end_offset)) = position {
            if entries.len() >= limit {
                break; // Reached the requested limit
            }

            // Sanity checks that we don't read past the end of file (if cursor
            // is somehow beyond file length), and we automatically move to the
            // next file if the cursor is already at 0
            let file_len = self.files[file_idx].metadata()?.len();
            let effective_end = end_offset.min(file_len);
            if effective_end == 0 {
                position = self.find_next_nonempty_file(file_idx + 1)?;
                continue;
            }

            let (mut file_entries, cursor_in_file) =
                self.scan_file_for_matches(file_idx, effective_end, limit - entries.len())?;

            entries.append(&mut file_entries);

            if let Some(next_offset) = cursor_in_file {
                // We have filled the quota while still within this file, so we
                // return a cursor pointing to where we left off
                let next_cursor = Cursor {
                    path: self.files[file_idx].clone(),
                    offset: next_offset,
                };
                return Ok(Page {
                    entries,
                    cursor: Some(next_cursor),
                    has_more: true,
                });
            }

            // Finished scanning this file without reaching quota, move to the
            // next and loop again
            position = self.find_next_nonempty_file(file_idx + 1)?;
        }

        // We ran out of files or reached the limit, either case we declare no
        // more further entries
        Ok(Page {
            entries,
            cursor: None,
            has_more: false,
        })
    }

    /// Parse a log entry from a line of bytes.
    ///
    /// This returns `None` if the bytes cannot be parsed as a valid log entry
    /// (e.g., invalid JSON or missing required fields), or if the log entry's
    /// level is below the minimum level.
    fn parse_entry(&self, line: &[u8]) -> Option<Entry> {
        let raw: serde_json::Value = serde_json::from_slice(line).ok()?;

        // We need to reject low severity, but tracing levels are ordered by
        // verbosity, so this is equivalent to rejecting high verbosity
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

    /// Find the next non-empty log file starting from the given index.
    ///
    /// If found, this returns `Ok(Some(...))` with the index in the files
    /// list and the length of the found file. If no non-empty file is found,
    /// this returns `Ok(None)`.
    fn find_next_nonempty_file(&self, start_idx: usize) -> Result<Option<(usize, u64)>> {
        let mut idx = start_idx;
        while idx < self.files.len() {
            let len = self.files[idx].metadata()?.len();
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
    /// cursor is provided, it locates the specified file and offset. If the
    /// offset is zero, it finds the next non-empty file after that one.
    ///
    /// If no suitable starting position is found, this returns `Ok(None)`,
    /// meaning there are no logs to read. Otherwise, it returns `Ok(Some(...))`
    /// with the file index and offset within that file.
    fn resolve_initial_position(&self, cursor: &Option<Cursor>) -> Result<Option<(usize, u64)>> {
        match cursor {
            None => self.find_next_nonempty_file(0),
            Some(c) => {
                let mut found_idx = None;
                for (i, p) in self.files.iter().enumerate() {
                    if p == &c.path {
                        found_idx = Some(i);
                        break;
                    }
                }

                let idx = found_idx.unwrap_or(0);
                if c.offset > 0 {
                    Ok(Some((idx, c.offset)))
                } else {
                    self.find_next_nonempty_file(idx + 1)
                }
            },
        }
    }

    /// Scan a log file backwards for matching log entries.
    ///
    /// This reads the file in blocks from the given end offset towards the
    /// start, extracting log entries that meet the minimum logging level until
    /// either the start of the file is reached or the limit of remaining
    /// entries is met.
    ///
    /// This method returns a vector of matching [`Entry`]s and an optional
    /// byte offset indicating where to continue scanning in the next call. If
    /// the entire file has been scanned, the offset is `None`.
    fn scan_file_for_matches(
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
                        // Reverse the accumulated line bytes to the correct order;
                        // we don't need to care about CR because trailing CR is
                        // still acceptable JSON
                        current_line_rev.reverse();
                        let line_bytes = std::mem::take(&mut current_line_rev);

                        if let Some(entry) = self.parse_entry(&line_bytes) {
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

            if let Some(entry) = self.parse_entry(&line_bytes) {
                matches.push(entry);
            }
        }

        Ok((matches, None)) // Entire file scanned without exceeding limit
    }
}
