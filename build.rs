use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    has_fpu(&target);

    if target.starts_with("thumbv") {
        fs::copy(
            format!("bin/{}.a", target),
            out_dir.join("libneutron-star-rt.a"),
        )
        .unwrap();
        println!("cargo:rustc-link-lib=static=neutron-star-rt");
    }

    // Put the linker script somewhere the linker can find it
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let link_x = include_bytes!("link.x");
    let mut f = File::create(out.join("link.x")).unwrap();
    f.write_all(link_x).unwrap();

    println!("cargo:rustc-link-search={}", out.display());

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=link.x");
}

fn has_fpu(target: &str) {
    if target.ends_with("eabihf") {
        println!("cargo:rustc-cfg=has_fpu");
    }
}