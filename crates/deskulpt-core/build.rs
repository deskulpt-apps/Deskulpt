fn main() {
    deskulpt_build::Builder::default()
        .commands(&[
            "complete_setup",
            "call_plugin",
            "open_widget",
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
