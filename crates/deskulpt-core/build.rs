fn main() {
    deskulpt_build::Builder::default()
        .commands(&[
            "call_plugin",
            "open_widget",
            "log",
            "read_log",
            "clear_logs",
        ])
        .events(&["ShowToastEvent"])
        .build();
}
