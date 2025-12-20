use crate::AudioBuffer;



impl AudioBuffer {

	/// Trim the quiet start and end off of the buffer.
	pub fn trim(&mut self, volume_threshold:f32) {
		self.trim_end(volume_threshold);
		self.trim_start(volume_threshold);
	}

	/// Trim the quiet start off of the buffer.
	pub fn trim_start(&mut self, volume_threshold:f32) {
		if !self.data.is_empty() {
			let volume_threshold:f32 = volume_threshold.abs();
			let mut cursor:usize = 0;
			for sample_list in self.data.chunks(self.channel_count) {
				if sample_list.iter().any(|sample| sample.abs() > volume_threshold) {
					break;
				} else {
					cursor += self.channel_count;
				}
			}
			self.data.drain(..cursor);
		}
	}

	/// Trim the quiet end off of the buffer.
	pub fn trim_end(&mut self, volume_threshold:f32) {
		if !self.data.is_empty() {
			let volume_threshold:f32 = volume_threshold.abs();
			let mut cursor:usize = self.data.len();
			for sample_list in self.data.chunks(self.channel_count).rev() {
				if sample_list.iter().any(|sample| sample.abs() > volume_threshold) {
					break;
				} else {
					cursor -= self.channel_count;
				}
			}
			self.data.drain(cursor..);
		}
	}
}