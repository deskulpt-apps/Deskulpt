fn main() {
    tauri_deskulpt_build::Builder::default()
        .commands(&["update"])
        .events(&["UpdateEvent"])
        .build();
}
