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
        .build_arg(format!("-j{}", std::thread::available_parallelism()?.get()))
        .build();

    println!(
        "cargo:rustc-link-search=static={}",
        dst.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=jpeg");

    Ok(())
}
