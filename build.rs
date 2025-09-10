//! Builds the start-up assembly code using cc-rs

fn main() {
    println!("cargo::rustc-link-arg=-Tlinker.x");
    println!("cargo::rerun-if-changed=linker.x");
    cc::Build::new()
        .flag("-mcpu=8540")
        .flag("-msoft-float")
        .file("src/c_library.c")
        .compile("c_library");
}
