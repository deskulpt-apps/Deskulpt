fn main() {
    deskulpt_build::Builder::default()
        .commands(&["bundle", "rescan", "complete_setup"])
        .events(&["RenderEvent", "UpdateEvent"])
        .build();
}
