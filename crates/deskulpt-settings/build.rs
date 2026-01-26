fn main() {
    deskulpt_build::Builder::default()
        .commands(&["read", "update"])
        .events(&["UpdateEvent"])
        .build();
}
