use fastnoise_simd_bindings as binds;
use std::iter::Iterator;

pub struct NoiseSet {
	pub(crate) inner: *mut f32,
	pub(crate) x_start: i32,
	pub(crate) y_start: i32,
	pub(crate) z_start: i32,
	pub(crate) x_size: i32,
	pub(crate) y_size: i32,
	pub(crate) z_size: i32,
}

impl Drop for NoiseSet {
	fn drop(&mut self) {
		unsafe {
			binds::FastNoiseSIMD::FreeNoiseSet(self.inner);
		}
	}
}

impl NoiseSet {
	pub fn empty(x_size: i32, y_size: i32, z_size: i32) -> Self {
		let inner = unsafe {
			binds::FastNoiseSIMD::GetEmptySet(x_size * y_size * z_size)
		};
		
		NoiseSet {
			inner,
			x_start: 0,
			y_start: 0,
			z_start: 0,
			x_size: x_size,
			y_size: y_size,
			z_size: z_size,
		}
	}
	
	pub fn iter(&self) -> NoiseSetIter<'_> {
		NoiseSetIter {
			index: 0,
			length: self.x_size as isize * self.y_size as isize * self.z_size as isize,
			inner: self
		}
	}
	
	pub fn iter_mut(&mut self) -> NoiseSetIterMut<'_> {
		NoiseSetIterMut {
			index: 0,
			length: self.x_size as isize * self.y_size as isize * self.z_size as isize,
			inner: self
		}
	}
	
	pub fn enumerated_iter(&self) -> NoiseSetIterEnumerated<'_> {
		NoiseSetIterEnumerated {
			x: self.x_start,
			y: self.y_start,
			z: self.z_start,
			index: 0,
			inner: self
		}
	}
	
	pub fn enumerated_iter_mut(&mut self) -> NoiseSetIterMutEnumerated<'_> {
		NoiseSetIterMutEnumerated {
			x: self.x_start,
			y: self.y_start,
			z: self.z_start,
			index: 0,
			inner: self
		}
	}
	
	pub fn as_vec(&self) -> Vec<f32> {
		let length = self.x_size as isize * self.y_size as isize * self.z_size as isize;
		let mut out = Vec::with_capacity(length as usize);
		
		for i in 0..length {
			unsafe {
				out.push(*(self.inner.offset(i)).clone());
			}
		}
		
		out
	}
	
	pub fn as_nested_vecs(&self) -> Vec<Vec<Vec<f32>>> {
		let mut x_out = Vec::with_capacity(self.x_size as usize);
		let mut i = 0;
	
		for _ in 0..self.x_size {
			let mut y_out = Vec::with_capacity(self.y_size as usize);
		
			for _ in 0..self.y_size {
				let mut z_out = Vec::with_capacity(self.z_size as usize);
				
				for _ in 0..self.z_size {
					unsafe {
						z_out.push(*(self.inner.offset(i)).clone());
						i += 1;
					}
				}
				
				y_out.push(z_out);
			}
			
			x_out.push(y_out);
		}
					
		x_out
	}
}

pub struct NoiseSetIter<'a> {
	index: isize,
	length: isize,
	inner: &'a NoiseSet,
}

impl<'a> Iterator for NoiseSetIter<'a> {
	type Item = &'a f32;
	
	#[inline]
	fn next(&mut self) -> Option<&'a f32> {
		if self.index == self.length {
			None
		} else {
			let out = unsafe {
				Some(&*(self.inner.inner.offset(self.index)))
			};
			
			self.index += 1;
			out
		}
	}
	
	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		(self.length as usize, Some(self.length as usize))
	}
}

pub struct NoiseSetIterMut<'a> {
	index: isize,
	length: isize,
	inner: &'a mut NoiseSet,
}

impl<'a> Iterator for NoiseSetIterMut<'a> {
	type Item = &'a mut f32;
	
	#[inline]
	fn next(&mut self) -> Option<&'a mut f32> {
		if self.index == self.length {
			None
		} else {
			let out = unsafe {
				Some(&mut *(self.inner.inner.offset(self.index)))
			};
			
			self.index += 1;
			out
		}
	}
	
	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		(self.length as usize, Some(self.length as usize))
	}
}

pub struct NoiseSetIterEnumerated<'a> {
	x: i32,
	y: i32,
	z: i32,
	index: isize,
	inner: &'a NoiseSet,
}

impl<'a> Iterator for NoiseSetIterEnumerated<'a> {
	type Item = (i32, i32, i32, &'a f32);
	
	#[inline]
	fn next(&mut self) -> Option<(i32, i32, i32, &'a f32)> {
		if self.x == self.inner.x_size + self.inner.x_start {
			None
		} else {
			let out = unsafe {
				Some((
					self.x,
					self.y,
					self.z,
					&*(self.inner.inner.offset(self.index))
				))
			};
			
			self.z += 1;
			self.index += 1;
			
			if self.z == self.inner.z_size + self.inner.z_start {
				self.z = self.inner.z_start;
				self.y += 1;
				
				if self.y == self.inner.y_size + self.inner.y_start {
					self.y = self.inner.y_start;
					self.x += 1;
				}
			}
			
			out
		}
	}
	
	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		let length = self.inner.x_size as usize
			* self.inner.y_size as usize
			* self.inner.z_size as usize;
		(length, Some(length))
	}
}

pub struct NoiseSetIterMutEnumerated<'a> {
	x: i32,
	y: i32,
	z: i32,
	index: isize,
	inner: &'a mut NoiseSet,
}

impl<'a> Iterator for NoiseSetIterMutEnumerated<'a> {
	type Item = (i32, i32, i32, &'a mut f32);
	
	#[inline]
	fn next(&mut self) -> Option<(i32, i32, i32, &'a mut f32)> {
		if self.x == self.inner.x_size + self.inner.x_start {
			None
		} else {
			let out = unsafe {
				Some((
					self.x,
					self.y,
					self.z,
					&mut *(self.inner.inner.offset(self.index))
				))
			};
			
			self.z += 1;
			self.index += 1;
			
			if self.z == self.inner.z_size + self.inner.z_start {
				self.z = self.inner.z_start;
				self.y += 1;
				
				if self.y == self.inner.y_size + self.inner.y_start {
					self.y = self.inner.y_start;
					self.x += 1;
				}
			}
			
			out
		}
	}
	
	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		let length = self.inner.x_size as usize
			* self.inner.y_size as usize
			* self.inner.z_size as usize;
		(length, Some(length))
	}
}
