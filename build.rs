//! Builds the start-up assembly code using cc-rs

fn main() {
    println!("cargo::rustc-link-arg=-Tlinker.x");
    println!("cargo::rerun-if-changed=linker.x");
}
