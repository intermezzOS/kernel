use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    assert!(Command::new("nasm")
                .args(&["src/asm/boot.asm", "-felf64", "-o"])
                .arg(&format!("{}/boot.o", out_dir))
                .status()
                .expect("failed to execute nasm")
                .success(),
            "compilation of boot.asm failed");

    assert!(Command::new("ar")
                .args(&["crus", "libboot.a", "boot.o"])
                .current_dir(&Path::new(&out_dir))
                .status()
                .expect("failed to execute ar")
                .success(),
            "ar command failed");

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=boot");
    println!("cargo:rerun-if-changed=/src/asm/boot.asm");
}
