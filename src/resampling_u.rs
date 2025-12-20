#[cfg(test)]
mod test {
	use std::f32::consts::PI;
	use crate::AudioBuffer;



	fn debug_wave(data_length:usize, channel_count:usize, sample_rate:u32) -> AudioBuffer {
		let scale:f32 = 1.0 / (sample_rate as f32 / PI / 2.0);
		AudioBuffer::new(
			(0..data_length * channel_count).map(|index| ((index / channel_count) as f32 * scale).sin()).collect(),
			channel_count,
			sample_rate
		)
	}


	#[test]
	fn create_debug_wave() {
		debug_wave(100, 2, 100).print_wave(100);
	}

	#[test]
	fn increase_channel_count() {
		let original_wave:AudioBuffer = debug_wave(800, 1, 100);
		let resampled_wave:AudioBuffer = original_wave.clone().resampled(3, 100);
		for index in 0..800 {
			assert_eq!(
				resampled_wave.data.iter().skip(index * 3).take(3).collect::<Vec<&f32>>(),
				vec![&original_wave.data[index]; 3]
			);
		}
	}

	#[test]
	fn decrease_channel_count() {
		let single_channel_wave:AudioBuffer = debug_wave(800, 1, 100);
		let original_wave:AudioBuffer = debug_wave(800, 5, 100);
		let resampled_wave:AudioBuffer = original_wave.clone().resampled(2, 100);
		for index in 0..800 {
			assert_eq!(
				original_wave.data.iter().skip(index * 5).take(5).collect::<Vec<&f32>>(),
				vec![&single_channel_wave.data[index]; 5]
			);
			assert_eq!(
				resampled_wave.data.iter().skip(index * 2).take(2).collect::<Vec<&f32>>(),
				vec![&single_channel_wave.data[index]; 2]
			);
		}
	}

	#[test]
	fn increase_sample_rate() {
		let original_wave:AudioBuffer = debug_wave(800, 1, 100);
		let resampled_wave:AudioBuffer = original_wave.clone().resampled(1, 200);
		for index in 0..400 {
			assert_eq!(
				original_wave.data[index],
				resampled_wave.data[index * 2]
			);
		}
	}

	#[test]
	fn decrease_sample_rate() {
		let original_wave:AudioBuffer = debug_wave(800, 1, 100);
		let resampled_wave:AudioBuffer = original_wave.clone().resampled(1, 50);
		for index in 0..400 {
			assert_eq!(
				original_wave.data[index * 2],
				resampled_wave.data[index]
			);
		}
	}
}