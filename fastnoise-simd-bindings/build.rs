extern crate cc;
extern crate bindgen;

use std::path::PathBuf;
use std::env;

fn main() {
	let mut build = cc::Build::new();
	build.compiler("clang")
		.cpp(true)
		.warnings(false)
		.extra_warnings(false)
		.flag_if_supported("-std=c++11")
		.cargo_metadata(true)
		.include("./src/cpp/")
		.file("./src/cpp/FastNoiseSIMD_internal.cpp")
		.file("./src/cpp/FastNoiseSIMD.cpp");

	if is_x86_feature_detected!("sse2") {
		build
			.flag_if_supported("-msse2")
			.file("./src/cpp/FastNoiseSIMD_sse2.cpp");
	}

	if is_x86_feature_detected!("fma") {
		build.
			flag_if_supported("-mfma");
	}

	if is_x86_feature_detected!("fma") {
		build.
			flag_if_supported("-mfma");
	}

	if is_x86_feature_detected!("sse4.1") {
		build
			.flag_if_supported("-msse4.1")
			.file("./src/cpp/FastNoiseSIMD_sse41.cpp");
	}

	if is_x86_feature_detected!("avx2") {
		build
			.flag_if_supported("-mavx2")
			.file("./src/cpp/FastNoiseSIMD_avx2.cpp");
	}

	if is_x86_feature_detected!("avx512f") {
		build
			.flag_if_supported("-mavx512f")
			.file("./src/cpp/FastNoiseSIMD_avx512.cpp");
	}

	build.compile("FastNoiseSIMD");
	println!("cargo:rerun-if-changed=./src/cpp/FastNoiseSIMD.h");

	let bindings = bindgen::Builder::default()
		.header("./src/cpp/FastNoiseSIMD.h")
		.parse_callbacks(Box::new(bindgen::CargoCallbacks))
		.clang_args(vec!["-x", "c++", "-std=c++11"].into_iter())
		.rust_target(bindgen::RustTarget::Nightly)
		.generate()
		.expect("Unable to generate bindings");

	let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
	bindings
		.write_to_file(out_path.join("./bindings.rs"))
		.expect("Couldn't write bindings!");
}
