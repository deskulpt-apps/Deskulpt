use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fs::{self, OpenOptions, create_dir_all};
use std::panic::PanicInfo;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use std::{fmt, thread};

use anyhow::{Context, Result};
use deskulpt_core::path::PathExt;
use once_cell::sync::OnceCell;
use serde_json::{Map as JsonMap, Value as JsonValue};
use tauri::{App, Runtime};
use tracing::field::{Field, Visit};
use tracing::{Event, Subscriber};
use tracing_appender::non_blocking::{self, NonBlocking, WorkerGuard};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::format::{FormatEvent, FormatFields, Writer as FormatWriter};
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::fmt::{self, FmtContext};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry::{LookupSpan, Registry};

/// Base file name for the rolling log files.
const LOG_FILE_PREFIX: &str = "app";
/// File suffix (extension) for the rolling log files.
const LOG_FILE_SUFFIX: &str = "log";
/// Prefix used once a log file has been rotated and renamed.
const PRIMARY_ROTATED_PREFIX: &str = "app.log.";
/// Legacy prefix created by `RollingFileAppender` before we normalise names.
const LEGACY_ROTATED_PREFIX: &str = "app.";
/// Legacy suffix created by `RollingFileAppender` before we normalise names.
const LEGACY_ROTATED_SUFFIX: &str = ".log";
/// The canonical log file consumers will tail.
const PRIMARY_LOG_BASENAME: &str = "app.log";
/// Maximum number of rotated log files, not counting the live `app.log`.
const MAX_LOG_FILES: usize = 10;
/// How often we update the `app.log` hard link to the active file.
const PRIMARY_LINK_REFRESH: Duration = Duration::from_secs(30);

/// Ensures the logging guard lives for the lifetime of the application.
static LOG_GUARD: OnceCell<WorkerGuard> = OnceCell::new();
/// Guards the setup routine so `init` is only executed once.
static INIT: OnceCell<()> = OnceCell::new();
/// Guards the custom panic hook installation.
static PANIC_HOOK: OnceCell<()> = OnceCell::new();
/// Ensures we only spawn the maintenance task once.
static LINK_TASK: OnceCell<()> = OnceCell::new();

/// Snapshot of a rotated log file used to normalise naming and prune history.
#[derive(Clone)]
struct LogInfo {
    path: PathBuf,
    created: SystemTime,
    date: String,
    sequence: Option<usize>,
}

/// Initialize tracing with JSON output and panic logging.
pub fn init<R: Runtime>(app: &mut App<R>) -> Result<()> {
    if INIT.get().is_some() {
        return Ok(());
    }

    let logs_dir = prepare_logs_dir(app)?;
    let (writer, guard) = build_writer(&logs_dir)?;
    install_subscriber(writer)?;
    retain_guard(guard);
    install_panic_hook();

    INIT.set(()).ok();
    Ok(())
}

fn prepare_logs_dir<R: Runtime>(app: &App<R>) -> Result<PathBuf> {
    let persist_dir = app
        .persist_dir()
        .context("persist dir must be initialised before logging")?;
    let logs_dir = persist_dir.join("logs");
    create_dir_all(&logs_dir).context("failed to create logs directory")?;
    Ok(logs_dir)
}

fn build_writer(logs_dir: &Path) -> Result<(NonBlocking, WorkerGuard)> {
    let appender = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        // Keep extra headroom here; we normalise names and prune to exactly
        // `MAX_LOG_FILES` ourselves once the files land on disk.
        .max_log_files(MAX_LOG_FILES * 5)
        .filename_prefix(LOG_FILE_PREFIX)
        .filename_suffix(LOG_FILE_SUFFIX)
        .build(logs_dir)
        .context("failed to build rolling file appender")?;

    let (writer, guard) = non_blocking::NonBlockingBuilder::default()
        .lossy(false)
        .finish(appender);

    // Ensure an `app.log` placeholder exists immediately.
    let _ = create_primary_placeholder(logs_dir);
    if let Err(error) = refresh_primary_log_link(logs_dir) {
        eprintln!("deskulpt logging: unable to link app.log to active log: {error:?}");
    }
    if let Err(error) = apply_sequence_suffixes(logs_dir) {
        eprintln!("deskulpt logging: unable to apply log sequence suffix: {error:?}");
    }
    if let Err(error) = refresh_primary_log_link(logs_dir) {
        eprintln!("deskulpt logging: unable to link app.log to active log: {error:?}");
    }
    start_primary_link_task(logs_dir.to_path_buf());

    Ok((writer, guard))
}

fn install_subscriber(writer: NonBlocking) -> Result<()> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let fmt_layer = fmt::layer()
        .with_ansi(false)
        .event_format(NdjsonFormatter::default())
        .with_writer(writer);

    let subscriber = Registry::default().with(env_filter).with(fmt_layer);

    tracing::subscriber::set_global_default(subscriber)
        .context("failed to install tracing subscriber")
}

fn retain_guard(guard: WorkerGuard) {
    // Ignore repeated initialisation attempts if they happen.
    let _ = LOG_GUARD.set(guard);
}

