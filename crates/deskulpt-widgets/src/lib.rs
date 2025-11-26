#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://github.com/deskulpt-apps/Deskulpt/raw/main/public/deskulpt.svg",
    html_favicon_url = "https://github.com/deskulpt-apps/Deskulpt/raw/main/public/deskulpt.svg"
)]

mod catalog;
mod commands;
mod events;
mod manager;
mod render;
mod setup;

use std::fs;
use std::path::Path;

use deskulpt_core::path::PathExt;
pub use manager::WidgetsManager;
use serde_json::Value;
use tauri::plugin::TauriPlugin;
use tauri::{Manager, Runtime};

deskulpt_common::bindings::build_bindings!();

/// Initialize the internal Deskulpt widgets plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    deskulpt_common::init::init_builder!()
        .setup(|app_handle, _| {
            seed_welcome_widget_if_empty(app_handle)?;
            app_handle.manage(WidgetsManager::new(app_handle.clone()));
            Ok(())
        })
        .build()
}

/// Extension to [`Manager`] for accessing Deskulpt widgets APIs.
trait WidgetsExt<R: Runtime> {
    /// Get a reference to the [`WidgetsManager`] to access the APIs.
    fn widgets(&self) -> &WidgetsManager<R>;
}

impl<R: Runtime, M: Manager<R>> WidgetsExt<R> for M {
    fn widgets(&self) -> &WidgetsManager<R> {
        self.state::<WidgetsManager<R>>().inner()
    }
}

fn seed_welcome_widget_if_empty<R: Runtime>(app: &tauri::AppHandle<R>) -> tauri::Result<()> {
    let widgets_dir = app.widgets_dir()?;
    let mut dir_names = vec![];
    for entry in fs::read_dir(widgets_dir)?.filter_map(Result::ok) {
        if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
            let name = entry.file_name();
            let name = name.to_string_lossy().to_string();
            if !name.starts_with('.') {
                dir_names.push(name);
            }
        }
    }

    let mut needs_seed = dir_names.is_empty();
    if !dir_names.is_empty() {
        if dir_names.len() == 1 && dir_names[0] == "welcome" {
            let manifest_path = widgets_dir.join("welcome/deskulpt.widget.json");
            let manifest: Value =
                serde_json::from_reader(fs::File::open(&manifest_path)?).unwrap_or_default();
            let entry = manifest
                .get("entry")
                .and_then(|v| v.as_str())
                .unwrap_or_default();
            let index_exists = widgets_dir.join("welcome/index.tsx").exists();
            needs_seed = entry != "index.tsx" || !index_exists;
            if needs_seed {
                println!(
                    "Updating bundled welcome widget to latest format in {}",
                    widgets_dir.display()
                );
            }
        } else {
            println!(
                "Skipping welcome widget seeding; widgets directory not empty (dirs found): {:?}",
                dir_names
            );
            return Ok(());
        }
    }

    if !needs_seed {
        return Ok(());
    }

    let resource_dir = app.path().resource_dir()?;
    let src = resource_dir.join("default-widgets/welcome");

    if src.exists() {
        let dst = widgets_dir.join("welcome");
        if dst.exists() {
            fs::remove_dir_all(&dst)?;
        }
        println!(
            "Seeding bundled welcome widget from {} to {}",
            src.display(),
            dst.display()
        );
        copy_welcome_files(&src, &dst)?;
    }

    Ok(())
}

fn copy_welcome_files(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_file() {
            fs::copy(entry.path(), dst.join(entry.file_name()))?;
        }
    }
    Ok(())
}
