use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    // Build boot.S
    Command::new("riscv64-unknown-elf-gcc")
        .args(&["-c", "boot.S", "-o"])
        .arg(&format!("{}/boot.o", out_dir))
        .status()
        .unwrap();

    // Create static library
    Command::new("riscv64-unknown-elf-ar")
        .args(&["crus", "libboot.a", "boot.o"])
        .current_dir(&Path::new(&out_dir))
        .status()
        .unwrap();

    // Tell cargo where to find the library
    println!("cargo:rustc-link-search={}", out_dir);
    println!("cargo:rustc-link-lib=static=boot");

    // Tell cargo to invalidate the built crate whenever the linker script changes
    println!("cargo:rerun-if-changed=boot.S");
    println!("cargo:rerun-if-changed=kernel.ld");
}
