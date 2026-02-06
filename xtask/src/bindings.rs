use anyhow::Result;

mod index;
mod module;

pub fn run() -> Result<()> {
    let all_bindings = vec![
        tauri_plugin_deskulpt_core::build_bindings(),
        tauri_plugin_deskulpt_settings::build_bindings(),
        tauri_plugin_deskulpt_widgets::build_bindings(),
        tauri_plugin_deskulpt_logs::build_bindings(),
    ];

    let mut index = index::Template::default();
    for bindings in all_bindings {
        let template = module::Template::from(bindings)?;
        index.add_module(template.module);
        template.render()?;
    }
    index.render()?;

    Ok(())
}
