extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/device_api/esa/esa.c")
        .flag_if_supported("-std=c17")
        .compile("esa_c");
}
