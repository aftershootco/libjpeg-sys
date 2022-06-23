use std::env;
use std::path::Path;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn main() -> Result<()> {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    // let out_dir = Path::new(&out_dir_);

    #[cfg(feature = "clone")]
    clone(&out_dir)?;
    #[cfg(feature = "build")]
    build(&out_dir)?;

    println!(
        "cargo:include={}",
        concat!(env!("CARGO_MANIFEST_DIR"), "/include")
    );

    Ok(())
}

#[cfg(feature = "clone")]
fn clone(our_dir: impl AsRef<Path>) -> Result<()> {
    use std::process::{Command, Stdio};
    eprintln!("\x1b[31mCloning libjpeg");
    let libjpeg_repo_url = std::env::var("LIBJPEG_REPO")
        .unwrap_or_else(|_| String::from("https://github.com/libjpeg-turbo/libjpeg-turbo"));

    let _git_out = Command::new("git")
        .arg("clone")
        .arg("--depth")
        .arg("1")
        .arg(&libjpeg_repo_url)
        .arg(our_dir.as_ref().join("libjpeg"))
        .stdout(Stdio::inherit())
        .output()?;

    println!(
        "cargo:include={}",
        our_dir.as_ref().join("libjpeg").display()
    );

    Ok(())
}

pub fn build(out_dir: impl AsRef<Path>) -> Result<()> {
    use cmake::Config;

    std::env::set_current_dir(&out_dir)?;
    // std::fs::create_dir_all(out_dir.as_ref().join("build"))?;

    let libjpeg = Config::new("libjpeg")
        .generator("Unix Makefiles")
        .define("ENABLE_SHARED", "OFF")
        .define("ENABLE_STATIC", "ON")
        .define("WITH_JPEG8", "ON")
        .define("WITH_JPEG7", "ON")
        .build();

    println!(
        "cargo:rustc-link-search=native={}",
        libjpeg.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=jpeg");

    Ok(())
}
