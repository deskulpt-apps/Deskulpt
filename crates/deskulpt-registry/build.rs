fn main() {
    deskulpt_build::Builder::default()
        .commands(&[
            "fetch_widgets",
            "install_widget",
            "upgrade_widget",
            "uninstall_widget",
            "preview_widget",
        ])
        .build();
}
