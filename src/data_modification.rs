use crate::AudioBuffer;
use std::ops::Range;



impl AudioBuffer {
	
	/// Drain a specific amount of data and remove it from the buffer front of the. Take the channel count in consideration when passing the range. For example, a stereo sample should always take a range where the start and end are both divisible by 2.
	pub fn drain_data(&mut self, range:Range<usize>) -> Vec<f32> {
		self.data.drain(range).collect()
	}
}