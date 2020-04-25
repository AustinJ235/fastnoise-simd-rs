#![allow(non_upper_case_globals)]

extern crate fastnoise_simd_bindings;

pub mod enums;
pub mod vector_set;
pub mod noise_set;
pub mod properties;

pub use enums::{
	NoiseType,
	FractalType,
	PerturbType,
	CellularDistanceFn,
	CellularReturnType,
	SIMDType,
};
pub use properties::{
	GeneralProperties,
	FractalProperties,
	CellularProperties,
	PerturbProperties,
};
pub use vector_set::VectorSet;
pub use noise_set::NoiseSet;
use fastnoise_simd_bindings as binds;

pub struct FastNoiseSIMD {
	inner: *mut binds::FastNoiseSIMD,
}

impl FastNoiseSIMD {
	pub fn new(seed: i32) -> Self {
		unsafe {
			FastNoiseSIMD {
				inner: binds::FastNoiseSIMD::NewFastNoiseSIMD(seed),
			}
		}
	}
	
	pub fn get_simd_type() -> SIMDType {
		unsafe {
			SIMDType::from_c(binds::FastNoiseSIMD::GetSIMDLevel() as u32)
		}
	}
	
	pub fn set_general_properties(
		&mut self,
		properties: GeneralProperties
	) {
		let inner = unsafe { &mut *self.inner };
		inner.m_noiseType = properties.noise_type.into_c();
		inner.m_seed = properties.seed;
		inner.m_frequency = properties.frequency;
		inner.m_xScale = properties.x_scale;
		inner.m_yScale = properties.y_scale;
		inner.m_zScale = properties.z_scale;
	}
	
	pub fn get_general_properties(&self) -> GeneralProperties {
		let inner = unsafe { &*self.inner };
		
		GeneralProperties {
			noise_type: NoiseType::from_c(inner.m_noiseType),
			seed: inner.m_seed,
			frequency: inner.m_frequency,
			x_scale: inner.m_xScale,
			y_scale: inner.m_yScale,
			z_scale: inner.m_zScale,
		}
	}
	
	pub fn set_fractal_properties(
		&mut self,
		properties: FractalProperties
	) {
		let inner = unsafe { &mut *self.inner };
		inner.m_fractalType = properties.fractal_type.into_c();
		inner.m_octaves = properties.octaves;
		inner.m_lacunarity = properties.lacunarity;
		inner.m_gain = properties.gain;
		inner.m_fractalBounding =  unsafe {
			binds::FastNoiseSIMD::CalculateFractalBounding(
				inner.m_octaves, inner.m_gain) };
	}
	
	pub fn get_fractal_properties(&self) -> FractalProperties {
		let inner = unsafe { &*self.inner };
		
		FractalProperties {
			fractal_type: FractalType::from_c(inner.m_fractalType),
			octaves: inner.m_octaves,
			lacunarity: inner.m_lacunarity,
			gain: inner.m_gain
		}
	}
	
	pub fn set_cellular_properties(
		&mut self,
		properties: CellularProperties
	) {
		let inner = unsafe { &mut *self.inner };
		inner.m_cellularReturnType = properties.return_type.into_c();
		inner.m_cellularDistanceFunction = properties.return_func.into_c();
		inner.m_cellularDistanceIndex0 = properties.distance2_index0;
		inner.m_cellularDistanceIndex1 = properties.distance2_index1;
		inner.m_cellularJitter = properties.jitter;
		inner.m_cellularNoiseLookupType = properties.noise_lookup_type.into_c();
		inner.m_cellularNoiseLookupFrequency = properties.noise_lookup_freq;
	}
	
	pub fn get_cellular_properties(&self) -> CellularProperties {
		let inner = unsafe { &*self.inner };
		
		CellularProperties {
			return_type: CellularReturnType::from_c(inner.m_cellularReturnType),
			return_func: CellularDistanceFn::from_c(inner.m_cellularDistanceFunction),
			distance2_index0: inner.m_cellularDistanceIndex0,
			distance2_index1: inner.m_cellularDistanceIndex1,
			jitter: inner.m_cellularJitter,
			noise_lookup_type: NoiseType::from_c(inner.m_cellularNoiseLookupType),
			noise_lookup_freq: inner.m_cellularNoiseLookupFrequency
		}
	}
	
