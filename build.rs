// use std::process::abort;
use std::path::{Path, PathBuf};
use std::{env, fs, fs::File};
use std::io::prelude::*;
use anyhow::Result;
use serde_json::json;
use handlebars::Handlebars;
use rxxgen::*;

fn genc_code(gen_types: &[&str]) -> String{
    let tpl = r#"
#include <wrapper.hh>

{{#each code}}
{{{this}}}
{{/each}}
"#.trim_start();

    let hb = Handlebars::new();

    hb.render_template(tpl, &json!({
	"code": gen_types,
    })).unwrap()
}

fn main() -> Result<()> {

    let pkg_name = env::var("CARGO_PKG_NAME")?;
    let prefix = PathBuf::from(env::var("CONDA_PREFIX")?);
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let inc_dirs = vec![Path::new("include").to_path_buf(), prefix.join("include")];

    let header_files = vec!["include/wrapper.hh"];
    let mut src_files = vec!["csrc/wrapper.cc"];

    for i in [].iter()
	.chain(header_files.iter())
	.chain(src_files.iter()) {
        println!("cargo:rerun-if-changed={}", i);
    }


    let genc_dir = out_dir.join("gen");
    let genc_file = genc_dir.join("ffi.cc");

    fs::create_dir_all(&genc_dir)?;
    let mut file = File::create(genc_dir.join(&genc_file))?;
    file.write_all(genc_code(
	&[
	    &genc_unique_ptr("rxx_unique_string", "std::unique_ptr<std::string>"),
	    &genc_shared_ptr("rxx_shared_string", "std::shared_ptr<std::string>"),
	    &genc_weak_ptr("rxx_weak_string", "std::weak_ptr<std::string>", "std::shared_ptr<std::string>"),
	]
    ).as_bytes()).unwrap();

    src_files.push(genc_file.to_str().unwrap());

    let genc_file_test;

    if cfg!(feature="test") {
	src_files.push("csrc/test.cc");

	for i in [
	    "csrc/test.cc"
	] {
	    println!("cargo:rerun-if-changed={}", i);
	}

	genc_file_test = genc_dir.join("ffi_test.cc");

	fs::create_dir_all(&genc_dir)?;
	let mut file = File::create(genc_dir.join(&genc_file_test))?;
	file.write_all(genc_code(
	    &[
		&genc_unique_ptr("rxx_unique_i64", "std::unique_ptr<int64_t>"),
		&genc_shared_ptr("rxx_shared_i64", "std::shared_ptr<int64_t>"),
		&genc_weak_ptr("rxx_weak_i64", "std::weak_ptr<int64_t>", "std::shared_ptr<int64_t>"),
		&genc_vector("rxx_vector_i64", "std::vector<int64_t>", "int64_t"),
	    ]
	).as_bytes()).unwrap();

	src_files.push(genc_file_test.to_str().unwrap());
    }

    cc::Build::new()
        .files(&src_files)
        .cpp(true)
        .flag_if_supported("-std=c++14")
        .includes(&inc_dirs)
        .compile("rxx");

    // generate output files for downstream
    let dir = out_dir.join("include").join(&pkg_name);
    fs::create_dir_all(&dir)?;
    let dst_f = dir.join("wrapper.hh");
    fs::copy("include/wrapper.hh", &dst_f)?;

    // let dir = out_dir.join("genrxx").join(&pkg_name);
    // fs::create_dir_all(&dir)?;
    // let dst_f = dir.join("genrxx.py");
    // fs::copy("genrxx/genrxx.py", &dst_f)?;

    Ok(())
}
