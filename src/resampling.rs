use crate::AudioBuffer;



impl AudioBuffer {

	/// Return the audio-buffer resampled. Automatically picks the best available algorithms depending on the buffer.
	pub fn resampled(mut self, channel_count:usize, sample_rate:u32) -> Self {
		self.resample(channel_count, sample_rate);
		self
	}

	/// Resample the audio. Automatically picks the best available algorithms depending on the buffer.
	pub fn resample(&mut self, channel_count:usize, sample_rate:u32) {
		if channel_count < self.channel_count {
			self.shrink_channels(channel_count);
		}
		if channel_count > self.channel_count {
			self.grow_channels(channel_count);
		}
		if sample_rate < self.sample_rate {
			self.shrink_sample_rate_fast(sample_rate);
		}
		if sample_rate > self.sample_rate {
			self.grow_sample_rate_fast(sample_rate);
		}
	}
	
	/// Shrink the channel count of this sample.
	fn shrink_channels(&mut self, new_channel_count:usize) {

		// Combine stereo to mono.
		if self.channel_count == 2 && new_channel_count == 1 {
			let data_len:usize = self.data.len();
			let mut store_cursor:usize = 0;
			let mut take_cursor:usize = 0;
			while take_cursor < data_len {
				self.data[store_cursor] = (self.data[take_cursor] + self.data[take_cursor + 1]).max(-1.0).min(1.0);
				store_cursor += 1;
				take_cursor += 2;
			}
			self.data.drain(..data_len / 2);
		}

		// Remove spare channels.
		else {
			let current_channel_count:usize = self.channel_count;
			let mut index:usize = 0;
			self.data.retain(|_| {
				let keep:bool = index % current_channel_count > new_channel_count;
				index += 1;
				keep
			});
		}

		// Set new channel count.
		self.channel_count = new_channel_count;
	}

	/// Grow the channel count of this sample.
	fn grow_channels(&mut self, new_channel_count:usize) {
		let current_channel_count:usize = self.channel_count;
		self.data = (0..self.data.len() / current_channel_count).map(|sample_index|
			(0..new_channel_count).map(|channel_index|
				self.data[sample_index * current_channel_count + (channel_index % current_channel_count)]
			).collect::<Vec<f32>>()
		).flatten().collect();
		self.channel_count = new_channel_count;
	}

	/// Shrink the sample rate of this sample quickly, but slightly inaccurately.
	fn shrink_sample_rate_fast(&mut self, new_sample_rate:u32) {
		let current_sample_count:usize = self.data.len();
		let channel_count:usize = self.channel_count;

		let new_sample_count:usize = (current_sample_count as f32 / self.sample_rate as f32 * new_sample_rate as f32) as usize;
		let samples_to_remove:usize = current_sample_count - new_sample_count;
		let remove_sample_every:usize = current_sample_count / samples_to_remove / channel_count; // Make sure to remove sample in each channel.
		let max_mod_index:usize = remove_sample_every - channel_count;
		
		let mut index:usize = 0;
		self.data.retain(|_| {
			let keep:bool = index % remove_sample_every < max_mod_index;
			index += 1;
			keep
		});

		self.sample_rate = new_sample_rate;
	}

	/// Grow the sample rate of this sample quickly, but slightly inaccurately.
	fn grow_sample_rate_fast(&mut self, new_sample_rate:u32) {
		let current_sample_count:usize = self.data.len();
		let channel_count:usize = self.channel_count;

		let new_sample_count:usize = (current_sample_count as f32 / self.sample_rate as f32 * new_sample_rate as f32) as usize;
		let samples_to_insert:usize = new_sample_count - current_sample_count;
		let insert_one_sample_every:usize = current_sample_count / samples_to_insert / channel_count; // Make sure to insert sample in each channel.
		
		let mut insert_index:usize = 1;
		for _ in 0..samples_to_insert {
			let new_data:Vec<f32> = self.data.iter().skip(insert_index).take(channel_count).cloned().collect();
			self.data.splice(insert_index..insert_index, new_data);
			insert_index += insert_one_sample_every + 1; // Take inserted sample into account.
		}

		self.sample_rate = new_sample_rate;
	}
}