fn install_panic_hook() {
    if PANIC_HOOK.get().is_some() {
        return;
    }

    let previous_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        log_panic(panic_info);
        previous_hook(panic_info);
    }));

    PANIC_HOOK.set(()).ok();
}

fn log_panic(panic_info: &PanicInfo<'_>) {
    let backtrace = std::backtrace::Backtrace::force_capture();
    let message = panic_message(panic_info);

    if let Some(location) = panic_info.location() {
        tracing::error!(
            target: "panic",
            backtrace = %backtrace,
            panic.file = location.file(),
            panic.line = location.line(),
            "Unhandled panic: {message}"
        );
    } else {
        tracing::error!(target: "panic", backtrace = %backtrace, "Unhandled panic: {message}");
    }
}

fn panic_message(panic_info: &PanicInfo<'_>) -> String {
    if let Some(message) = panic_info.payload().downcast_ref::<&str>() {
        (*message).to_owned()
    } else if let Some(message) = panic_info.payload().downcast_ref::<String>() {
        message.clone()
    } else {
        "panic payload is not a string".to_owned()
    }
}

fn create_primary_placeholder(logs_dir: &Path) -> Result<()> {
    OpenOptions::new()
        .create(true)
        .append(true)
        .open(logs_dir.join(PRIMARY_LOG_BASENAME))
        .map(|_| ())
        .context("failed to create app.log placeholder")
}

fn start_primary_link_task(logs_dir: PathBuf) {
    if LINK_TASK.get().is_some() {
        return;
    }

    let _ = thread::Builder::new()
        .name("deskulpt-log-link".into())
        .spawn(move || {
            loop {
                if let Err(error) = apply_sequence_suffixes(&logs_dir) {
                    eprintln!("deskulpt logging: unable to refresh log sequences: {error:?}");
                }
                if let Err(error) = refresh_primary_log_link(&logs_dir) {
                    eprintln!("deskulpt logging: unable to refresh app.log link: {error:?}");
                }
                thread::sleep(PRIMARY_LINK_REFRESH);
            }
        });

    let _ = LINK_TASK.set(());
}

fn refresh_primary_log_link(logs_dir: &Path) -> Result<()> {
    let mut newest: Option<(PathBuf, SystemTime)> = None;

    for entry in fs::read_dir(logs_dir).context("failed to read logs directory")? {
        let entry = entry.context("failed to read log directory entry")?;
        let metadata = entry.metadata().context("failed to read log metadata")?;

        if !metadata.is_file() {
            continue;
        }

        let name = entry.file_name();
        let name = name.to_string_lossy();
        if !is_rotated_log(&name) {
            continue;
        }

        let created = metadata
            .created()
            .or_else(|_| metadata.modified())
            .context("failed to read log timestamps")?;

        match newest {
            Some((_, ref mut ts)) if *ts >= created => {},
            _ => {
                newest = Some((entry.path(), created));
            },
        }
    }

    let primary_path = logs_dir.join(PRIMARY_LOG_BASENAME);

    if let Some((latest_path, _)) = newest {
        if primary_path.exists() {
            let _ = fs::remove_file(&primary_path);
        }

        if let Err(link_err) = fs::hard_link(&latest_path, &primary_path) {
            fs::copy(&latest_path, &primary_path).with_context(|| {
                format!("failed to copy latest log to app.log after hard link error: {link_err}")
            })?;
        }
    } else {
        create_primary_placeholder(logs_dir)?;
    }

    Ok(())
}

fn is_rotated_log(name: &str) -> bool {
    parse_log_components(name).is_some()
}

/// Renames legacy `app.YYYY-MM-DD.log` files into `app.log.YYYY-MM-DD.N` and
/// keeps the most recent `MAX_LOG_FILES` historical logs.
fn apply_sequence_suffixes(logs_dir: &Path) -> Result<()> {
    let mut groups: BTreeMap<String, Vec<LogInfo>> = BTreeMap::new();
    for info in collect_rotated_logs(logs_dir)? {
        groups.entry(info.date.clone()).or_default().push(info);
    }

    let mut all_logs = Vec::new();

    for (date, mut infos) in groups {
        infos.sort_by(|a, b| match a.created.cmp(&b.created) {
            Ordering::Equal => a.sequence.cmp(&b.sequence),
            ordering => ordering,
        });

        for (idx, mut info) in infos.into_iter().enumerate() {
            let sequence = idx + 1;
            let target = logs_dir.join(format!("{PRIMARY_LOG_BASENAME}.{date}.{sequence}"));
            if info.path != target {
                if target.exists() {
                    fs::remove_file(&target).ok();
                }
                fs::rename(&info.path, &target).with_context(|| {
                    format!(
                        "failed to rename {} to {}",
                        info.path.display(),
                        target.display()
                    )
                })?;
                info.path = target;
            }
            info.sequence = Some(sequence);
            all_logs.push(info);
        }
    }

    prune_sequence(all_logs)?;

    Ok(())
}

