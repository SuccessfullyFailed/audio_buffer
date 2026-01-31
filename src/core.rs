use std::fmt::Debug;



#[derive(Clone, PartialEq, Debug)]
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

	/// Wether or not this buffer is similar to another.
	pub fn similar_to(&self, other:&AudioBuffer, max_sample_offset:f32) -> bool {

		// If simple properties don't match, return false.
		if self.channel_count != other.channel_count || self.sample_rate != other.sample_rate || self.data.len() != other.data.len() {
			return false;
		}

		// Check for differences in data values using a raw pointer, should be faster and increase performance for large audio.
		let own_data_ptr:*const f32 = self.data.as_ptr();
		let other_data_ptr:*const f32 = other.data.as_ptr();
		unsafe {
			for sample_index in 0..self.data.len() {
				if (*own_data_ptr.add(sample_index) - *other_data_ptr.add(sample_index)).abs() > max_sample_offset {
					return false;
				}
			}
		}

		// No meaningful differences were found.
		true
	}

	/// Assert this buffer is similar to another. A lot like the 'assert_eq' macro, but allows for a sample offset.
	pub fn assert_similar(&self, other:&AudioBuffer, max_sample_offset:f32) {
		if !self.similar_to(other, max_sample_offset) {
			let data_scale:f32 = if max_sample_offset < 1.0 { 1.0 / max_sample_offset } else { max_sample_offset };
			panic!(
				"Assertion `left similar to right` failed.\nleft:  AudioBuffer {{\n\tchannel_count: {}\n\tsample_rate: {}\n\tdata length: {}\n\tdata simplified: {:?}\n\tdata raw: {:?}\n}}\nright: AudioBuffer {{\n\tchannel_count: {}\n\tsample_rate: {}\n\tdata length: {}\n\tdata simplified: {:?}\n\tdata raw: {:?}\n}}",
				self.channel_count,
				self.sample_rate,
				self.data.len(),
				self.data.iter().map(|value| (value * data_scale).round() / data_scale).collect::<Vec<f32>>(),
				self.data,
				other.channel_count,
				other.sample_rate,
				self.data.len(),
				other.data.iter().map(|value| (value * data_scale).round() / data_scale).collect::<Vec<f32>>(),
				other.data
			);
		}
	}
}