extern crate cc;

fn main() {
	cc::Build::new()
		.cpp(true)
		.warnings(false)
		.extra_warnings(false)
		.flag_if_supported("/arch:SSE2")
		.flag_if_supported("/arch:AVX2")
		.flag_if_supported("-std=c++11")
		.flag_if_supported("-std=c++11")
		.flag_if_supported("-msse2")
		.flag_if_supported("-msse4.1")
		.flag_if_supported("-mavx2")
		.flag_if_supported("-mavx512f")
		.cargo_metadata(true)
		.include("./src/cpp/")
		.file("./src/cpp/FastNoiseSIMD_sse41.cpp")
		.file("./src/cpp/FastNoiseSIMD_sse2.cpp")
		.file("./src/cpp/FastNoiseSIMD_neon.cpp")
		.file("./src/cpp/FastNoiseSIMD_internal.cpp")
		.file("./src/cpp/FastNoiseSIMD_avx512.cpp")
		.file("./src/cpp/FastNoiseSIMD_avx2.cpp")
		.file("./src/cpp/FastNoiseSIMD.cpp")
		.compile("FastNoiseSIMD");
	
	// bindgen -o ./src/lib.rs --rust-target nightly ./src/cpp/FastNoiseSIMD.h -- -std=c++11 -x c++
	
	/*println!("cargo:rerun-if-changed=./src/cpp/FastNoiseSIMD.h");
	
	let bindings = bindgen::Builder::default()
		.header("./src/cpp/FastNoiseSIMD.h")
		.parse_callbacks(Box::new(bindgen::CargoCallbacks))
		.clang_arg("-std=c++11")
		.clang_arg("-x c++")
		.rust_target(bindgen::RustTarget::Nightly)
		.generate()
		.expect("Unable to generate bindings");
	
	let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
	bindings
		.write_to_file(out_path.join("bindings.rs"))
		.expect("Couldn't write bindings!");*/
		
}