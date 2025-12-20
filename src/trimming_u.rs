#[cfg(test)]
mod tests {
	use crate::AudioBuffer;



	#[test]
	fn trim_start() {
		let raw_audio:Vec<f32> = (0..100).map(|index| index as f32 * 0.01).collect();
		let mut audio:AudioBuffer = AudioBuffer::new(raw_audio.clone(), 1, 100);
		audio.trim_start(0.2);
		assert_eq!(&audio.data, &raw_audio[21..]);
	}

	#[test]
	fn trim_start_negative_wave() {
		let raw_audio:Vec<f32> = (0..100).map(|index| index as f32 * -0.01).collect();
		let mut audio:AudioBuffer = AudioBuffer::new(raw_audio.clone(), 1, 100);
		audio.trim_start(0.2);
		assert_eq!(&audio.data, &raw_audio[21..]);
	}

	#[test]
	fn trim_start_negative_threshold() {
		let raw_audio:Vec<f32> = (0..100).map(|index| index as f32 * 0.01).collect();
		let mut audio:AudioBuffer = AudioBuffer::new(raw_audio.clone(), 1, 100);
		audio.trim_start(-0.2);
		assert_eq!(&audio.data, &raw_audio[21..]);
	}

	#[test]
	fn trim_start_multiple_channels() {
		let raw_audio:Vec<f32> = (0..100).map(|index| [index as f32 * 0.01; 3]).flatten().collect();
		let mut audio:AudioBuffer = AudioBuffer::new(raw_audio.clone(), 3, 100);
		audio.trim_start(-0.2);
		assert_eq!(&audio.data, &raw_audio[21 * 3..]);
	}

	#[test]
	fn trim_start_asymmetrical_channels() {
		let raw_audio:Vec<f32> = (0..100).map(|index| [index as f32 * 0.01, index as f32 * 0.003, index as f32 * 0.02]).flatten().collect();
		let mut audio:AudioBuffer = AudioBuffer::new(raw_audio.clone(), 3, 100);
		audio.trim_start(-0.2);
		assert_eq!(&audio.data, &raw_audio[11 * 3..]);
	}



	

	#[test]
	fn trim_end() {
		let raw_audio:Vec<f32> = (0..100).map(|index| (100 - index) as f32 * 0.01).collect();
		let mut audio:AudioBuffer = AudioBuffer::new(raw_audio.clone(), 1, 100);
		audio.trim_end(0.2);
		assert_eq!(&audio.data, &raw_audio[..80]);
	}

	#[test]
	fn trim_end_negative_wave() {
		let raw_audio:Vec<f32> = (0..100).map(|index| (100 - index) as f32 * -0.01).collect();
		let mut audio:AudioBuffer = AudioBuffer::new(raw_audio.clone(), 1, 100);
		audio.trim_end(0.2);
		assert_eq!(&audio.data, &raw_audio[..80]);
	}

	#[test]
	fn trim_end_negative_threshold() {
		let raw_audio:Vec<f32> = (0..100).map(|index| (100 - index) as f32 * 0.01).collect();
		let mut audio:AudioBuffer = AudioBuffer::new(raw_audio.clone(), 1, 100);
		audio.trim_end(-0.2);
		assert_eq!(&audio.data, &raw_audio[..80]);
	}

	#[test]
	fn trim_end_multiple_channels() {
		let raw_audio:Vec<f32> = (0..100).map(|index| [(100 - index) as f32 * 0.01; 3]).flatten().collect();
		let mut audio:AudioBuffer = AudioBuffer::new(raw_audio.clone(), 3, 100);
		audio.trim_end(-0.2);
		assert_eq!(&audio.data, &raw_audio[..80 * 3]);
	}

	#[test]
	fn trim_end_asymmetrical_channels() {
		let raw_audio:Vec<f32> = (0..100).map(|index| [(100 - index) as f32 * 0.01, (100 - index) as f32 * 0.003, (100 - index) as f32 * 0.02]).flatten().collect();
		let mut audio:AudioBuffer = AudioBuffer::new(raw_audio.clone(), 3, 100);
		audio.trim_end(-0.2);
		assert_eq!(&audio.data, &raw_audio[..90 * 3]);
	}
}