/*! Auto-generated via `cargo xtask bindings`. DO NOT EDIT! */

import { invoke } from "@tauri-apps/api/core";

// =============================================================================
// Types
// =============================================================================

/**
 * Cursor for log pagination.
 */
export type Cursor = { 
/**
 * The index of the log file in the files list.
 */
fileIdx: number; 
/**
 * The byte offset within the log file.
 * 
 * When continuing from this cursor, reading resumes backwards from this
 * offset. An offset of zero means this file has been fully read, and the
 * reader should move to the next older file.
 */
offset: number }

/**
 * Deskulpt window enum.
 */
export type DeskulptWindow = 
/**
 * Deskulpt portal.
 */
"portal" | 
/**
 * Deskulpt canvas.
 */
"canvas"

/**
 * A single log entry.
 */
export type Entry = { 
/**
 * Timestamp of the log entry in RFC 3339 format.
 */
timestamp: string; 
/**
 * The stringified logging level (e.g., "INFO", "ERROR").
 */
level: string; 
/**
 * The log message text.
 */
message: string; 
/**
 * The complete raw JSON object representing the log entry.
 */
raw: JsonValue }

export type JsonValue = null | boolean | number | string | JsonValue[] | { [key in string]: JsonValue }

/**
 * Level of severity for logging.
 */
export type Level = 
/**
 * At least the severity of [`tracing::Level::TRACE`].
 */
"trace" | 
/**
 * At least the severity of [`tracing::Level::DEBUG`].
 */
"debug" | 
/**
 * At least the severity of [`tracing::Level::INFO`].
 */
"info" | 
/**
 * At least the severity of [`tracing::Level::WARN`].
 */
"warn" | 
/**
 * At least the severity of [`tracing::Level::ERROR`].
 */
"error"

/**
 * A page of log entries.
 */
export type Page = { 
/**
 * Log entries in reverse chronological order (most recent first).
 */
entries: Entry[]; 
/**
 * Cursor for reading the next page of older log entries.
 * 
 * If `None`, there are no more entries to read beyond this page.
 */
cursor: Cursor | null }


// =============================================================================
// Commands
// =============================================================================

export namespace Commands {
  /**
   * Clear all log files.
   * 
   * This returns the amount of freed space in bytes.
   */
  export const clear = () => invoke<number>("plugin:deskulpt-logs|clear");

  /**
   * Read a page of log entries.
   * 
   * This retrieves log entries from the log files, from newest to oldest. At
   * most `limit` log entries will be returned. Only log entries with at least
   * the severity of `min_level` will be included.
   * 
   * An optional `cursor` can be provided. Pass `null` to start from the latest
   * log entry. Pass a cursor returned from a previous call to continue reading
   * from where you left off. An invalid cursor will be ignored.
   */
  export const read = (
    limit: number,
    minLevel: Level,
    cursor: Cursor | null,
  ) => invoke<Page>("plugin:deskulpt-logs|read", {
    limit,
    minLevel,
    cursor,
  });

  /**
   * Emit a log message at the specified level.
   * 
   * This command allows the frontend to send log messages to the backend's
   * logging system, tagged by the window label they originate from.
   * 
   * The `meta` parameter accepts any JSON-serializable value to include extra
   * metadata along with the log message. Pass `null` if no metadata is needed.
   */
  export const log = (
    level: Level,
    message: string,
    meta: JsonValue,
  ) => invoke<null>("plugin:deskulpt-logs|log", {
    level,
    message,
    meta,
  });
}
