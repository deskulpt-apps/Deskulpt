fn main() {
    deskulpt_build::Builder::default()
        .commands(&[
            "call_plugin",
            "open_widget",
            "open_logs_dir",
            "list_logs",
            "read_log",
            "clear_logs",
        ])
        .plain_commands(&["log"])
        .events(&["ShowToastEvent"])
        .build();
}
