use crate::AudioBuffer;



impl AudioBuffer {

	/// Return the audio with the volume multiplied by the given amount.
	pub fn amplified(mut self, volume_multiplier:f32) -> Self {
		self.amplify(volume_multiplier);
		self
	}

	/// Multiply the volume of the audio buffer by the given multiplier.
	pub fn amplify(&mut self, volume_multiplier:f32) {
		if volume_multiplier != 1.0 {
			self.data.iter_mut().for_each(|sample| *sample *= volume_multiplier);
		}
	}
}