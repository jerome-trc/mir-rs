use std::path::PathBuf;

fn main() -> miette::Result<(), Box<dyn std::error::Error>> {
	let repo_path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "mir"].iter().collect();

	if !repo_path.exists() {
		return Err(err_msg(format!(
			"Repository not found at: `{}`",
			repo_path.display()
		)));
	}

	let mut build = cc::Build::new();

	if std::env::var("DEBUG").unwrap() == "false" {
		build.define("NDEBUG", "1");
	}

	if std::env::var("CARGO_FEATURE_PARALLEL").is_ok() {
		build.define("MIR_PARALLEL_GEN", "1");
	}

	build
		.flag("-std=gnu11")
		.flag("-Wno-all")
		.flag("-Wno-extra")
		.flag("-Wno-abi")
		.flag("-fsigned-char")
		.flag_if_supported("-fno-tree-sra")
		.flag_if_supported("-fno-ipa-cp-clone")
		.include(&repo_path)
		.file(repo_path.join("mir.c"))
		.file(repo_path.join("mir-gen.c"))
		.file(repo_path.join("c2mir/c2mir.c"))
		.compile("mir");

	println!("cargo:rustc-link-lib=static=mir");

	bindgen::Builder::default()
		.clang_arg(format!("-I{}", repo_path.display()))
		.header(path_string(repo_path.join("mir.h")))
		.header(path_string(repo_path.join("mir-gen.h")))
		.header(path_string(repo_path.join("c2mir/c2mir.h")))
		.parse_callbacks(Box::new(bindgen::CargoCallbacks))
		.generate()?
		.write_to_file(PathBuf::from(std::env::var("OUT_DIR")?).join("bindings.rs"))?;

	Ok(())
}

/// Used to reduce the number of noisy turbofishes that need to be written.
#[must_use]
fn err_msg(msg: String) -> Box<dyn std::error::Error> {
	Box::from(msg)
}

/// Used for [`bindgen::Builder::header`] arguments.
#[must_use]
fn path_string(pb: PathBuf) -> String {
	let cow = pb.to_string_lossy();
	cow.as_ref().to_string()
}
