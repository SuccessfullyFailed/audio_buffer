pub struct AudioBuffer {
	pub(crate) data:Vec<f32>,
	pub(crate) channel_count:usize,
	pub(crate) sample_rate:u32
}
impl AudioBuffer {

	/* CONSTRUCTOR METHODS */

	/// Create a new audio buffer from raw data.
	pub fn new(data:Vec<f32>, channel_count:usize, sample_rate:u32) -> AudioBuffer {
		AudioBuffer {
			data,
			channel_count,
			sample_rate
		}
	}
}