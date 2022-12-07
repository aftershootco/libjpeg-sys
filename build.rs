use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;
pub type Error = Box<dyn std::error::Error>;
pub type Result<T, E = Error> = std::result::Result<T, E>;

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
    env::set_current_dir(&out_dir)?;

    let mut libjpeg = cc::Build::new();

    libjpeg.include("configured");
    let mut libjpeg = configure_jconfigint(libjpeg)?;

    libjpeg.include("libjpeg");
    libjpeg.file("libjpeg/cdjpeg.c");
    libjpeg.file("libjpeg/cjpeg.c");
    libjpeg.file("libjpeg/djpeg.c");
    libjpeg.file("libjpeg/jaricom.c");
    libjpeg.file("libjpeg/jcapimin.c");
    libjpeg.file("libjpeg/jcapistd.c");
    libjpeg.file("libjpeg/jcarith.c");
    libjpeg.file("libjpeg/jccoefct.c");
    libjpeg.file("libjpeg/jccolext.c");
    libjpeg.file("libjpeg/jccolor.c");
    libjpeg.file("libjpeg/jcdctmgr.c");
    libjpeg.file("libjpeg/jchuff.c");
    libjpeg.file("libjpeg/jcicc.c");
    libjpeg.file("libjpeg/jcinit.c");
    libjpeg.file("libjpeg/jcmainct.c");
    libjpeg.file("libjpeg/jcmarker.c");
    libjpeg.file("libjpeg/jcmaster.c");
    libjpeg.file("libjpeg/jcomapi.c");
    libjpeg.file("libjpeg/jcparam.c");
    libjpeg.file("libjpeg/jcphuff.c");
    libjpeg.file("libjpeg/jcprepct.c");
    libjpeg.file("libjpeg/jcsample.c");
    libjpeg.file("libjpeg/jcstest.c");
    libjpeg.file("libjpeg/jctrans.c");
    libjpeg.file("libjpeg/jdapimin.c");
    libjpeg.file("libjpeg/jdapistd.c");
    libjpeg.file("libjpeg/jdarith.c");
    libjpeg.file("libjpeg/jdatadst-tj.c");
    libjpeg.file("libjpeg/jdatadst.c");
    libjpeg.file("libjpeg/jdatasrc-tj.c");
    libjpeg.file("libjpeg/jdatasrc.c");
    libjpeg.file("libjpeg/jdcoefct.c");
    libjpeg.file("libjpeg/jdcol565.c");
    libjpeg.file("libjpeg/jdcolext.c");
    libjpeg.file("libjpeg/jdcolor.c");
    libjpeg.file("libjpeg/jddctmgr.c");
    libjpeg.file("libjpeg/jdhuff.c");
    libjpeg.file("libjpeg/jdicc.c");
    libjpeg.file("libjpeg/jdinput.c");
    libjpeg.file("libjpeg/jdmainct.c");
    libjpeg.file("libjpeg/jdmarker.c");
    libjpeg.file("libjpeg/jdmaster.c");
    libjpeg.file("libjpeg/jdmerge.c");
    libjpeg.file("libjpeg/jdmrg565.c");
    libjpeg.file("libjpeg/jdmrgext.c");
    libjpeg.file("libjpeg/jdphuff.c");
    libjpeg.file("libjpeg/jdpostct.c");
    libjpeg.file("libjpeg/jdsample.c");
    libjpeg.file("libjpeg/jdtrans.c");
    libjpeg.file("libjpeg/jerror.c");
    libjpeg.file("libjpeg/jfdctflt.c");
    libjpeg.file("libjpeg/jfdctfst.c");
    libjpeg.file("libjpeg/jfdctint.c");
    libjpeg.file("libjpeg/jidctflt.c");
    libjpeg.file("libjpeg/jidctfst.c");
    libjpeg.file("libjpeg/jidctint.c");
    libjpeg.file("libjpeg/jidctred.c");
    libjpeg.file("libjpeg/jmemmgr.c");
    libjpeg.file("libjpeg/jmemnobs.c");
    libjpeg.file("libjpeg/jpegtran.c");
    libjpeg.file("libjpeg/jquant1.c");
    libjpeg.file("libjpeg/jquant2.c");
    libjpeg.file("libjpeg/jsimd_none.c");
    libjpeg.file("libjpeg/jstdhuff.c");
    libjpeg.file("libjpeg/jutils.c");
    libjpeg.file("libjpeg/rdbmp.c");
    libjpeg.file("libjpeg/rdcolmap.c");
    libjpeg.file("libjpeg/rdgif.c");
    libjpeg.file("libjpeg/rdjpgcom.c");
    libjpeg.file("libjpeg/rdppm.c");
    libjpeg.file("libjpeg/rdswitch.c");
    libjpeg.file("libjpeg/rdtarga.c");
    libjpeg.file("libjpeg/strtest.c");
    libjpeg.file("libjpeg/tjbench.c");
    libjpeg.file("libjpeg/tjexample.c");
    libjpeg.file("libjpeg/tjunittest.c");
    libjpeg.file("libjpeg/tjutil.c");
    libjpeg.file("libjpeg/transupp.c");
    libjpeg.file("libjpeg/turbojpeg-jni.c");
    libjpeg.file("libjpeg/turbojpeg.c");
    libjpeg.file("libjpeg/wrbmp.c");
    libjpeg.file("libjpeg/wrgif.c");
    libjpeg.file("libjpeg/wrjpgcom.c");
    libjpeg.file("libjpeg/wrppm.c");
    libjpeg.file("libjpeg/wrtarga.c");
    libjpeg.compile("jpeg");
    println!(
        "cargo:rustc-link-search=native={}",
        out_dir.as_ref().join("lib").display()
    );

    // println!(
    //     "cargo:rustc-link-search=native={}",
    //     libjpeg.join("lib").display()
    // );
    println!("cargo:rustc-link-lib=static=jpeg");

    Ok(())
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum JpegLib {
    Jpeg8,
    Jpeg7,
    Other,
}

