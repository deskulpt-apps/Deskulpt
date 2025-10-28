fn main() {
    deskulpt_build::Builder::default()
        .commands(&["bundle", "rescan"])
        .events(&["UpdateEvent"])
        .build();
}
