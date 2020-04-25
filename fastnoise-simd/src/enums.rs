use fastnoise_simd_bindings::{
	FN_AVX2, FN_AVX512, FN_NEON, FN_NO_SIMD_FALLBACK, FN_SSE2, FN_SSE41,
	FastNoiseSIMD_CellularDistanceFunction_Euclidean, 
	FastNoiseSIMD_CellularDistanceFunction_Natural,
	FastNoiseSIMD_CellularDistanceFunction_Manhattan,
	FastNoiseSIMD_CellularReturnType_CellValue,
	FastNoiseSIMD_CellularReturnType_Distance,
	FastNoiseSIMD_CellularReturnType_Distance2,
	FastNoiseSIMD_CellularReturnType_Distance2Add,
	FastNoiseSIMD_CellularReturnType_Distance2Sub,
	FastNoiseSIMD_CellularReturnType_Distance2Mul,
	FastNoiseSIMD_CellularReturnType_Distance2Div,
	FastNoiseSIMD_CellularReturnType_Distance2Cave,
	FastNoiseSIMD_CellularReturnType_NoiseLookup,
	FastNoiseSIMD_FractalType_Billow, FastNoiseSIMD_FractalType_FBM,
	FastNoiseSIMD_FractalType_RigidMulti, FastNoiseSIMD_NoiseType_Cellular,
	FastNoiseSIMD_NoiseType_Cubic, FastNoiseSIMD_NoiseType_CubicFractal,
	FastNoiseSIMD_NoiseType_Perlin, FastNoiseSIMD_NoiseType_PerlinFractal,
	FastNoiseSIMD_NoiseType_Simplex, FastNoiseSIMD_NoiseType_SimplexFractal,
	FastNoiseSIMD_NoiseType_Value, FastNoiseSIMD_NoiseType_ValueFractal,
	FastNoiseSIMD_NoiseType_WhiteNoise, FastNoiseSIMD_PerturbType_Gradient,
	FastNoiseSIMD_PerturbType_GradientFractal,
	FastNoiseSIMD_PerturbType_GradientFractal_Normalise,
	FastNoiseSIMD_PerturbType_Gradient_Normalise, FastNoiseSIMD_PerturbType_None,
	FastNoiseSIMD_PerturbType_Normalise, FastNoiseSIMD_CellularDistanceFunction,
	FastNoiseSIMD_CellularReturnType, FastNoiseSIMD_FractalType, FastNoiseSIMD_NoiseType,
	FastNoiseSIMD_PerturbType,
};

#[derive(PartialEq,Eq,Debug,Clone,Copy)]
pub enum NoiseType {
	Cellular,
	Cubic,
	CubicFractal,
	Perlin,
	PerlinFractal,
	Simplex,
	SimplexFractal,
	Value,
	ValueFractal,
	WhiteNoise,
}

impl NoiseType {
	pub(crate) fn from_c(from: FastNoiseSIMD_NoiseType) -> NoiseType {
		match from {
			FastNoiseSIMD_NoiseType_Cellular => NoiseType::Cellular,
			FastNoiseSIMD_NoiseType_Cubic => NoiseType::Cubic,
			FastNoiseSIMD_NoiseType_CubicFractal => NoiseType::CubicFractal,
			FastNoiseSIMD_NoiseType_Perlin => NoiseType::Perlin,
			FastNoiseSIMD_NoiseType_PerlinFractal => NoiseType::PerlinFractal,
			FastNoiseSIMD_NoiseType_Simplex	=> NoiseType::Simplex,
			FastNoiseSIMD_NoiseType_SimplexFractal => NoiseType::SimplexFractal,
			FastNoiseSIMD_NoiseType_Value => NoiseType::Value,
			FastNoiseSIMD_NoiseType_ValueFractal => NoiseType::ValueFractal,
			FastNoiseSIMD_NoiseType_WhiteNoise => NoiseType::WhiteNoise,
			_ => unreachable!("Unexpected Noise Type")
		}
	}
	
