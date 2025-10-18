fn main() {
    deskulpt_build::Builder::default()
        .commands(&[
            "bundle_widgets",
            "call_plugin",
            "open_widget",
            "rescan_widgets",
            "set_render_ready",
            "update_settings",
        ])
        .events(&[
            "RenderWidgetsEvent",
            "ShowToastEvent",
            "UpdateSettingsEvent",
            "UpdateWidgetCatalogEvent",
        ])
        .build();
}
