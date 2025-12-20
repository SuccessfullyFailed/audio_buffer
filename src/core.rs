#[derive(PartialEq, Clone)]
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



	/* DEBUGGING METHODS */

	/// Print a vertical wave over time that shows the data of the buffer.
	pub fn debug_print(&self, print_width:usize) {
		let print_width_per_lane:f32 = print_width as f32 / self.channel_count as f32;
		let half_print_width_per_lane:f32 = print_width_per_lane / 2.0;
		for samples in self.data.chunks(self.channel_count) {
			println!(
				"||{}||",
				samples.iter().map(|sample| {
					let left:f32 = (half_print_width_per_lane + half_print_width_per_lane * sample.max(-1.0).min(1.0)).ceil();
					let right:f32 = print_width_per_lane - left;
					format!("{}.{}", " ".repeat(left as usize), " ".repeat(right as usize))
				}).collect::<Vec<String>>().join("|")
			);
		}
	}
}