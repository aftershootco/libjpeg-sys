use anyhow::{Error, Result};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

pub fn main() -> Result<()> {
    let out_dir = env::var("OUT_DIR")?;

    build(&out_dir)?;

    // compile::configure_jconfigint().expect("Configure jconfigint.h");
    // compile::configure_jversion().expect("Configure jversion.h");
    // compile::configure_jconfig().expect("Configure jconfig.h");
    bindgen::builder()
        .header(format!("{MANIFEST_DIR}/vendor/jpeglib.h"))
        .generate()
        .expect("Unable to generate bindings");

    Ok(())
}

pub fn build(out_dir: impl AsRef<Path>) -> Result<()> {
    use std::path::PathBuf;

    env::set_current_dir(&out_dir)?;

    let mut libjpeg = cc::Build::new();
    libjpeg.flag("-Wno-unused-parameter");
    libjpeg.flag("-Wno-unused-variable");
    libjpeg.flag("-Wno-shift-negative-value");

    std::fs::create_dir_all(out_dir.as_ref().join("configured"))
        .expect("Failed to create configured dir");
    // compile::configure_jconfigint().expect("Configure jconfigint.h");
    // compile::configure_jversion().expect("Configure jversion.h");
    // compile::configure_jconfig().expect("Configure jconfig.h");
    libjpeg.include(out_dir.as_ref().join("configured"));
    libjpeg.include(std::path::PathBuf::from(MANIFEST_DIR).join("vendor"));

    let sources = [
        "jcapimin.c",
        "jcapistd.c",
        "jccoefct.c",
        "jccolor.c",
        "jcdctmgr.c",
        "jcdiffct.c",
        "jchuff.c",
        "jcicc.c",
        "jcinit.c",
        "jclhuff.c",
        "jclossls.c",
        "jcmainct.c",
        "jcmarker.c",
        "jcmaster.c",
        "jcomapi.c",
        "jcparam.c",
        "jcphuff.c",
        "jcprepct.c",
        "jcsample.c",
        "jctrans.c",
        "jdapimin.c",
        "jdapistd.c",
        "jdatadst.c",
        "jdatasrc.c",
        "jdcoefct.c",
        "jdcolor.c",
        "jddctmgr.c",
        "jddiffct.c",
        "jdhuff.c",
        "jdicc.c",
        "jdinput.c",
        "jdlhuff.c",
        "jdlossls.c",
        "jdmainct.c",
        "jdmarker.c",
        "jdmaster.c",
        "jdmerge.c",
        "jdphuff.c",
        "jdpostct.c",
        "jdsample.c",
        "jdtrans.c",
        "jerror.c",
        "jfdctflt.c",
        "jfdctfst.c",
        "jfdctint.c",
        "jidctflt.c",
        "jidctfst.c",
        "jidctint.c",
        "jidctred.c",
        "jquant1.c",
        "jquant2.c",
        "jutils.c",
        "jmemmgr.c",
        "jmemnobs.c",
    ];

    let jpeg12 = [
        "jcapistd.c",
        "jccoefct.c",
        "jccolor.c",
        "jcdctmgr.c",
        "jcdiffct.c",
        "jclossls.c",
        "jcmainct.c",
        "jcprepct.c",
        "jcsample.c",
        "jdapistd.c",
        "jdcoefct.c",
        "jdcolor.c",
        "jddctmgr.c",
        "jddiffct.c",
        "jdlossls.c",
        "jdmainct.c",
        "jdmerge.c",
        "jdpostct.c",
        "jdsample.c",
        "jfdctfst.c",
        "jfdctint.c",
        "jidctflt.c",
        "jidctfst.c",
        "jidctint.c",
        "jidctred.c",
        "jquant1.c",
        "jquant2.c",
        "jutils.c",
    ];

    let sources = sources.into_iter().map(|p| {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("vendor")
            .join(p)
    });

    let jpeg12_source = jpeg12.into_iter().map(|p| {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("vendor")
            .join(p)
    });

    // Build and link to jpeg12
    compile::jpeg(jpeg12_source, compile::BitsInJSample::Bits12)?;
    libjpeg.files(sources);

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
    libjpeg.flag("-Wno-unused-parameter");
    libjpeg.flag("-Wno-unused-variable");
    libjpeg.flag("-Wno-shift-negative-value");

    // #[cfg(feature = "simd")]
    // #[cfg(feature = "simd")]
    // {
    //     let simd = compile::simd::simd()?;
    //     libjpeg.define("WITH_SIMD", None);
    //     println!(
    //         "cargo:rustc-link-search=native={}",
    //         simd.0.to_string_lossy()
    //     );
    //     println!("cargo:rustc-link-lib=static={}", simd.1);
    // }

    libjpeg.compile("jpeg");

    Ok(())
}

mod compile {
    use std::io::Write;
    use std::path::PathBuf;

    pub use super::*;
    #[derive(Clone, Copy, Eq, PartialEq, Debug)]
    pub enum JpegLib {
        Jpeg8,
        Jpeg7,
    }

