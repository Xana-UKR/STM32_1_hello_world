use std::{env, fs, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from(
        env::var("OUT_DIR").expect("Cargo must provide OUT_DIR"),
    );

    fs::copy("memory.x", out_dir.join("memory.x"))
        .expect("Could not copy memory.x");

    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rerun-if-changed=memory.x");
}