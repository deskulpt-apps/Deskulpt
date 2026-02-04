fn main() {
    deskulpt_build::Builder::default()
        .commands(&[
            "fetch_registry_index",
            "install",
            "preview",
            "refresh",
            "refresh_all",
            "uninstall",
            "update_settings",
            "upgrade",
        ])
        .events(&["RenderEvent", "UpdateEvent"])
        .build();
}
