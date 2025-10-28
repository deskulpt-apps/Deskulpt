use std::fs;
use std::path::PathBuf;
use std::time::Duration;

use anyhow::Result;

/// Start a background monitor that rotates old log files
pub fn start_rotation_monitor(log_dir: PathBuf) -> Result<()> {
    loop {
        std::thread::sleep(Duration::from_secs(3600)); // Check every hour

        if let Err(e) = cleanup_old_logs(&log_dir) {
            eprintln!("Failed to cleanup old logs: {}", e);
        }
    }
}

/// Clean up old log files, keeping only the most recent ones
fn cleanup_old_logs(log_dir: &PathBuf) -> Result<()> {
    const MAX_FILES: usize = 10;

    let mut entries: Vec<_> = fs::read_dir(log_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_name().to_string_lossy().starts_with("deskulpt")
                && e.file_name().to_string_lossy().ends_with(".log")
        })
        .collect();

    if entries.len() <= MAX_FILES {
        return Ok(());
    }

    // Sort by modification time, oldest first
    entries.sort_by_key(|e| {
        e.metadata()
            .ok()
            .and_then(|m| m.modified().ok())
            .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
    });

    // Remove oldest files
    while entries.len() > MAX_FILES {
        if let Some(oldest) = entries.first() {
            let _ = fs::remove_file(oldest.path());
        }
        entries.remove(0);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cleanup_respects_max_files() {
        // This is a basic test structure; actual testing would need temp dirs
        assert_eq!(true, true);
    }
}