	pub(crate) fn into_c(self) -> FastNoiseSIMD_NoiseType {
		match self {
			NoiseType::Cellular => FastNoiseSIMD_NoiseType_Cellular,
			NoiseType::Cubic => FastNoiseSIMD_NoiseType_Cubic,
			NoiseType::CubicFractal => FastNoiseSIMD_NoiseType_CubicFractal,
			NoiseType::Perlin => FastNoiseSIMD_NoiseType_Perlin,
			NoiseType::PerlinFractal => FastNoiseSIMD_NoiseType_PerlinFractal,
			NoiseType::Simplex => FastNoiseSIMD_NoiseType_Simplex,
			NoiseType::SimplexFractal => FastNoiseSIMD_NoiseType_SimplexFractal,
			NoiseType::Value => FastNoiseSIMD_NoiseType_Value,
			NoiseType::ValueFractal => FastNoiseSIMD_NoiseType_ValueFractal,
			NoiseType::WhiteNoise => FastNoiseSIMD_NoiseType_WhiteNoise,
		}
	}
}

#[derive(PartialEq,Eq,Debug,Clone,Copy)]
pub enum FractalType {
	Billow,
	FBM,
	RigidMulti,
}

impl FractalType {
	pub(crate) fn from_c(from: FastNoiseSIMD_FractalType) -> FractalType {
		match from {
			FastNoiseSIMD_FractalType_Billow => FractalType::Billow,
			FastNoiseSIMD_FractalType_FBM => FractalType::FBM,
			FastNoiseSIMD_FractalType_RigidMulti => FractalType::RigidMulti,
			_ => unreachable!("Unexpected Fractal Type")
		}
	}
	
	pub(crate) fn into_c(self) -> FastNoiseSIMD_FractalType {
		match self {
			FractalType::Billow => FastNoiseSIMD_FractalType_Billow,
			FractalType::FBM => FastNoiseSIMD_FractalType_FBM,
			FractalType::RigidMulti => FastNoiseSIMD_FractalType_RigidMulti,
		}
	}
}

#[derive(PartialEq,Eq,Debug,Clone,Copy)]
pub enum PerturbType {
	Gradient,
	GradientFractal,
	GradientFractalNormalize,
	GradientNormalize,
	None,
	Normalize,
}

impl PerturbType {
	pub(crate) fn from_c(from: FastNoiseSIMD_PerturbType) -> PerturbType {
		match from {
			FastNoiseSIMD_PerturbType_Gradient => PerturbType::Gradient,
			FastNoiseSIMD_PerturbType_GradientFractal => PerturbType::GradientFractal,
			FastNoiseSIMD_PerturbType_GradientFractal_Normalise	=> PerturbType::GradientFractalNormalize,
			FastNoiseSIMD_PerturbType_Gradient_Normalise => PerturbType::GradientNormalize,
			FastNoiseSIMD_PerturbType_None => PerturbType::None,
			FastNoiseSIMD_PerturbType_Normalise => PerturbType::Normalize,
			_ => unreachable!("Unexpected PerturbType")
		}
	}
	
	pub(crate) fn into_c(self) -> FastNoiseSIMD_PerturbType {
		match self {
			PerturbType::Gradient => FastNoiseSIMD_PerturbType_Gradient,
			PerturbType::GradientFractal => FastNoiseSIMD_PerturbType_GradientFractal,
			PerturbType::GradientFractalNormalize => FastNoiseSIMD_PerturbType_GradientFractal_Normalise,
			PerturbType::GradientNormalize => FastNoiseSIMD_PerturbType_Gradient_Normalise,
			PerturbType::None => FastNoiseSIMD_PerturbType_None,
			PerturbType::Normalize => FastNoiseSIMD_PerturbType_Normalise,
		}
	}
}

#[derive(PartialEq,Eq,Debug,Clone,Copy)]
pub enum CellularDistanceFn {
	Euclidean,
	Manhattan,
	Natural,
}

impl CellularDistanceFn {
	pub(crate) fn from_c(from: FastNoiseSIMD_CellularDistanceFunction) -> CellularDistanceFn {
		match from {
			FastNoiseSIMD_CellularDistanceFunction_Euclidean => CellularDistanceFn::Euclidean,
			FastNoiseSIMD_CellularDistanceFunction_Manhattan => CellularDistanceFn::Manhattan,
			FastNoiseSIMD_CellularDistanceFunction_Natural => CellularDistanceFn::Natural,
			_ => unreachable!("Unexpected Cellular Distance Function")
		}
	}
	