    #[derive(Clone, Copy, Eq, PartialEq, Debug)]
    #[repr(u8)]
    pub enum BitsInJSample {
        Bits8 = 8,
        Bits12 = 12,
        Bits16 = 16,
    }

    pub fn jpeg<P: AsRef<Path>>(
        sources: impl IntoIterator<Item = P>,
        bits: BitsInJSample,
    ) -> Result<()> {
        let mut cc = cc::Build::new();
        cc.flag("-Wno-unused-parameter");
        cc.flag("-Wno-unused-variable");
        cc.flag("-Wno-shift-negative-value");
        cc.define("BITS_IN_JSAMPLE", (bits as u8).to_string().as_str());
        cc.static_flag(true);
        cc.files(sources);
        // cc.cargo_metadata(false);
        let name = format!("jpeg{}", bits as u8);
        cc.try_compile(&name)?;
        // Ok(PathBuf::from(
        //     std::env::var_os("OUT_DIR").ok_or_else(|| anyhow::anyhow!("OUT_DIR not set"))?,
        // )
        // .join(format!("lib{}.a", name)))
        Ok(())
    }

    impl FromStr for JpegLib {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "jpeg8" => Ok(JpegLib::Jpeg8),
                "jpeg7" => Ok(JpegLib::Jpeg7),
                _ => Err(anyhow::anyhow!("Unknown jpeg lib: {}", s)),
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
                    .ok_or_else(|| anyhow::anyhow!("Unable to get version from CMakeLists.txt"));
            }
        }
        anyhow::bail!("Unable to get version from CMakeLists.txt")
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

    pub fn configure_jversion() -> Result<PathBuf> {
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

        jversion.define(
            "JCOPYRIGHT",
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
        Ok(jversion.path)
    }

    pub fn configure_jconfigint() -> Result<PathBuf> {
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
        let target_pointer_width: u8 = env::var("CARGO_CFG_TARGET_POINTER_WIDTH")?.parse()?;
        jconfigint.define(
            "SIZEOF_SIZE_T",
            (target_pointer_width / 8).to_string().as_str(),
        );

        let thread_local = if compiler.is_like_msvc() {
            "__declspec(thread)"
        } else {
            "__thread"
        };
        jconfigint.define("THREAD_LOCAL", thread_local);
        if try_build_c(&format!(
            "{thread_local} int i;  int main(void) {{ i = 0;  return i; }}"
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
            if target_pointer_width == 64 {
                jconfigint.define("HAVE__BITSCANFORWARD64", None);
            } else if target_pointer_width == 32 {
                jconfigint.define("HAVE__BITSCANFORWARD", None);
            }
        }
        jconfigint.write_all(
            br##"
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
        jconfigint.write()?;

        Ok(jconfigint.path)
    }

    pub fn configure_jconfig() -> Result<PathBuf> {
        let mut jconfig = sorse::SorseHeader::new(
            PathBuf::from(env::var("OUT_DIR")?)
                .join("configured")
                .join("jconfig.h"),
        );

        let jpeg_lib_version = jpeg_lib_version()?;
        jconfig.define("JPEG_LIB_VERSION", jpeg_lib_version.to_string().as_str());

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
        // jconfig.define("JCONFIG_INCLUDED", None);
        jconfig.write()?;
        Ok(jconfig.path)
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
            .ok_or_else(|| anyhow::anyhow!("failed to find inline"))?)
    }

    fn try_build_c(c: &str) -> Result<bool> {
        let out_dir = env::var_os("OUT_DIR").unwrap();
        let pwd = env::current_dir()?;
        env::set_current_dir(out_dir)?;
        let mut config = cc::Build::new();
        config.flag("-Wno-unused-parameter");
        config.flag("-Wno-unused-variable");
        config.flag("-Wno-shift-negative-value");
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
                "x86_64" => simd_x86_64(),
                "i386" => todo!(),
                "aarch64" => simd_neon(),
                _ => Err(anyhow::anyhow!("Unsupported arch for simd")),
            }
        }

        pub fn simd_x86_64() -> Result<(PathBuf, String)> {
            let sources = [
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

            let sources: Vec<PathBuf> = sources
                .iter()
                .map(|s| {
                    PathBuf::from(MANIFEST_DIR)
                        .join("vendor")
                        .join("simd")
                        .join(s)
                })
                .collect();

            // let sources = sources.iter_mut().map(|src| {
            // });

            let mut simd = cc::Build::new();
            simd.files(sources);
            simd.flag("-E");
            simd.compile("sus");
            todo!()
        }

        pub fn simd_neon() -> Result<(PathBuf, String)> {
            let mut neon = cc::Build::new();
            neon.flag("-Wno-unused-parameter");
            neon.flag("-Wno-unused-variable");
            neon.flag("-Wno-shift-negative-value");
            neon.include(PathBuf::from(MANIFEST_DIR).join("vendor"));
            neon.include(PathBuf::from(env::var("OUT_DIR")?).join("configured"));
            // neon.define("BITS_IN_JSAMPLE", "8");
            // neon.define("JCONFIG_INCLUDED", None);

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
