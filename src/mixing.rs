use std::ops::{Add, AddAssign};
use crate::AudioBuffer;



impl AudioBuffer {

	/// Return the audio combined with another. Makes the two audio samples overlap and return as one sample. Resamples the addition if it doesn't match the sampling settings as self. Grows self if there is not enough space.
	pub fn combined_with<T:AudioBufferAddition>(mut self, addition:T) -> Self {
		self.combine_with(addition);
		self
	}
	
	/// Combine the audio with another. Makes the addition overlap the current one. Resamples the addition if it doesn't match the sampling settings as self. Grows self if there is not enough space.
	pub fn combine_with<T:AudioBufferAddition>(&mut self, addition:T) {

		// Parse addition.
		let raw_addition:Vec<Vec<f32>> = addition.as_raw_list(self.channel_count, self.sample_rate);
		let longest_buffer_size:usize = raw_addition.iter().map(|data| data.len()).max().unwrap_or_default();

		// Grow data to fit largest buffer.
		if longest_buffer_size > self.data.len() {
			self.data.extend(vec![0.0; longest_buffer_size - self.data.len()]);
		}

		// Overlap the additions with self.
		for addition_wave in raw_addition {
			for (source_sample, addition_sample) in self.data[..addition_wave.len()].iter_mut().zip(addition_wave) {
				*source_sample += addition_sample;
			}
		}

		// Make sure the new wave does not exceed limits.
		self.data[..longest_buffer_size].iter_mut().for_each(|sample| *sample = sample.max(-1.0).min(1.0));
	}
}
impl<T:AudioBufferAddition> Add<T> for AudioBuffer {
	type Output = AudioBuffer;

	fn add(self, addition:T) -> Self::Output {
		self.combined_with(addition)
	}
}
impl<T:AudioBufferAddition> AddAssign<T> for AudioBuffer {
	fn add_assign(&mut self, addition:T) {
		self.combine_with(addition);
	}
}



pub trait AudioBufferAddition {
	fn as_raw_list(self, target_channel_count:usize, target_sample_rate:u32) -> Vec<Vec<f32>>;
}
impl AudioBufferAddition for Vec<f32> {
	fn as_raw_list(self, _target_channel_count:usize, _target_sample_rate:u32) -> Vec<Vec<f32>> {
		vec![self]
	}
}
impl AudioBufferAddition for Vec<Vec<f32>> {
	fn as_raw_list(self, _target_channel_count:usize, _target_sample_rate:u32) -> Vec<Vec<f32>> {
		self
	}
}
impl AudioBufferAddition for AudioBuffer {
	fn as_raw_list(self, target_channel_count:usize, target_sample_rate:u32) -> Vec<Vec<f32>> {
		vec![self.resampled(target_channel_count, target_sample_rate).data]
	}
}
impl AudioBufferAddition for Vec<AudioBuffer> {
	fn as_raw_list(self, target_channel_count:usize, target_sample_rate:u32) -> Vec<Vec<f32>> {
		self.into_iter().map(|buffer| buffer.resampled(target_channel_count, target_sample_rate).data).collect()
	}
}