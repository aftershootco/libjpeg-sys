use anyhow::Result;
use cmake::Config;
use std::{env, path::PathBuf};
use tap::prelude::*;

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

pub fn main() -> Result<()> {
    let os = std::env::var("CARGO_CFG_TARGET_OS").expect("CARGO_CFG_TARGET_OS not set");

    if os == "emscripten" {
        println!("cargo:rustc-link-arg=--no-entry");
    }
    let vendor = PathBuf::from(MANIFEST_DIR).join("vendor");
    anyhow::ensure!(
        vendor.exists(),
        "`{vendor:?}` folder not found, please run `git submodule update --init`"
    );
    let dst = Config::new(vendor)
        // OFF
        .define("WITH_JAVA", "OFF")
        .pipe(|x| {
            if os == "emscripten" {
                x.define("WITH_SIMD", "OFF")
            } else {
                x
            }
        })
        .define("ENABLE_SHARED", "OFF")
        .define("WITH_TURBOJPEG", "OFF")
        // ON
        .define("WITH_JPEG8", "ON")
        .define("ENABLE_STATIC", "ON")
        // .build_arg(format!("-j{}", std::thread::available_parallelism()?.get()))
        .build();

    let lib_folder = if cfg!(target_os = "linux")
        && std::env::var("CARGO_CFG_TARGET_POINTER_WIDTH").expect("CARGO_CFG_TARGET_FAMILY") == "64"
    {
        "lib64"
    } else {
        "lib"
    };
    println!("cargo:rustc-link-search={}", dst.join(lib_folder).display());
    let name = if cfg!(unix) { "jpeg" } else { "jpeg-static" };

    println!("cargo:rustc-link-lib=static={name}");
    println!("cargo:include={}", dst.join("include").display());
    println!("cargo:name={name}");
    Ok(())
}
