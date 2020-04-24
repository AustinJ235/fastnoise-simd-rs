extern crate cc;

fn main() {
	cc::Build::new()
		.cpp(true)
		.warnings(false)
		.extra_warnings(false)
		.flag_if_supported("-std=c++11")
		.flag_if_supported("-msse2")
		.flag_if_supported("-msse4.1")
		.flag_if_supported("-mavx2")
		.flag_if_supported("-mavx512f")
		.include("./src/cpp/")
		.file("./src/cpp/FastNoiseSIMD_sse41.cpp")
		.file("./src/cpp/FastNoiseSIMD_sse2.cpp")
		.file("./src/cpp/FastNoiseSIMD_neon.cpp")
		.file("./src/cpp/FastNoiseSIMD_internal.cpp")
		.file("./src/cpp/FastNoiseSIMD_avx512.cpp")
		.file("./src/cpp/FastNoiseSIMD_avx2.cpp")
		.file("./src/cpp/FastNoiseSIMD.cpp")
		.compile("FastNoiseSIMD");
}