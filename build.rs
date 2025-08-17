use std::env;
use std::path::Path;
use std::process::Command;

extern crate napi_build;

macro_rules! join {
    ($start: expr $(, $join: expr)*) => {
        Path::new($start)
        $(.join($join))*
    };
}

fn main() {
    let in_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();

    let status = Command::new("zig")
        .arg("build-lib")
        .arg("-O")
        .arg("ReleaseSmall")
        .arg("-static")
        .arg("-target")
        .arg(match_target(&target_arch, &target_os))
        .arg(format!(
            "-femit-bin={}",
            Path::display(&join!(&out_dir, "libzig-dxt.a"))
        ))
        .arg(dbg!(join!(&in_dir, "zig-lib", "dxt", "lib.zig")))
        .status()
        .expect("Failed to compile Zig code");

    if !status.success() {
        panic!("Zig compilation failed! status -> {}", status);
    }

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=zig-dxt");
    napi_build::setup();
}

fn match_target(arch: &str, os: &str) -> String {
    if arch == "wasm32" {
        return String::from("wasm32-freestanding");
    }

    format!("{}-{}", map_arch(arch), map_os(os))
}

fn map_arch(input: &str) -> &str {
    match input {
        "i686" => "x86",
        "armv7" => "arm",
        _ => input,
    }
}

fn map_os(input: &str) -> &str {
    match input {
        "darwin" => "macos",
        "android" | "androideabi" => "linux",
        _ => input,
    }
}
