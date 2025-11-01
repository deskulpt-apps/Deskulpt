fn main() {
    deskulpt_build::Builder::default()
        .commands(&[
            "complete_setup",
            "call_plugin",
            "open_widget",
            "refresh_widget",
            "refresh_widgets_all",
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
