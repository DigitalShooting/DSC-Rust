extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/device_api/esa.c")
        .compile("esa_c");
}
