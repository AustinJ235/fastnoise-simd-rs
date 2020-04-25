use super::{
	NoiseType,
	FractalType,
	CellularReturnType,
	CellularDistanceFn,
	PerturbType,
};

#[derive(PartialEq,Debug,Clone)]
pub struct GeneralProperties {
	pub noise_type: NoiseType,
	/// Default: 1337; Seed used for all noise types
	pub seed: i32,
	/// Frequency for all noise types
	pub frequency: f32,
	/// Default: 1.0; Scaling factor for x axis.
	pub x_scale: f32,
	/// Default: 1.0; Scaling factor for y axis.
	pub y_scale: f32,
	/// Default: 1.0; Scaling factor for z axis.
	pub z_scale: f32,
}

impl Default for GeneralProperties {
	fn default() -> Self {
		GeneralProperties {
			noise_type: NoiseType::SimplexFractal,
			seed: 1337,
			frequency: 0.01,
			x_scale: 1.0,
			y_scale: 1.0,
			z_scale: 1.0,
		}
	}
}

#[derive(PartialEq,Debug,Clone)]
pub struct FractalProperties {
	/// Default: FBM; Method for combining octaves in all fractal noise types.
	pub fractal_type: FractalType,
	/// Default: 3; Octave count for all fractal noise types
	pub octaves: i32,
	/// Default: 2.0; Octave lacunarity for all fractal noise types
	pub lacunarity: f32,
	/// Default: 0.5; Octave gain for all fractal noise types.
	pub gain: f32,
}

impl Default for FractalProperties {
	fn default() -> Self {
		FractalProperties {
			fractal_type: FractalType::FBM,
			octaves: 3,
			lacunarity: 2.0,
			gain: 0.5,
		}
	}
}

#[derive(PartialEq,Debug,Clone)]
pub struct CellularProperties {
	/// Default: Distance; Return type from cellular noise calculations.
	pub return_type: CellularReturnType,
	/// Default: Euclidean; Distance function used in cellular noise calculations.
	pub return_func: CellularDistanceFn,
	/// Default: 0; Should be lower than `distance2_index1` and < 4 && >= 0.
	pub distance2_index0: i32,
	/// Default: 1; Should be greater than `distance2_index0` and >= 0.
	pub distance2_index1: i32,
	/// Default: 0.45; Sets the maximum distance a cellular point can move from it's grid
	/// position. Setting this high will make artifacts more common.
	pub jitter: f32,
	/// Default: Simplex; The type of noise used if cellular return type is set
	/// the NoiseLookup.
	pub noise_lookup_type: NoiseType,
	/// Default: 0.2; Relative frequency on the cellular noise lookup return type.
	pub noise_lookup_freq: f32
}

impl Default for CellularProperties {
	fn default() -> Self {
		CellularProperties {
			return_type: CellularReturnType::Distance,
			return_func: CellularDistanceFn::Euclidean,
			distance2_index0: 0,
			distance2_index1: 1,
			jitter: 0.45,
			noise_lookup_type: NoiseType::Simplex,
			noise_lookup_freq: 0.2
		}
	}
}

#[derive(PartialEq,Debug,Clone)]
pub struct PerturbProperties {
	pub perturb_type: PerturbType,
	/// Default: 1.0; The maximum distance the input position can be perturbed.
	pub amp: f32,
	/// Default: 0.5; The relative frequency for the perturb gradient.
	pub frequency: f32,
	/// Default: 3; Octave count for perturb fractal types.
	pub fractal_octaves: i32,
	/// Default: 2.0; Octave lacunarity for perturb fractal types.
	pub fractal_lacunarity: f32,
	/// Default: 0.5; Octave gain for perturb fractal types.
	pub fractal_gain: f32,
	/// Default: 1.0; The length for vectors after perturb normalising.
	pub normalise_length: f32
}

impl Default for PerturbProperties {
	fn default() -> Self {
		PerturbProperties {
			perturb_type: PerturbType::None,
			amp: 1.0,
			frequency: 0.5,
			fractal_octaves: 3,
			fractal_lacunarity: 2.0,
			fractal_gain: 0.5,
			normalise_length: 1.0,
		}
	}
}
