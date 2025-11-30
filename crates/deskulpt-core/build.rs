fn main() {
    deskulpt_build::Builder::default()
        .commands(&["call_plugin", "clear_logs", "fetch_logs", "log", "open"])
        .events(&["ShowToastEvent"])
        .build();
}
