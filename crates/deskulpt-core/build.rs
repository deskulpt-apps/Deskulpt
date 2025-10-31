fn main() {
    deskulpt_build::Builder::default()
        .commands(&[
            "bundle_widgets",
            "complete_setup",
            "call_plugin",
            "open_widget",
            "rescan_widgets",
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
