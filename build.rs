use anyhow::Result;
use cmake::Config;
use std::{env, path::PathBuf};

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

pub fn main() -> Result<()> {
    // let out_dir = env::var("OUT_DIR")?;
    let dst = Config::new(PathBuf::from(MANIFEST_DIR).join("vendor"))
        // OFF
        .define("WITH_JAVA", "OFF")
        .define("ENABLE_SHARED", "OFF")
        .define("WITH_TURBOJPEG", "OFF")
        // ON
        .define("WITH_JPEG8", "ON")
        .define("ENABLE_STATIC", "ON")
        // .build_arg(format!("-j{}", std::thread::available_parallelism()?.get()))
        .build();

    println!("cargo:rustc-link-search={}", dst.join("lib").display());
    let name = if cfg!(unix) { "jpeg" } else { "jpeg-static" };

    println!("cargo:rustc-link-lib=static={name}");
    println!("cargo:include={}", dst.join("include").display());
    println!("cargo:name={name}");
    Ok(())
}
