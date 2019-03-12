extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/device_api/esa/esa.c")
        .flag_if_supported("-std=gnu99")
        .compile("esa_c");
}
