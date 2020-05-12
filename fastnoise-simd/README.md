# FastNoise SIMD

A mostly safe wrapper for [fastnoise-simd-bindings](https://crates.io/crates/fastnoise-simd-bindings) which is bindings for [FastNoiseSIMD](https://github.com/Auburns/FastNoiseSIMD).

FastNoise SIMD is the SIMD implementation of my noise library [FastNoise](https://github.com/Auburns/FastNoise). It aims to provide faster performance through the use of intrinsic(SIMD) CPU functions. Vectorisation of the code allows noise functions to process data in sets of 4/8/16 increasing performance by 700% in some cases (Simplex).

After releasing FastNoise I got in contact with the author of [FastNoise SIMD](https://github.com/jackmott/FastNoise-SIMD) (naming is coincidence) and was inspired to work with SIMD functions myself. Through his code and discussions with him I created my implementation with even more optimisation thanks to the removal of lookup tables. 

Runtime detection of highest supported instruction set ensures the fastest possible performance with only 1 compile needed. If no support is found it will fallback to standard types (float/int).

## Features

- Value Noise 3D
- Perlin Noise 3D
- Simplex Noise 3D
- Cubic Noise 3D
- Multiple fractal options for all of the above
- White Noise 3D
- Cellular Noise 3D
- Perturb input coordinates in 3D space
- Integrated up-sampling
- Easy to use 3D cave noise

Credit to [CubicNoise](https://github.com/jobtalle/CubicNoise) for the cubic noise algorithm

## Supported Instruction Sets
- ARM NEON
- AVX-512F
- AVX2 - FMA3
- SSE4.1
- SSE2