	pub(crate) fn into_c(self) -> FastNoiseSIMD_CellularDistanceFunction {
		match self {
			CellularDistanceFn::Euclidean => FastNoiseSIMD_CellularDistanceFunction_Euclidean,
			CellularDistanceFn::Manhattan => FastNoiseSIMD_CellularDistanceFunction_Manhattan,
			CellularDistanceFn::Natural => FastNoiseSIMD_CellularDistanceFunction_Natural,
		}
	}
}

#[derive(PartialEq,Eq,Debug,Clone,Copy)]
pub enum CellularReturnType {
	CellValue,
	Distance,
	Distance2,
	Distance2Add,
	Distance2Sub,
	Distance2Mul,
	Distance2Div,
	Distance2Cave,
	NoiseLookup
}

impl CellularReturnType {
	pub(crate) fn from_c(from: FastNoiseSIMD_CellularReturnType) -> CellularReturnType {
		match from {
			FastNoiseSIMD_CellularReturnType_CellValue => CellularReturnType::CellValue,
			FastNoiseSIMD_CellularReturnType_Distance => CellularReturnType::Distance,
			FastNoiseSIMD_CellularReturnType_Distance2 => CellularReturnType::Distance2,
			FastNoiseSIMD_CellularReturnType_Distance2Add => CellularReturnType::Distance2Add,
			FastNoiseSIMD_CellularReturnType_Distance2Sub => CellularReturnType::Distance2Sub,
			FastNoiseSIMD_CellularReturnType_Distance2Mul => CellularReturnType::Distance2Mul,
			FastNoiseSIMD_CellularReturnType_Distance2Div => CellularReturnType::Distance2Div,
			FastNoiseSIMD_CellularReturnType_Distance2Cave => CellularReturnType::Distance2Cave,
			FastNoiseSIMD_CellularReturnType_NoiseLookup => CellularReturnType::NoiseLookup,
			_ => unreachable!("Unexpected Cellular Return Type")
		}
	}
	
	pub(crate) fn into_c(self) -> FastNoiseSIMD_CellularReturnType {
		match self {
			CellularReturnType::CellValue => FastNoiseSIMD_CellularReturnType_CellValue,
			CellularReturnType::Distance => FastNoiseSIMD_CellularReturnType_Distance,
			CellularReturnType::Distance2 => FastNoiseSIMD_CellularReturnType_Distance2,
			CellularReturnType::Distance2Add => FastNoiseSIMD_CellularReturnType_Distance2Add,
			CellularReturnType::Distance2Sub => FastNoiseSIMD_CellularReturnType_Distance2Sub,
			CellularReturnType::Distance2Mul => FastNoiseSIMD_CellularReturnType_Distance2Mul,
			CellularReturnType::Distance2Div => FastNoiseSIMD_CellularReturnType_Distance2Div,
			CellularReturnType::Distance2Cave => FastNoiseSIMD_CellularReturnType_Distance2Cave,
			CellularReturnType::NoiseLookup => FastNoiseSIMD_CellularReturnType_NoiseLookup,
		}
	}
}

#[derive(PartialEq,Eq,Debug,Clone,Copy)]
pub enum SIMDType {
	AVX2,
	AVX512,
	Neon,
	NoSIMD,
	SSE2,
	SSE41,
}

impl SIMDType {
	pub(crate) fn from_c(from: u32) -> SIMDType {
		match from {
			FN_AVX2	=> SIMDType::AVX2,
			FN_AVX512 => SIMDType::AVX512,
			FN_NEON	=> SIMDType::Neon,
			FN_NO_SIMD_FALLBACK	=> SIMDType::NoSIMD,
			FN_SSE2	=> SIMDType::SSE2,
			FN_SSE41 => SIMDType::SSE41,
			_ => unreachable!("Unexpected SIMD Type")
		}
	}
}
