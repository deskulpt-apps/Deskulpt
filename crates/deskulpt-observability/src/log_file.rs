use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use anyhow::Result;

/// Configuration for log file rotation.
pub struct RotationConfig {
    /// Maximum size per log file in bytes.
    pub max_file_size: u64,
    /// Maximum number of log files to keep.
    pub max_files: usize,
    /// Function to generate log file names.
    pub format_fn: Box<dyn Fn(usize) -> String + Send + Sync>,
}

/// A rotating file appender that rotates logs based on size.
pub struct RotatingFileAppender {
    log_dir: PathBuf,
    current_file: Mutex<Option<File>>,
    current_size: Mutex<u64>,
    config: RotationConfig,
}

impl RotatingFileAppender {
    /// Create a new rotating file appender.
    pub fn new(log_dir: PathBuf, config: RotationConfig) -> Result<Self> {
        std::fs::create_dir_all(&log_dir)?;

        let appender = RotatingFileAppender {
            log_dir,
            current_file: Mutex::new(None),
            current_size: Mutex::new(0),
            config,
        };

        // Initialize by opening the current log file
        appender.ensure_file_open()?;

        Ok(appender)
    }

    fn ensure_file_open(&self) -> Result<()> {
        let mut current_file = self.current_file.lock().unwrap();

        if current_file.is_some() {
            return Ok(());
        }

        // Clean up old logs
        self.cleanup_old_logs()?;

        // Open the current log file
        let file_name = (self.config.format_fn)(0);
        let file_path = self.log_dir.join(&file_name);

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&file_path)?;

        let size = file.metadata()?.len();

        *current_file = Some(file);
        *self.current_size.lock().unwrap() = size;

        Ok(())
    }

    fn cleanup_old_logs(&self) -> Result<()> {
        let mut entries: Vec<_> = std::fs::read_dir(&self.log_dir)?
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.file_name().to_string_lossy().starts_with("deskulpt-")
                    && e.file_name().to_string_lossy().ends_with(".log")
            })
            .collect();

        // Sort by modification time, oldest first
        entries.sort_by_key(|e| {
            e.metadata()
                .ok()
                .and_then(|m| m.modified().ok())
                .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
        });

        // Remove old files if we exceed max_files
        while entries.len() > self.config.max_files {
            if let Some(oldest) = entries.first() {
                let _ = std::fs::remove_file(oldest.path());
            }
            entries.remove(0);
        }

        Ok(())
    }

    fn rotate(&self) -> Result<()> {
        let mut current_file = self.current_file.lock().unwrap();

        if let Some(file) = current_file.take() {
            drop(file);
        }

        // Rename old files
        for i in (0..self.config.max_files - 1).rev() {
            let current_name = (self.config.format_fn)(i);
            let next_name = (self.config.format_fn)(i + 1);

            let current_path = self.log_dir.join(&current_name);
            let next_path = self.log_dir.join(&next_name);

            if current_path.exists() {
                let _ = std::fs::rename(&current_path, &next_path);
            }
        }

        // Open new file
        let file_name = (self.config.format_fn)(0);
        let file_path = self.log_dir.join(&file_name);

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&file_path)?;

        *current_file = Some(file);
        *self.current_size.lock().unwrap() = 0;

        Ok(())
    }
}

impl Write for RotatingFileAppender {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let len = buf.len() as u64;
        let mut size = self.current_size.lock().unwrap();

        // Check if rotation is needed
        if *size + len > self.config.max_file_size {
            if let Err(e) = self.rotate() {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Failed to rotate log file: {}", e),
                ));
            }
            *size = 0;
        }

        let mut current_file = self.current_file.lock().unwrap();
        if let Some(ref mut file) = *current_file {
            file.write_all(buf)?;
            *size += len;
            Ok(buf.len())
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "No file open"))
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        let mut current_file = self.current_file.lock().unwrap();
        if let Some(ref mut file) = *current_file {
            file.flush()
        } else {
            Ok(())
        }
    }
}

impl<'a> Write for &'a RotatingFileAppender {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let len = buf.len() as u64;
        let mut size = self.current_size.lock().unwrap();

        // Check if rotation is needed
        if *size + len > self.config.max_file_size {
            if let Err(e) = self.rotate() {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Failed to rotate log file: {}", e),
                ));
            }
            *size = 0;
        }

        let mut current_file = self.current_file.lock().unwrap();
        if let Some(ref mut file) = *current_file {
            file.write_all(buf)?;
            *size += len;
            Ok(buf.len())
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "No file open"))
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        let mut current_file = self.current_file.lock().unwrap();
        if let Some(ref mut file) = *current_file {
            file.flush()
        } else {
            Ok(())
        }
    }
}
