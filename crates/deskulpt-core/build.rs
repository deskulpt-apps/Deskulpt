fn main() {
    deskulpt_build::Builder::default()
        .commands(&["call_plugin", "open"])
        .events(&["ShowToastEvent"])
        .build();
}
