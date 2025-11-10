fn main() {
    deskulpt_build::Builder::default()
        .commands(&["update"])
        .events(&["UpdateEvent"])
        .build();
}
