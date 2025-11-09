fn main() {
    deskulpt_build::Builder::default()
        .commands(&["call_plugin", "open_widget"])
        .events(&["ShowToastEvent"])
        .build();
}
