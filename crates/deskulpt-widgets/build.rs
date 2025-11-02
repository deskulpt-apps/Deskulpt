fn main() {
    deskulpt_build::Builder::default()
        .commands(&["complete_setup", "refresh", "refresh_all"])
        .events(&["RenderEvent", "UpdateEvent"])
        .build();
}