	pub fn set_perturb_properties(
		&mut self,
		properties: PerturbProperties,
	) {
		let inner = unsafe { &mut *self.inner };
		inner.m_perturbType = properties.perturb_type.into_c();
		inner.m_perturbAmp = properties.amp / 511.5;
		inner.m_perturbFrequency = properties.frequency;
		inner.m_perturbOctaves = properties.fractal_octaves;
		inner.m_perturbLacunarity = properties.fractal_lacunarity;
		inner.m_perturbGain = properties.fractal_gain;
		inner.m_perturbNormaliseLength = properties.normalise_length;
		inner.m_perturbFractalBounding = unsafe {
			binds::FastNoiseSIMD::CalculateFractalBounding(
				inner.m_perturbOctaves, inner.m_perturbGain) };
	}
	
	pub fn get_perturb_properties(&self) -> PerturbProperties {
		let inner = unsafe { &*self.inner };
		
		PerturbProperties {
			perturb_type: PerturbType::from_c(inner.m_perturbType),
			amp: inner.m_perturbAmp * 511.5,
			frequency: inner.m_perturbFrequency,
			fractal_octaves: inner.m_perturbOctaves,
			fractal_lacunarity: inner.m_perturbLacunarity,
			fractal_gain: inner.m_perturbGain,
			normalise_length: inner.m_perturbNormaliseLength
		}
	}
			
	/// TODO: Not implemented
	pub fn get_vector_set(
		_x_size: i32,
		_y_size: i32, 
		_z_size: i32
	) -> VectorSet {
		/*fn GetVectorSet(
			xSize: c_int,
			ySize: c_int,
			zSize: c_int
		) -> *mut FastNoiseVectorSet*/
		
		unimplemented!()
	}

	/// TODO: Not implemented
	pub fn get_sampling_vector_set(
		_sample_scale: i32,
		_x_size: i32,
		_y_size: i32,
		_z_size: i32
	) -> VectorSet {
		/*fn GetSamplingVectorSet(
			sampleScale: c_int,
			xSize: c_int,
			ySize: c_int,
			zSize: c_int
		) -> *mut FastNoiseVectorSet*/
		
		unimplemented!()
	}
	
	/// TODO: Not implemented
	pub fn fill_vector_set(
		_vector_set: &mut VectorSet,
		_x_size: i32,
		_y_size: i32,
		_z_size: i32
	) {
		/*fn FillVectorSet(
			vectorSet: *mut FastNoiseVectorSet,
			xSize: c_int,
			ySize: c_int,
			zSize: c_int
		)*/
		
		unimplemented!()
	}
	
	/// TODO: Not implemented
	pub fn fill_sampling_vector_set(
		_vector_set: &mut VectorSet,
		_sample_scale: i32,
		_x_size: i32,
		_y_size: i32,
		_z_size: i32
	) {
		/*fn FillSamplingVectorSet(
			vectorSet: *mut FastNoiseVectorSet,
			sampleScale: c_int,
			xSize: c_int,
			ySize: c_int,
			zSize: c_int
		)*/
		
		unimplemented!()
	}
	
	/// TODO: Not implemented
	pub fn fill_noise_set1(
		&mut self,
		_noise_set: &mut NoiseSet,
		_vector_set: &mut VectorSet,
		_x_offset: f32,
		_y_offset: f32,
		_z_offset: f32
	) {
		/*fn FillNoiseSet1(
			&mut self,
			noiseSet: *mut f32,
			vectorSet: *mut FastNoiseVectorSet,
			xOffset: f32,
			yOffset: f32,
			zOffset: f32
		)*/
		
		unimplemented!()
	}
	
