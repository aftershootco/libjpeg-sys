use std::env;
use std::path::Path;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn main() -> Result<()> {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    #[cfg(all(feature = "clone", not(feature = "no-build")))]
    clone(&out_dir)?;

    #[cfg(all(feature = "build", not(feature = "no-build")))]
    build(&out_dir)?;

    #[cfg(not(feature = "no-build"))]
    println!(
        "cargo:include={}",
        concat!(env!("CARGO_MANIFEST_DIR"), "/include")
    );

    Ok(())
}

#[cfg(all(feature = "clone", not(feature = "no-build")))]
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

#[cfg(all(feature = "build", not(feature = "no-build")))]
pub fn build(out_dir: impl AsRef<Path>) -> Result<()> {
    use cmake::Config;

    std::env::set_current_dir(&out_dir)?;
    // std::fs::create_dir_all(out_dir.as_ref().join("build"))?;

    let libjpeg = Config::new("libjpeg")
        .generator("Unix Makefiles")
        .define("ENABLE_SHARED", "OFF")
        .define("ENABLE_STATIC", "ON")
        .define("WITH_TURBOJPEG", "OFF")
        .define("WITH_JPEG8", "ON")
        .define("WITH_JPEG7", "ON")
        .build();

    // let mut libjpeg = cc::Build::new();
    // libjpeg.include("libjpeg");
    // libjpeg.file("libjpeg/cdjpeg.c");
    // libjpeg.file("libjpeg/cjpeg.c");
    // libjpeg.file("libjpeg/djpeg.c");
    // libjpeg.file("libjpeg/jaricom.c");
    // libjpeg.file("libjpeg/jcapimin.c");
    // libjpeg.file("libjpeg/jcapistd.c");
    // libjpeg.file("libjpeg/jcarith.c");
    // libjpeg.file("libjpeg/jccoefct.c");
    // libjpeg.file("libjpeg/jccolext.c");
    // libjpeg.file("libjpeg/jccolor.c");
    // libjpeg.file("libjpeg/jcdctmgr.c");
    // libjpeg.file("libjpeg/jchuff.c");
    // libjpeg.file("libjpeg/jcicc.c");
    // libjpeg.file("libjpeg/jcinit.c");
    // libjpeg.file("libjpeg/jcmainct.c");
    // libjpeg.file("libjpeg/jcmarker.c");
    // libjpeg.file("libjpeg/jcmaster.c");
    // libjpeg.file("libjpeg/jcomapi.c");
    // libjpeg.file("libjpeg/jcparam.c");
    // libjpeg.file("libjpeg/jcphuff.c");
    // libjpeg.file("libjpeg/jcprepct.c");
    // libjpeg.file("libjpeg/jcsample.c");
    // libjpeg.file("libjpeg/jcstest.c");
    // libjpeg.file("libjpeg/jctrans.c");
    // libjpeg.file("libjpeg/jdapimin.c");
    // libjpeg.file("libjpeg/jdapistd.c");
    // libjpeg.file("libjpeg/jdarith.c");
    // libjpeg.file("libjpeg/jdatadst-tj.c");
    // libjpeg.file("libjpeg/jdatadst.c");
    // libjpeg.file("libjpeg/jdatasrc-tj.c");
    // libjpeg.file("libjpeg/jdatasrc.c");
    // libjpeg.file("libjpeg/jdcoefct.c");
    // libjpeg.file("libjpeg/jdcol565.c");
    // libjpeg.file("libjpeg/jdcolext.c");
    // libjpeg.file("libjpeg/jdcolor.c");
    // libjpeg.file("libjpeg/jddctmgr.c");
    // libjpeg.file("libjpeg/jdhuff.c");
    // libjpeg.file("libjpeg/jdicc.c");
    // libjpeg.file("libjpeg/jdinput.c");
    // libjpeg.file("libjpeg/jdmainct.c");
    // libjpeg.file("libjpeg/jdmarker.c");
    // libjpeg.file("libjpeg/jdmaster.c");
    // libjpeg.file("libjpeg/jdmerge.c");
    // libjpeg.file("libjpeg/jdmrg565.c");
    // libjpeg.file("libjpeg/jdmrgext.c");
    // libjpeg.file("libjpeg/jdphuff.c");
    // libjpeg.file("libjpeg/jdpostct.c");
    // libjpeg.file("libjpeg/jdsample.c");
    // libjpeg.file("libjpeg/jdtrans.c");
    // libjpeg.file("libjpeg/jerror.c");
    // libjpeg.file("libjpeg/jfdctflt.c");
    // libjpeg.file("libjpeg/jfdctfst.c");
    // libjpeg.file("libjpeg/jfdctint.c");
    // libjpeg.file("libjpeg/jidctflt.c");
    // libjpeg.file("libjpeg/jidctfst.c");
    // libjpeg.file("libjpeg/jidctint.c");
    // libjpeg.file("libjpeg/jidctred.c");
    // libjpeg.file("libjpeg/jmemmgr.c");
    // libjpeg.file("libjpeg/jmemnobs.c");
    // libjpeg.file("libjpeg/jpegtran.c");
    // libjpeg.file("libjpeg/jquant1.c");
    // libjpeg.file("libjpeg/jquant2.c");
    // libjpeg.file("libjpeg/jsimd_none.c");
    // libjpeg.file("libjpeg/jstdhuff.c");
    // libjpeg.file("libjpeg/jutils.c");
    // libjpeg.file("libjpeg/rdbmp.c");
    // libjpeg.file("libjpeg/rdcolmap.c");
    // libjpeg.file("libjpeg/rdgif.c");
    // libjpeg.file("libjpeg/rdjpgcom.c");
    // libjpeg.file("libjpeg/rdppm.c");
    // libjpeg.file("libjpeg/rdswitch.c");
    // libjpeg.file("libjpeg/rdtarga.c");
    // libjpeg.file("libjpeg/strtest.c");
    // libjpeg.file("libjpeg/tjbench.c");
    // libjpeg.file("libjpeg/tjexample.c");
    // libjpeg.file("libjpeg/tjunittest.c");
    // libjpeg.file("libjpeg/tjutil.c");
    // libjpeg.file("libjpeg/transupp.c");
    // libjpeg.file("libjpeg/turbojpeg-jni.c");
    // libjpeg.file("libjpeg/turbojpeg.c");
    // libjpeg.file("libjpeg/wrbmp.c");
    // libjpeg.file("libjpeg/wrgif.c");
    // libjpeg.file("libjpeg/wrjpgcom.c");
    // libjpeg.file("libjpeg/wrppm.c");
    // libjpeg.file("libjpeg/wrtarga.c");
    // libjpeg.compile("jpeg");
    // println!(
    //     "cargo:rustc-link-search=native={}",
    //     out_dir.as_ref().join("lib").display()
    // );

    println!(
        "cargo:rustc-link-search=native={}",
        libjpeg.join("libjpeg").display()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        libjpeg.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=jpeg");

    Ok(())
}
