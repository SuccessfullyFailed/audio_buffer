use std::fmt::Debug;



#[derive(Clone)]
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



	/* PROPERTY GETTER METHODS */

	/// Get the data of the buffer.
	pub fn data(&self) -> &[f32] {
		&self.data
	}

	/// Get the data of the buffer mutably.
	pub fn data_mut(&mut self) -> &mut [f32] {
		&mut self.data
	}

	/// Get the channel count of the buffer.
	pub fn channel_count(&self) -> usize {
		self.channel_count
	}

	/// Get the sample rate of the buffer.
	pub fn sample_rate(&self) -> u32 {
		self.sample_rate
	}



	/* USAGE METHODS */

	/// Print a vertical wave over time that shows the data of the buffer.
	pub fn print_wave(&self, print_width:usize) {
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
impl Debug for AudioBuffer {
	fn fmt(&self, f:&mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "AudioBuffer {{ channel_count: {}, sample_rate: {}, data: {:?} }}", self.channel_count, self.sample_rate, self.data.iter().map(|value| (value * 1000.0).round() / 1000.0).collect::<Vec<f32>>())
	}
}
impl PartialEq for AudioBuffer {
	fn eq(&self, other:&Self) -> bool {
		const MAX_SAMPLE_OFFSET: f32 = 0.0001;

		// If simple properties don't match, return false.
		if self.channel_count != other.channel_count || self.sample_rate != other.sample_rate || self.data.len() != other.data.len() {
			return false;
		}

		// Check for differences in data values using a raw pointer, should be faster and increase performance for large audio.
		let own_data_ptr:*const f32 = self.data.as_ptr();
		let other_data_ptr:*const f32 = other.data.as_ptr();
		unsafe {
			for sample_index in 0..self.data.len() {
				if (*own_data_ptr.add(sample_index) - *other_data_ptr.add(sample_index)).abs() > MAX_SAMPLE_OFFSET {
					return false;
				}
			}
		}

		// No meaningful differences were found.
		true
	}
}