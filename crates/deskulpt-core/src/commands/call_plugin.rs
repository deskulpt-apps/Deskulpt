use deskulpt_common::{SerResult, ser_bail};
use once_cell::sync::Lazy;
use tauri::{AppHandle, Runtime, command};
use tokio::sync::Mutex;
use tracing::{Instrument, info_span};
use uuid::Uuid;

use crate::logging::{self, TriggerContext, WidgetContext};
use crate::path::PathExt;

// TODO: Remove this temporary implementation
static FS_PLUGIN: Lazy<Mutex<deskulpt_plugin_fs::FsPlugin>> =
    Lazy::new(|| Mutex::new(deskulpt_plugin_fs::FsPlugin));

// TODO: Remove this temporary implementation
static SYS_PLUGIN: Lazy<Mutex<deskulpt_plugin_sys::SysPlugin>> =
    Lazy::new(|| Mutex::new(Default::default()));

/// Call a plugin command (🚧 TODO 🚧).
///
/// ### 🚧 TODO 🚧
///
/// The Deskulpt core should keep a state of the registered plugins and call the
/// plugins dynamically. Also, instead of invoking the plugins directly, the
/// Deskulpt core should not depend on any of the plugins and should use IPC to
/// communicate with the plugins.
///
/// Also, in order to simplify the engine API for the plugin (because it is
/// a temporary implementation), `app_handle` is using the default runtime but
/// it should be a generic `R: Runtime` parameter in the final implementation.
#[command]
#[specta::specta]
pub async fn call_plugin<R: Runtime>(
    app_handle: AppHandle<R>,
    plugin: String,
    command: String,
    id: String,
    payload: Option<serde_json::Value>,
) -> SerResult<serde_json::Value> {
    let request_id = Uuid::new_v4();
    let span = info_span!(
        "widget_command",
        widget_id = %id,
        plugin_id = %plugin,
        request_id = %request_id,
        plugin_command = %command,
    );
    let widget_ctx = WidgetContext::new(id.clone(), Some(plugin.clone()));
    logging::attach_widget_context(&span, widget_ctx.widget_id(), widget_ctx.plugin_id());
    logging::attach_trigger(
        &span,
        TriggerContext::new(command.clone(), Some(widget_ctx)),
    );

    async move {
        let widget_dir_fn = move |id: &str| app_handle.widget_dir(id);

        match plugin.as_str() {
            "fs" => {
                let plugin = FS_PLUGIN.lock().await;
                let result = deskulpt_plugin::call_plugin(
                    widget_dir_fn,
                    &*plugin,
                    command.as_str(),
                    id,
                    payload,
                )?;
                Ok(result)
            },
            "sys" => {
                let plugin = SYS_PLUGIN.lock().await;
                let result = deskulpt_plugin::call_plugin(
                    widget_dir_fn,
                    &*plugin,
                    command.as_str(),
                    id,
                    payload,
                )?;
                Ok(result)
            },
            _ => ser_bail!("Unknown plugin: {}", plugin),
        }
    }
    .instrument(span)
    .await
}