fn collect_rotated_logs(logs_dir: &Path) -> Result<Vec<LogInfo>> {
    let mut infos = Vec::new();

    for entry in fs::read_dir(logs_dir).context("failed to read logs directory for sequencing")? {
        let entry = entry.context("failed to read log directory entry")?;
        if !entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
            continue;
        }

        let Ok(file_name) = entry.file_name().into_string() else {
            continue;
        };

        let Some((date, sequence)) = parse_log_components(&file_name) else {
            continue;
        };

        let metadata = entry.metadata().context("failed to read log metadata")?;
        let created = metadata
            .created()
            .or_else(|_| metadata.modified())
            .unwrap_or(SystemTime::UNIX_EPOCH);

        infos.push(LogInfo {
            path: entry.path(),
            created,
            date,
            sequence,
        });
    }

    Ok(infos)
}

fn parse_log_components(name: &str) -> Option<(String, Option<usize>)> {
    if let Some(rest) = name.strip_prefix(PRIMARY_ROTATED_PREFIX) {
        let mut parts = rest.split('.');
        let date = parts.next()?.to_owned();
        let sequence = parts.next().and_then(|value| value.parse().ok());
        return Some((date, sequence));
    }

    if let Some(rest) = name.strip_prefix(LEGACY_ROTATED_PREFIX) {
        let rest = rest.strip_suffix(LEGACY_ROTATED_SUFFIX)?;
        return Some((rest.to_owned(), None));
    }

    None
}

/// Drops the oldest log files so only `MAX_LOG_FILES` rotated logs remain.
fn prune_sequence(mut logs: Vec<LogInfo>) -> Result<()> {
    if logs.len() <= MAX_LOG_FILES {
        return Ok(());
    }

    logs.sort_by(|a, b| b.created.cmp(&a.created));

    for info in logs.into_iter().skip(MAX_LOG_FILES) {
        if let Err(error) = fs::remove_file(&info.path) {
            eprintln!(
                "deskulpt logging: failed to remove old log file {}: {error:?}",
                info.path.display()
            );
        }
    }

    Ok(())
}

#[derive(Clone)]
struct NdjsonFormatter {
    timer: UtcTime,
}

impl NdjsonFormatter {
    const fn new() -> Self {
        Self {
            timer: UtcTime::rfc3339(),
        }
    }
}

impl Default for NdjsonFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl<S, N> FormatEvent<S, N> for NdjsonFormatter
where
    S: Subscriber + for<'lookup> LookupSpan<'lookup>,
    N: for<'writer> FormatFields<'writer> + 'static,
{
    fn format_event(
        &self,
        _: &FmtContext<'_, S, N>,
        writer: &mut FormatWriter<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        let mut timestamp = String::new();
        self.timer.format_time(&mut timestamp)?;

        let metadata = event.metadata();
        let mut json = JsonMap::new();
        json.insert("ts".into(), JsonValue::String(timestamp));
        json.insert(
            "level".into(),
            JsonValue::String(metadata.level().to_string()),
        );
        json.insert(
            "target".into(),
            JsonValue::String(metadata.target().to_owned()),
        );

        if let Some(file) = metadata.file() {
            json.insert("file".into(), JsonValue::String(file.to_owned()));
        } else {
            json.insert("file".into(), JsonValue::Null);
        }

        if let Some(line) = metadata.line() {
            json.insert("line".into(), JsonValue::Number((line as u64).into()));
        } else {
            json.insert("line".into(), JsonValue::Null);
        }

        let mut visitor = JsonVisitor::new(&mut json);
        event.record(&mut visitor);

        json.entry("message".into())
            .or_insert_with(|| JsonValue::String(String::new()));

        let line = serde_json::to_string(&JsonValue::Object(json)).map_err(|_| fmt::Error)?;
        writer.write_str(&line)?;
        writer.write_char('\n')
    }
}

struct JsonVisitor<'a> {
    map: &'a mut JsonMap<String, JsonValue>,
}

impl<'a> JsonVisitor<'a> {
    fn new(map: &'a mut JsonMap<String, JsonValue>) -> Self {
        Self { map }
    }

    fn insert(&mut self, field: &Field, value: JsonValue) {
        self.map.insert(field.name().to_owned(), value);
    }
}

impl<'a> Visit for JsonVisitor<'a> {
    fn record_bool(&mut self, field: &Field, value: bool) {
        self.insert(field, JsonValue::Bool(value));
    }

    fn record_i64(&mut self, field: &Field, value: i64) {
        self.insert(field, JsonValue::Number(value.into()));
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        self.insert(field, JsonValue::Number(value.into()));
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        self.insert(field, JsonValue::String(value.to_owned()));
    }

    fn record_f64(&mut self, field: &Field, value: f64) {
        if let Some(number) = serde_json::Number::from_f64(value) {
            self.insert(field, JsonValue::Number(number));
        } else {
            self.record_debug(field, &value);
        }
    }

    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        self.insert(field, JsonValue::String(format!("{value:?}")));
    }
}