	pub fn get_noise_set(
		&mut self,
		x_start: i32,
		y_start: i32,
		z_start: i32,
		x_size: i32,
		y_size: i32,
		z_size: i32,
		scale_modifier: f32
	) -> NoiseSet {
		let inner = unsafe {
			(*self.inner).GetNoiseSet(
				x_start,
				y_start,
				z_start,
				x_size,
				y_size,
				z_size,
				scale_modifier
			)
		};
		
		NoiseSet {
			inner,
			x_start,
			y_start,
			z_start,
			x_size,
			y_size,
			z_size,
		}
	}
	
	pub fn fill_noise_set(
		&mut self,
		noise_set: &mut NoiseSet,
		x_start: i32,
		y_start: i32,
		z_start: i32,
		x_size: i32,
		y_size: i32,
		z_size: i32,
		scale_modifier: f32
	) {	
		unsafe {
			(*self.inner).FillNoiseSet(
				noise_set.inner,
				x_start,
				y_start,
				z_start,
				x_size,
				y_size,
				z_size,
				scale_modifier
			)
		}
		
		noise_set.x_start = x_start;
		noise_set.y_start = y_start;
		noise_set.z_start = z_start;
		noise_set.x_size = x_size;
		noise_set.y_size = y_size;
		noise_set.z_size = z_size;
	}
	
	pub fn get_sampled_noise_set(
		&mut self,
		x_start: i32,
		y_start: i32,
		z_start: i32,
		x_size: i32,
		y_size: i32,
		z_size: i32,
		sample_scale: i32,
	) -> NoiseSet {
		let inner = unsafe {
			(*self.inner).GetSampledNoiseSet(
				x_start,
				y_start,
				z_start,
				x_size,
				y_size,
				z_size,
				sample_scale
			)
		};
		
		NoiseSet {
			inner,
			x_start,
			y_start,
			z_start,
			x_size,
			y_size,
			z_size,
		}
	}
	
	pub fn get_white_noise_set(
		&mut self,
		x_start: i32,
		y_start: i32,
		z_start: i32,
		x_size: i32,
		y_size: i32,
		z_size: i32,
		scale_modifier: f32
	) -> NoiseSet {
		let inner = unsafe {
			(*self.inner).GetWhiteNoiseSet(
				x_start,
				y_start,
				z_start,
				x_size,
				y_size,
				z_size,
				scale_modifier
			)
		};
		
		NoiseSet {
			inner,
			x_start,
			y_start,
			z_start,
			x_size,
			y_size,
			z_size,
		}
	}
	
	pub fn get_value_set(
		&mut self,
		x_start: i32,
		y_start: i32,
		z_start: i32,
		x_size: i32,
		y_size: i32,
		z_size: i32,
		scale_modifier: f32
	) -> NoiseSet {
		let inner = unsafe {
			(*self.inner).GetValueSet(
				x_start,
				y_start,
				z_start,
				x_size,
				y_size,
				z_size,
				scale_modifier
			)
		};
		
		NoiseSet {
			inner,
			x_start,
			y_start,
			z_start,
			x_size,
			y_size,
			z_size,
		}
	}
	
	pub fn get_value_fractal_set(
		&mut self,
		x_start: i32,
		y_start: i32,
		z_start: i32,
		x_size: i32,
		y_size: i32,
		z_size: i32,
		scale_modifier: f32
	) -> NoiseSet {
		let inner = unsafe {
			(*self.inner).GetValueFractalSet(
				x_start,
				y_start,
				z_start,
				x_size,
				y_size,
				z_size,
				scale_modifier
			)
		};
		
		NoiseSet {
			inner,
			x_start,
			y_start,
			z_start,
			x_size,
			y_size,
			z_size,
		}
	}
	
