fn main() {
    deskulpt_build::Builder::default()
        .commands(&[
            "call_plugin",
            "open_widget",
            "log",
            "list_logs",
            "read_log",
            "clear_logs",
        ])
        .events(&["ShowToastEvent"])
        .build();
}