impl FromStr for JpegLib {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "jpeg8" => Ok(JpegLib::Jpeg8),
            "jpeg7" => Ok(JpegLib::Jpeg7),
            "jpeg6" => Ok(JpegLib::Jpeg6),
            _ => Err(format!("Unknown jpeg lib: {}", s)),
        }
    }
}

fn version() -> Result<String> {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let file = BufReader::new(File::open(
        std::path::PathBuf::from(out_dir)
            .join("libjpeg")
            .join("CMakeLists.txt"),
    )?);
    for line in file.lines().flatten() {
        if line.starts_with("set(VERSION") {
            return line
                .strip_prefix("set(VERSION ")
                .and_then(|s| s.strip_suffix(')'))
                .map(|s| s.to_string())
                .ok_or_else(|| "Unable to get version from CMakeLists.txt".into());
        }
    }
    Err("Unable to get version from CMakeLists.txt".into())
}
fn jpeg_lib_version() -> Result<u32> {
    let jpeg8 = env("WITH_JPEG8", 0_u8);
    let jpeg7 = env("WITH_JPEG7", 0_u8);

    if jpeg8 == 1 {
        Ok(80)
    } else if jpeg7 == 1 {
        Ok(70)
    } else {
        Ok(62)
    }
}

fn jversion(mut libjpeg: cc::Build) -> Result<cc::Build> {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let jversion = std::path::PathBuf::from(out_dir)
        .join("libjpeg")
        .join("jversion.h");


    Ok(libjpeg)
}

fn configure_jconfigint(mut libjpeg: cc::Build) -> Result<cc::Build> {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let jconfigint = std::path::PathBuf::from(out_dir)
        .join("libjpeg")
        .join("jconfigint.h");
    let ctzl = try_build_c(
        "int main(int argc, char **argv) { unsigned long a = argc;  return __builtin_ctzl(a); }",
    )?;
    if ctzl {
        libjpeg.define("HAVE_BUILTIN_CTZL", None);
    }
    let compiler = libjpeg.get_compiler();
    let inline = inline(&compiler, env("FORCE_INLINE", true))?;
    libjpeg.define("INLINE", inline);
    libjpeg.define(
        "SIZEOF_SIZE_T",
        itoa::Buffer::new().format(core::mem::size_of::<usize>()),
    );
    libjpeg.define("PACKAGE_NAME", env!("CARGO_PKG_NAME"));
    libjpeg.define("VERSION", version()?.as_str());
    let have_intin_h = try_build_c("#include <intrin.h>")?;
    if compiler.is_like_msvc() && have_intin_h {
        if core::mem::size_of::<usize>() == 8 {
            libjpeg.define("HAVE__BITSCANFORWARD", None);
        } else {
            libjpeg.define("HAVE__BITSCANFORWARD64", None);
        }
    }
    let fallthrough = try_expand_c(
        r##"
#if defined(__has_attribute)
#if __has_attribute(fallthrough)
#define FALLTHROUGH  __attribute__((fallthrough));
#else
#define FALLTHROUGH
#endif
#else
#define FALLTHROUGH
#endif
"##,
    )?;
    std::fs::write(jconfigint, fallthrough.as_bytes())?;

    Ok(libjpeg)
}

fn inline(compiler: &cc::Tool, force_inline: bool) -> Result<&'static str> {
    let mut inline = if compiler.is_like_msvc() {
        vec!["__inline;inline"]
    } else {
        vec!["__inline__;inline"]
    };
    if force_inline {
        if compiler.is_like_msvc() {
            inline.insert(0, "__forceinline");
        } else {
            inline.insert(0, "inline __attribute__((always_inline))");
            inline.insert(0, "__inline__ __attribute__((always_inline))");
        }
    }

    Ok(inline
        .iter()
        .map(|i| {
            let code = format!(
                "{i} static int foo(void) {{ return 0; }} int main(void) {{ return foo(); }}"
            );
            (i, try_build_c(&code))
        })
        // .flatten()
        .find_map(|(code, compiled)| {
            if let Ok(true) = compiled {
                Some(code)
            } else {
                None
            }
        })
        .ok_or_else(|| -> Box<dyn std::error::Error> { "failed to find inline".into() })?)
}

fn try_build_c(c: &str) -> Result<bool> {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let pwd = env::current_dir()?;
    env::set_current_dir(&out_dir)?;
    let mut config = cc::Build::new();
    std::fs::write("test.c", c)?;
    config.file("test.c");
    env::set_current_dir(&pwd)?;
    Ok(config.try_compile("test").is_ok())
}

fn try_expand_c(c: &str) -> Result<String> {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let pwd = env::current_dir()?;
    env::set_current_dir(&out_dir)?;
    let mut config = cc::Build::new();
    std::fs::write("test.c", c)?;
    config.file("test.c");
    env::set_current_dir(&pwd)?;
    Ok(String::from_utf8(config.try_expand()?)?)
}

fn env<T: std::str::FromStr>(var: &'static str, default: T) -> T {
    std::env::var(var)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}