	pub fn get_perlin_set(
		&mut self,
		x_start: i32,
		y_start: i32,
		z_start: i32,
		x_size: i32,
		y_size: i32,
		z_size: i32,
		scale_modifier: f32
	) -> NoiseSet {
		let inner = unsafe {
			(*self.inner).GetPerlinSet(
				x_start,
				y_start,
				z_start,
				x_size,
				y_size,
				z_size,
				scale_modifier
			)
		};
		
		NoiseSet {
			inner,
			x_start,
			y_start,
			z_start,
			x_size,
			y_size,
			z_size,
		}
	}
	
	pub fn get_perlin_fractal_set(
		&mut self,
		x_start: i32,
		y_start: i32,
		z_start: i32,
		x_size: i32,
		y_size: i32,
		z_size: i32,
		scale_modifier: f32
	) -> NoiseSet {
		let inner = unsafe {
			(*self.inner).GetPerlinFractalSet(
				x_start,
				y_start,
				z_start,
				x_size,
				y_size,
				z_size,
				scale_modifier
			)
		};
		
		NoiseSet {
			inner,
			x_start,
			y_start,
			z_start,
			x_size,
			y_size,
			z_size,
		}
	}
	
	pub fn get_simplex_set(
		&mut self,
		x_start: i32,
		y_start: i32,
		z_start: i32,
		x_size: i32,
		y_size: i32,
		z_size: i32,
		scale_modifier: f32
	) -> NoiseSet {
		let inner = unsafe {
			(*self.inner).GetSimplexSet(
				x_start,
				y_start,
				z_start,
				x_size,
				y_size,
				z_size,
				scale_modifier
			)
		};
		
		NoiseSet {
			inner,
			x_start,
			y_start,
			z_start,
			x_size,
			y_size,
			z_size,
		}
	}
	
	pub fn get_simplex_fractal_set(
		&mut self,
		x_start: i32,
		y_start: i32,
		z_start: i32,
		x_size: i32,
		y_size: i32,
		z_size: i32,
		scale_modifier: f32
	) -> NoiseSet {
		let inner = unsafe {
			(*self.inner).GetSimplexFractalSet(
				x_start,
				y_start,
				z_start,
				x_size,
				y_size,
				z_size,
				scale_modifier
			)
		};
		
		NoiseSet {
			inner,
			x_start,
			y_start,
			z_start,
			x_size,
			y_size,
			z_size,
		}
	}
	
	pub fn get_cellular_set(
		&mut self,
		x_start: i32,
		y_start: i32,
		z_start: i32,
		x_size: i32,
		y_size: i32,
		z_size: i32,
		scale_modifier: f32
	) -> NoiseSet {
		let inner = unsafe {
			(*self.inner).GetCellularSet(
				x_start,
				y_start,
				z_start,
				x_size,
				y_size,
				z_size,
				scale_modifier
			)
		};
		
		NoiseSet {
			inner,
			x_start,
			y_start,
			z_start,
			x_size,
			y_size,
			z_size,
		}
	}
	
	pub fn get_cubic_set(
		&mut self,
		x_start: i32,
		y_start: i32,
		z_start: i32,
		x_size: i32,
		y_size: i32,
		z_size: i32,
		scale_modifier: f32
	) -> NoiseSet {
		let inner = unsafe {
			(*self.inner).GetCubicSet(
				x_start,
				y_start,
				z_start,
				x_size,
				y_size,
				z_size,
				scale_modifier
			)
		};
		
		NoiseSet {
			inner,
			x_start,
			y_start,
			z_start,
			x_size,
			y_size,
			z_size,
		}
	}
	
	pub fn get_cubic_fractal_set(
		&mut self,
		x_start: i32,
		y_start: i32,
		z_start: i32,
		x_size: i32,
		y_size: i32,
		z_size: i32,
		scale_modifier: f32
	) -> NoiseSet {
		let inner = unsafe {
			(*self.inner).GetCubicFractalSet(
				x_start,
				y_start,
				z_start,
				x_size,
				y_size,
				z_size,
				scale_modifier
			)
		};
		
		NoiseSet {
			inner,
			x_start,
			y_start,
			z_start,
			x_size,
			y_size,
			z_size,
		}
	}
}