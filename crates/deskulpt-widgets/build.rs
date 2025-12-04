fn main() {
    deskulpt_build::Builder::default()
        .commands(&[
            "complete_setup",
            "fetch_registry_index",
            "install",
            "refresh",
            "refresh_all",
            "uninstall",
            "upgrade",
        ])
        .events(&["RenderEvent", "UpdateEvent"])
        .build();
}
