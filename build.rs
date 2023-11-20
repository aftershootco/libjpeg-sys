use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;
pub type Error = Box<dyn std::error::Error>;
pub type Result<T, E = Error> = std::result::Result<T, E>;

static MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

pub fn main() -> Result<()> {
    let out_dir = env::var("OUT_DIR")?;

    #[cfg(all(feature = "build", not(feature = "no-build")))]
    build(&out_dir)?;

    Ok(())
}

#[cfg(all(feature = "build", not(feature = "no-build")))]
pub fn build(out_dir: impl AsRef<Path>) -> Result<()> {
    use std::path::PathBuf;

    env::set_current_dir(&out_dir)?;

    let mut libjpeg = cc::Build::new();

    std::fs::create_dir_all(out_dir.as_ref().join("configured"))
        .expect("Failed to create configured dir");
    compile::configure_jconfigint().expect("Configure jconfigint.h");
    compile::configure_jversion().expect("Configure jversion.h");
    compile::configure_jconfig().expect("Configure jconfig.h");
    libjpeg.include(out_dir.as_ref().join("configured"));
    libjpeg.include(std::path::PathBuf::from(MANIFEST_DIR).join("vendor"));

    // libjpeg.file("libjpeg/jpeglib.h");
    // std::fs::write("/tmp/file.txt", libjpeg.expand())?;
    let sources = [
        "vendor/jcapimin.c",
        "vendor/jcapistd.c",
        "vendor/jccoefct.c",
        "vendor/jccolor.c",
        "vendor/jcdctmgr.c",
        "vendor/jchuff.c",
        "vendor/jcicc.c",
        "vendor/jcinit.c",
        "vendor/jcmainct.c",
        "vendor/jcmarker.c",
        "vendor/jcmaster.c",
        "vendor/jcomapi.c",
        "vendor/jcparam.c",
        "vendor/jcphuff.c",
        "vendor/jcprepct.c",
        "vendor/jcsample.c",
        "vendor/jctrans.c",
        "vendor/jdapimin.c",
        "vendor/jdapistd.c",
        "vendor/jdatadst.c",
        "vendor/jdatasrc.c",
        "vendor/jdcoefct.c",
        "vendor/jdcolor.c",
        "vendor/jddctmgr.c",
        "vendor/jdhuff.c",
        "vendor/jdicc.c",
        "vendor/jdinput.c",
        "vendor/jdmainct.c",
        "vendor/jdmarker.c",
        "vendor/jdmaster.c",
        "vendor/jdmerge.c",
        "vendor/jdphuff.c",
        "vendor/jdpostct.c",
        "vendor/jdsample.c",
        "vendor/jdtrans.c",
        "vendor/jerror.c",
        "vendor/jfdctflt.c",
        "vendor/jfdctfst.c",
        "vendor/jfdctint.c",
        "vendor/jidctflt.c",
        "vendor/jidctfst.c",
        "vendor/jidctint.c",
        "vendor/jidctred.c",
        "vendor/jquant1.c",
        "vendor/jquant2.c",
        "vendor/jutils.c",
        "vendor/jmemmgr.c",
        "vendor/jmemnobs.c",
    ];

    let sources = sources
        .into_iter()
        .map(|p| std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(p));
    libjpeg.files(sources);
    // libjpeg.file("libjpeg/jsimd.c");

    #[cfg(feature = "simd")]
    let simd = compile::simd::simd()?;

    println!(
        "cargo:include={}",
        env::join_paths([
            out_dir.as_ref().join("configured"),
            PathBuf::from(MANIFEST_DIR).join("vendor")
        ])?
        .to_string_lossy()
    );

    // libjpeg.flag(&format!("-L{}", simd.0.to_string_lossy()));
    // libjpeg.flag(&format!("-l{}", simd.1));
    println!(
        "cargo:rustc-link-search=native={}",
        simd.0.to_string_lossy()
    );
    println!("cargo:rustc-link-lib=static={}", simd.1);
    libjpeg.flag("-Wno-unused-parameter");
    libjpeg.flag("-Wno-unused-variable");
    libjpeg.flag("-Wno-shift-negative-value");

    // #[cfg(feature = "simd")]
    libjpeg.define("WITH_SIMD", None);

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

#[cfg(all(feature = "build", not(feature = "no-build")))]
mod compile {
    use std::io::Write;
    use std::path::PathBuf;

    pub use super::*;
    #[derive(Clone, Copy, Eq, PartialEq, Debug)]
    pub enum JpegLib {
        Jpeg8,
        Jpeg7,
    }

    impl FromStr for JpegLib {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "jpeg8" => Ok(JpegLib::Jpeg8),
                "jpeg7" => Ok(JpegLib::Jpeg7),
                _ => Err(format!("Unknown jpeg lib: {}", s)),
            }
        }
    }

    pub fn version() -> Result<String> {
        let file = BufReader::new(File::open(
            std::path::PathBuf::from(MANIFEST_DIR)
                .join("vendor")
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

    fn turbo_version() -> Result<(u32, u32, u32)> {
        let v = version()?;
        Ok(v.split('.')
            .map(|s| s.parse::<u32>())
            .collect::<Result<Vec<_>, _>>()
            .map(|v| (v[0], v[1], v[2]))?)
    }

    fn jpeg_lib_version() -> Result<u32> {
        if cfg!(feature = "jpeg80_abi") {
            Ok(80)
        } else if cfg!(feature = "jpeg70_abi") {
            Ok(70)
        } else {
            Ok(62)
        }
    }

    pub fn configure_jversion() -> Result<()> {
        let mut jversion = sorse::SorseHeader::new(
            PathBuf::from(env::var("OUT_DIR")?)
                .join("configured")
                .join("jversion.h"),
        );

        let jpeg_lib_version = jpeg_lib_version()?;
        match jpeg_lib_version {
            x if x >= 80 => {
                jversion.define("JVERSION", "\"8d  15-Jan-2012\"");
            }
            x if x >= 70 => {
                jversion.define("JVERSION", "\"7  27-Jun-2009\"");
            }
            _ => {
                jversion.define("JVERSION", "\"6b  27-Mar-1998\"");
            }
        }
        // #define JCOPYRIGHT \
        //   "Copyright (C) 2009-2022 D. R. Commander\n" \
        //   "Copyright (C) 2015, 2020 Google, Inc.\n" \
        //   "Copyright (C) 2019-2020 Arm Limited\n" \
        //   "Copyright (C) 2015-2016, 2018 Matthieu Darbois\n" \
        //   "Copyright (C) 2011-2016 Siarhei Siamashka\n" \
        //   "Copyright (C) 2015 Intel Corporation\n" \
        //   "Copyright (C) 2013-2014 Linaro Limited\n" \
        //   "Copyright (C) 2013-2014 MIPS Technologies, Inc.\n" \
        //   "Copyright (C) 2009, 2012 Pierre Ossman for Cendio AB\n" \
        //   "Copyright (C) 2009-2011 Nokia Corporation and/or its subsidiary(-ies)\n" \
        //   "Copyright (C) 1999-2006 MIYASAKA Masaru\n" \
        //   "Copyright (C) 1991-2020 Thomas G. Lane, Guido Vollbeding"

        jversion.define(
            "JCOPYRIGHT \\",
            r#"
"Copyright (C) 2009-2022 D. R. Commander\n" \
"Copyright (C) 2015, 2020 Google, Inc.\n" \
"Copyright (C) 2019-2020 Arm Limited\n" \
"Copyright (C) 2015-2016, 2018 Matthieu Darbois\n" \
"Copyright (C) 2011-2016 Siarhei Siamashka\n" \
"Copyright (C) 2015 Intel Corporation\n" \
"Copyright (C) 2013-2014 Linaro Limited\n" \
"Copyright (C) 2013-2014 MIPS Technologies, Inc.\n" \
"Copyright (C) 2009, 2012 Pierre Ossman for Cendio AB\n" \
"Copyright (C) 2009-2011 Nokia Corporation and/or its subsidiary(-ies)\n" \
"Copyright (C) 1999-2006 MIYASAKA Masaru\n" \
"Copyright (C) 1991-2020 Thomas G. Lane, Guido Vollbeding"
"#,
        );

        jversion.define(
            "JCOPYRIGHT_SHORT",
            r#""Copyright (C) 1991-2020 The libjpeg-turbo Project and many others""#,
        );

        jversion.write()?;
        Ok(())
    }

    pub fn configure_jconfigint() -> Result<()> {
        let mut jconfigint = sorse::SorseHeader::new(
            PathBuf::from(env::var("OUT_DIR")?)
                .join("configured")
                .join("jconfigint.h"),
        );
        let ctzl = try_build_c("int main(int argc, char **argv) { unsigned long a = argc;  return __builtin_ctzl(a); }")?;
        if ctzl {
            jconfigint.define("HAVE_BUILTIN_CTZL", None);
        }

        let compiler = cc::Build::new().get_compiler();
        jconfigint.write_all(b"#undef inline")?;
        let inline = inline(&compiler, env("FORCE_INLINE", true))?;
        jconfigint.define("INLINE", inline);
        jconfigint.define(
            "SIZEOF_SIZE_T",
            itoa::Buffer::new().format(core::mem::size_of::<usize>()),
        );

        let thread_local = if compiler.is_like_msvc() {
            "__declspec(thread)"
        } else {
            "__thread"
        };
        jconfigint.define("THREAD_LOCAL", thread_local);
        if try_build_c(&format!(
            "${thread_local} int i;  int main(void) {{ i = 0;  return i; }}"
        ))
        .is_ok()
        {
            println!("cargo:info=THREAD_LOCAL={}", thread_local);
        } else {
            println!("cargo:info=Thread-local storage is not available.  The TurboJPEG API library's global error handler will not be thread-safe.");
            jconfigint.undef("THREAD_LOCAL");
        }

        jconfigint.define("PACKAGE_NAME", concat!("\"", env!("CARGO_PKG_NAME"), "\""));
        jconfigint.define("VERSION", format!("\"{}\"", version()?).as_str());
        jconfigint.define("BUILD", format!("\"{}\"", "HELLO").as_str());
        let have_intin_h = try_build_c("#include <intrin.h>")?;
        if have_intin_h {
            jconfigint.define("HAVE_INTRIN_H", None);
        }
        if compiler.is_like_msvc() && have_intin_h {
            if core::mem::size_of::<usize>() == 8 {
                jconfigint.define("HAVE__BITSCANFORWARD", None);
            } else {
                jconfigint.define("HAVE__BITSCANFORWARD64", None);
            }
        }
        let fallthrough = r##"
#if defined(__has_attribute)
#if __has_attribute(fallthrough)
#define FALLTHROUGH  __attribute__((fallthrough));
#else
#define FALLTHROUGH
#endif
#else
#define FALLTHROUGH
#endif
"##;
        use std::io::Write;
        jconfigint.write_all(fallthrough.as_bytes())?;
        jconfigint.write()?;

        Ok(())
    }

    pub fn configure_jconfig() -> Result<()> {
        let mut jconfig = sorse::SorseHeader::new(
            PathBuf::from(env::var("OUT_DIR")?)
                .join("configured")
                .join("jconfig.h"),
        );

        let jpeg_lib_version = jpeg_lib_version()?;
        jconfig.define(
            "JPEG_LIB_VERSION",
            itoa::Buffer::new().format(jpeg_lib_version),
        );

        jconfig.define("LIBJPEG_TURBO_VERSION", version()?.as_str());
        let turbo_version = turbo_version()?;

        jconfig.define(
            "LIBJPEG_TURBO_VERSION_NUMBER",
            format!(
                "\"{}{:0>3}{:0>3}\"",
                turbo_version.0, turbo_version.1, turbo_version.2
            )
            .as_str(),
        );

        #[cfg(feature = "arith_enc")]
        jconfig.define("C_ARITH_CODING_SUPPORTED", "1");

        #[cfg(feature = "arith_dec")]
        jconfig.define("D_ARITH_CODING_SUPPORTED", "1");

        jconfig.define("MEM_SRCDST_SUPPORTED", "1");

        if env::var("CARGO_CFG_TARGET_OS")? == "windows" {
            jconfig.write_all(br#"
                #undef RIGHT_SHIFT_IS_UNSIGNED

                /* Define "boolean" as unsigned char, not int, per Windows custom */
                #ifndef __RPCNDR_H__            /* don't conflict if rpcndr.h already read */
                typedef unsigned char boolean;
                #endif
                #define HAVE_BOOLEAN            /* prevent jmorecfg.h from redefining it */

                /* Define "INT32" as int, not long, per Windows custom */
                #if !(defined(_BASETSD_H_) || defined(_BASETSD_H))   /* don't conflict if basetsd.h already read */
                typedef short INT16;
                typedef signed int INT32;
                #endif
                #define XMD_H                   /* prevent jmorecfg.h from redefining it */
            "#)?;
        } else {
            jconfig.define("RIGHT_SHIFT_IS_UNSIGNED", "1");
        }

        jconfig.define("BITS_IN_JSAMPLE", "8");
        jconfig.write()?;
        Ok(())
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
        env::set_current_dir(out_dir)?;
        let mut config = cc::Build::new();
        // config.flag("-Wno-unused-parameter");
        // config.flag("-Wno-unused-variable");
        // config.flag("-Wno-shift-negative-value");
        std::fs::write("test.c", c)?;
        config.file("test.c");
        env::set_current_dir(pwd)?;
        Ok(config.try_compile("test").is_ok())
    }

    // fn try_expand_c(c: &str) -> Result<String> {
    //     let out_dir = env::var_os("OUT_DIR").unwrap();
    //     let pwd = env::current_dir()?;
    //     env::set_current_dir(out_dir)?;
    //     let mut config = cc::Build::new();
    //     std::fs::write("test.c", c)?;
    //     config.file("test.c");
    //     env::set_current_dir(pwd)?;
    //     Ok(String::from_utf8(config.try_expand()?)?)
    // }

    fn env<T: std::str::FromStr>(var: &'static str, default: T) -> T {
        std::env::var(var)
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(default)
    }

    pub mod simd {
        use super::*;
        use std::io::Write;
        use std::path::PathBuf;

        pub fn simd() -> Result<(PathBuf, String)> {
            let target_arch = env::var("CARGO_CFG_TARGET_ARCH")?;
            match target_arch.as_str() {
                "x86_64" => todo!(),
                "i386" => todo!(),
                "aarch64" => simd_neon(),
                _ => Err("Unsupported arch for simd".into()),
            }
        }

        pub fn simd_x86_64() -> Result<()> {
            let simd = PathBuf::from(std::env::var("OUT_DIR")?)
                .join("libjpeg")
                .join("simd");
            env::set_current_dir(simd)?;
            let asm_sources = [
                "x86_64/jsimdcpu.asm",
                "x86_64/jfdctflt-sse.asm",
                "x86_64/jccolor-sse2.asm",
                "x86_64/jcgray-sse2.asm",
                "x86_64/jchuff-sse2.asm",
                "x86_64/jcphuff-sse2.asm",
                "x86_64/jcsample-sse2.asm",
                "x86_64/jdcolor-sse2.asm",
                "x86_64/jdmerge-sse2.asm",
                "x86_64/jdsample-sse2.asm",
                "x86_64/jfdctfst-sse2.asm",
                "x86_64/jfdctint-sse2.asm",
                "x86_64/jidctflt-sse2.asm",
                "x86_64/jidctfst-sse2.asm",
                "x86_64/jidctint-sse2.asm",
                "x86_64/jidctred-sse2.asm",
                "x86_64/jquantf-sse2.asm",
                "x86_64/jquanti-sse2.asm",
                "x86_64/jccolor-avx2.asm",
                "x86_64/jcgray-avx2.asm",
                "x86_64/jcsample-avx2.asm",
                "x86_64/jdcolor-avx2.asm",
                "x86_64/jdmerge-avx2.asm",
                "x86_64/jdsample-avx2.asm",
                "x86_64/jfdctint-avx2.asm",
                "x86_64/jidctint-avx2.asm",
                "x86_64/jquanti-avx2.asm",
            ];
            let mut simd = cc::Build::new();
            simd.files(asm_sources);
            simd.flag("-E");
            simd.compile("sss");
            Ok(())
        }

        pub fn simd_neon() -> Result<(PathBuf, String)> {
            let mut neon = cc::Build::new();
            neon.flag("-Wno-unused-parameter");
            neon.flag("-Wno-unused-variable");
            neon.flag("-Wno-shift-negative-value");
            neon.include(PathBuf::from(MANIFEST_DIR).join("vendor"));
            neon.include(PathBuf::from(env::var("OUT_DIR")?).join("configured"));

            let mut simd_sources = vec![
                "arm/jcgray-neon.c",
                "arm/jcphuff-neon.c",
                "arm/jcsample-neon.c",
                "arm/jdmerge-neon.c",
                "arm/jdsample-neon.c",
                "arm/jfdctfst-neon.c",
                "arm/jidctred-neon.c",
                "arm/jquanti-neon.c",
            ];
            let neon_intrinsics = true;
            let target_pointer_width = std::env::var("CARGO_CFG_TARGET_POINTER_WIDTH")?;

            if neon_intrinsics {
                simd_sources.extend(["arm/jccolor-neon.c", "arm/jidctint-neon.c"]);
            }

            if neon_intrinsics || target_pointer_width == "64" {
                simd_sources.push("arm/jidctfst-neon.c");
            }

            if neon_intrinsics || target_pointer_width == "32" {
                simd_sources.extend([
                    match target_pointer_width.as_str() {
                        "32" => "arm/aarch32/jchuff-neon.c",
                        "64" => "arm/aarch64/jchuff-neon.c",
                        _ => unreachable!(),
                    },
                    "arm/jdcolor-neon.c",
                    "arm/jfdctint-neon.c",
                ]);
            }

            simd_sources.extend(match target_pointer_width.as_str() {
                "32" => ["arm/aarch32/jsimd_neon.S", "arm/aarch32/jsimd.c"],
                "64" => ["arm/aarch64/jsimd_neon.S", "arm/aarch64/jsimd.c"],
                _ => unreachable!(),
            });

            let sources = simd_sources
                .iter()
                // .chain(aarch64.iter())
                .map(|s| {
                    PathBuf::from(MANIFEST_DIR)
                        .join("vendor")
                        .join("simd")
                        .join(s)
                });

            let have_vld1_s16_x3 = try_build_c(
                r#"#include <arm_neon.h>
                int main(int argc, char **argv) {
                  int16_t input[] = {
                    (int16_t)argc, (int16_t)argc, (int16_t)argc, (int16_t)argc,
                    (int16_t)argc, (int16_t)argc, (int16_t)argc, (int16_t)argc,
                    (int16_t)argc, (int16_t)argc, (int16_t)argc, (int16_t)argc
                  };
                  int16x4x3_t output = vld1_s16_x3(input);
                  vst3_s16(input, output);
                  return (int)input[0];
                }"#,
            )?;

            let have_vld1_u16_x2 = try_build_c(
                r#"
                  #include <arm_neon.h>
                  int main(int argc, char **argv) {
                    uint16_t input[] = {
                      (uint16_t)argc, (uint16_t)argc, (uint16_t)argc, (uint16_t)argc,
                      (uint16_t)argc, (uint16_t)argc, (uint16_t)argc, (uint16_t)argc
                    };
                    uint16x4x2_t output = vld1_u16_x2(input);
                    vst2_u16(input, output);
                    return (int)input[0];
                  }"#,
            )?;
            let have_vld1_q_u8_x4 = try_build_c(
                r#"
                  #include <arm_neon.h>
                  int main(int argc, char **argv) {
                    uint8_t input[] = {
                      (uint8_t)argc, (uint8_t)argc, (uint8_t)argc, (uint8_t)argc,
                      (uint8_t)argc, (uint8_t)argc, (uint8_t)argc, (uint8_t)argc,
                      (uint8_t)argc, (uint8_t)argc, (uint8_t)argc, (uint8_t)argc,
                      (uint8_t)argc, (uint8_t)argc, (uint8_t)argc, (uint8_t)argc,
                      (uint8_t)argc, (uint8_t)argc, (uint8_t)argc, (uint8_t)argc,
                      (uint8_t)argc, (uint8_t)argc, (uint8_t)argc, (uint8_t)argc,
                      (uint8_t)argc, (uint8_t)argc, (uint8_t)argc, (uint8_t)argc,
                      (uint8_t)argc, (uint8_t)argc, (uint8_t)argc, (uint8_t)argc,
                      (uint8_t)argc, (uint8_t)argc, (uint8_t)argc, (uint8_t)argc,
                      (uint8_t)argc, (uint8_t)argc, (uint8_t)argc, (uint8_t)argc,
                      (uint8_t)argc, (uint8_t)argc, (uint8_t)argc, (uint8_t)argc,
                      (uint8_t)argc, (uint8_t)argc, (uint8_t)argc, (uint8_t)argc,
                      (uint8_t)argc, (uint8_t)argc, (uint8_t)argc, (uint8_t)argc,
                      (uint8_t)argc, (uint8_t)argc, (uint8_t)argc, (uint8_t)argc,
                      (uint8_t)argc, (uint8_t)argc, (uint8_t)argc, (uint8_t)argc,
                      (uint8_t)argc, (uint8_t)argc, (uint8_t)argc, (uint8_t)argc
                    };
                    uint8x16x4_t output = vld1q_u8_x4(input);
                    vst4q_u8(input, output);
                    return (int)input[0];
                  }"#,
            )?;

            let mut neon_compat = sorse::SorseHeader::new(
                PathBuf::from(env::var("OUT_DIR")?)
                    .join("configured")
                    .join("neon-compat.h"),
            );

            if have_vld1_s16_x3 {
                neon_compat.define("HAVE_VLD1_S16_X3", None);
            }
            if have_vld1_u16_x2 {
                neon_compat.define("HAVE_VLD1_U16_X2", None);
            }
            if have_vld1_q_u8_x4 {
                neon_compat.define("HAVE_VLD1Q_U8_X4", None);
            }

            // /* Define compiler-independent count-leading-zeros and byte-swap macros */
            neon_compat.write_all(
                r#"
                #if defined(_MSC_VER) && !defined(__clang__)
                #define BUILTIN_CLZ(x)  _CountLeadingZeros(x)
                #define BUILTIN_CLZLL(x)  _CountLeadingZeros64(x)
                #define BUILTIN_BSWAP64(x)  _byteswap_uint64(x)
                #elif defined(__clang__) || defined(__GNUC__)
                #define BUILTIN_CLZ(x)  __builtin_clz(x)
                #define BUILTIN_CLZLL(x)  __builtin_clzll(x)
                #define BUILTIN_BSWAP64(x)  __builtin_bswap64(x)
                #else
                #error "Unknown compiler"
                #endif
            "#
                .as_bytes(),
            )?;

            neon_compat.write()?;

            neon.files(sources);
            neon.define("NEON_INTRINSICS", None);
            neon.define("WITH_SIMD", None);
            neon.compile("neon");
            // Return path to compiled libneon.a
            Ok((PathBuf::from(env::var("OUT_DIR")?), "neon".into()))
        }

        pub fn simd_i386() -> Result<()> {
            let asm_sources = [
                "i386/jsimdcpu.asm",
                "i386/jfdctflt-3dn.asm",
                "i386/jidctflt-3dn.asm",
                "i386/jquant-3dn.asm",
                "i386/jccolor-mmx.asm",
                "i386/jcgray-mmx.asm",
                "i386/jcsample-mmx.asm",
                "i386/jdcolor-mmx.asm",
                "i386/jdmerge-mmx.asm",
                "i386/jdsample-mmx.asm",
                "i386/jfdctfst-mmx.asm",
                "i386/jfdctint-mmx.asm",
                "i386/jidctfst-mmx.asm",
                "i386/jidctint-mmx.asm",
                "i386/jidctred-mmx.asm",
                "i386/jquant-mmx.asm",
                "i386/jfdctflt-sse.asm",
                "i386/jidctflt-sse.asm",
                "i386/jquant-sse.asm",
                "i386/jccolor-sse2.asm",
                "i386/jcgray-sse2.asm",
                "i386/jchuff-sse2.asm",
                "i386/jcphuff-sse2.asm",
                "i386/jcsample-sse2.asm",
                "i386/jdcolor-sse2.asm",
                "i386/jdmerge-sse2.asm",
                "i386/jdsample-sse2.asm",
                "i386/jfdctfst-sse2.asm",
                "i386/jfdctint-sse2.asm",
                "i386/jidctflt-sse2.asm",
                "i386/jidctfst-sse2.asm",
                "i386/jidctint-sse2.asm",
                "i386/jidctred-sse2.asm",
                "i386/jquantf-sse2.asm",
                "i386/jquanti-sse2.asm",
                "i386/jccolor-avx2.asm",
                "i386/jcgray-avx2.asm",
                "i386/jcsample-avx2.asm",
                "i386/jdcolor-avx2.asm",
                "i386/jdmerge-avx2.asm",
                "i386/jdsample-avx2.asm",
                "i386/jfdctint-avx2.asm",
                "i386/jidctint-avx2.asm",
                "i386/jquanti-avx2.asm",
            ];
            Ok(())
        }
    }
}
