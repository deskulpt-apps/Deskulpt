fn main() {
    deskulpt_build::Builder::default()
        .commands(&["call_plugin", "open_widget", "update_settings"])
        .events(&["ShowToastEvent", "UpdateSettingsEvent"])
        .build();
}
