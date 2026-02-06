fn main() {
    tauri_deskulpt_build::Builder::default()
        .commands(&["clear", "read", "log"])
        .build();
}
