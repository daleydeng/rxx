// use std::process::abort;
use anyhow::Result;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};

fn main() -> Result<()> {
    for i in [
        "include/wrapper.hh",
        "genrxx/ffi.cc",
        "genrxx/ffi.rs",
        "genrxx/genrxx.py",
	"src/gen",
    ] {
        println!("cargo:rerun-if-changed={}", i);
    }

    let pkg_name = env::var("CARGO_PKG_NAME")?;
    let prefix = PathBuf::from(env::var("CONDA_PREFIX")?);
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    fs::create_dir_all("src/gen")?;
    Command::new("cog")
        .args(&[
            "-o",
            "src/gen/ffi.cc",
            "genrxx/ffi.cc",
        ])
        .status()?;
    Command::new("cog")
        .args(&[
            "-o",
            "src/gen/ffi.rs",
            "genrxx/ffi.rs",
        ])
        .status()?;

    let inc_dirs = vec![Path::new("include").to_path_buf(), prefix.join("include")];

    cc::Build::new()
        .file("src/gen/ffi.cc")
        .cpp(true)
        .flag_if_supported("-std=c++14")
        .includes(&inc_dirs)
        .compile("rxx");

    // generate output files for downstream
    let dir = out_dir.join("include").join(&pkg_name);
    fs::create_dir_all(&dir)?;
    let dst_f = dir.join("wrapper.hh");
    fs::copy("include/wrapper.hh", &dst_f)?;

    let dir = out_dir.join("genrxx").join(&pkg_name);
    fs::create_dir_all(&dir)?;
    let dst_f = dir.join("genrxx.py");
    fs::copy("genrxx/genrxx.py", &dst_f)?;

    Ok(())
}
