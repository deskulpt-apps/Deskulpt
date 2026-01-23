fn main() {
    deskulpt_build::Builder::default()
        .commands(&[
            "fetch_registry_index",
            "install",
            "preview",
            "refresh",
            "refresh_all",
            "uninstall",
            "upgrade",
        ])
        .events(&["RenderEvent", "UpdateEvent"])
        .build();
}
