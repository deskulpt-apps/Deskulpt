fn main() {
    deskulpt_build::Builder::default()
        .commands(&[
            "complete_setup",
            "call_plugin",
            "open_widget",
            "refresh_all_widgets",
            "refresh_widget",
            "update_settings",
        ])
        .events(&[
            "RenderWidgetEvent",
            "ShowToastEvent",
            "UpdateSettingsEvent",
            "UpdateWidgetCatalogEvent",
        ])
        .build();
}
