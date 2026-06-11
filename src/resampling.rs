use crate::AudioBuffer;



impl AudioBuffer {

	/// Return the audio-buffer resampled. Automatically picks the best available algorithms depending on the buffer.
	pub fn resampled(mut self, channel_count:usize, sample_rate:u32) -> Self {
		self.resample(channel_count, sample_rate);
		self
	}

	/// Resample the audio. Automatically picks the best available algorithms depending on the buffer.
	pub fn resample(&mut self, channel_count:usize, sample_rate:u32) {

		// If no changes are required, return now.
		if self.channel_count == channel_count && self.sample_rate == sample_rate {
			return;
		}

		// If required channel count is 0, remove all data.
		if channel_count == 0 {
			self.data = Vec::new();
			self.channel_count = 0;
			return;
		}

		// Index loop through samples in a single channel of the output data.
		let sample_rate_scale:f32 = 1.0 / sample_rate as f32 * self.sample_rate as f32;
		let current_channel_size:usize = self.data.len() / self.channel_count;
		let target_channel_size:usize = (current_channel_size as f32 / self.sample_rate as f32 * sample_rate as f32) as usize;
		let max_source_index:usize = self.data.len() - self.channel_count;
		let mut new_data:Vec<f32> = Vec::with_capacity(target_channel_size * channel_count);
		for target_sample_index in 0..target_channel_size {

			// For this index of the output list, find the position in own data.
			let source_sample_index:f32 = target_sample_index as f32 * sample_rate_scale;
			let source_sample_left_index:usize = (source_sample_index.floor() as usize * self.channel_count).min(max_source_index);
			let source_sample_right_factor:f32 = source_sample_index.fract();
			let source_sample_right_factor_zero:bool = source_sample_right_factor == 0.0;

			// Add a single value to the output data for each channel.
			for target_channel_index in 0..channel_count {
				let source_channel_index:usize = target_channel_index % self.channel_count;
				let left_index:usize = source_sample_left_index + source_channel_index;
				let left:f32 = self.data[left_index];
				new_data.push(
					if source_sample_right_factor_zero {
						left
					} else {
						let right_index:usize = (left_index + self.channel_count).min(max_source_index + source_channel_index);
						let right:f32 = self.data[right_index];
						left + (right - left) * source_sample_right_factor
					}
				);
			}
		}
		self.data = new_data;
		self.channel_count = channel_count;
		self.sample_rate = sample_rate;
	}
}