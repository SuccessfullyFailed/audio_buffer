#[cfg(test)]
mod test {
	use crate::{ AudioAmplifier, AudioBuffer, AudioEffect };



	#[test]
	fn amplify_simple() {
		let mut buffer:AudioBuffer = AudioBuffer::new((0..100).map(|index| index as f32).collect(), 1, 1000);
		AudioAmplifier::new(2.0).apply_to(&mut buffer);
		assert_eq!(buffer.data, (0..100).map(|index| index as f32 * 2.0).collect::<Vec<f32>>());
	}

	#[test]
	fn amplify_negative() {
		let mut buffer:AudioBuffer = AudioBuffer::new((0..100).map(|index| index as f32).collect(), 1, 1000);
		AudioAmplifier::new(-1.0).apply_to(&mut buffer);
		assert_eq!(buffer.data, (0..100).map(|index| index as f32 * -1.0).collect::<Vec<f32>>());
	}

	#[test]
	fn amplify_negative_switch() {
		let mut buffer:AudioBuffer = AudioBuffer::new((0..100).map(|index| index as f32 * if index & 1 == 1 { -1.0 } else { 1.0 }).collect(), 1, 1000);
		AudioAmplifier::new(-5.0).apply_to(&mut buffer);
		assert_eq!(buffer.data, (0..100).map(|index| index as f32 * if index & 1 == 1 { 5.0 } else { -5.0 }).collect::<Vec<f32>>());
	}

	#[test]
	fn amplify_multi_channel() {
		let mut buffer:AudioBuffer = AudioBuffer::new((0..100).map(|index| index as f32).collect(), 2, 1000);
		AudioAmplifier::new(2.0).apply_to(&mut buffer);
		assert_eq!(buffer.data, (0..100).map(|index| index as f32 * 2.0).collect::<Vec<f32>>());
	}



	#[test]
	fn amplify_to_simple() {
		let mut buffer:AudioBuffer = AudioBuffer::new((0..100).map(|index| index as f32).collect(), 1, 1000);
		AudioAmplifier::new_maximizer(1.0).apply_to(&mut buffer);
		assert_eq!(buffer.data, (0..100).map(|index| index as f32 / 99.0).collect::<Vec<f32>>());
	}

	#[test]
	fn amplify_to_negative() {
		let mut buffer:AudioBuffer = AudioBuffer::new((0..100).map(|index| index as f32).collect(), 1, 1000);
		AudioAmplifier::new_maximizer(-1.0).apply_to(&mut buffer);
		assert_eq!(buffer.data, (0..100).map(|index| index as f32 / 99.0 * -1.0).collect::<Vec<f32>>());
	}

	#[test]
	fn amplify_to_negative_switch() {
		let mut buffer:AudioBuffer = AudioBuffer::new((0..100).map(|index| index as f32 * if index & 1 == 1 { -1.0 } else { 1.0 }).collect(), 1, 1000);
		AudioAmplifier::new_maximizer(-1.0).apply_to(&mut buffer);
		assert_eq!(buffer.data, (0..100).map(|index| index as f32 / 99.0 * if index & 1 == 1 { -1.0 } else { 1.0 }).collect::<Vec<f32>>());
	}

	#[test]
	fn amplify_to_multi_channel() {
		let mut buffer:AudioBuffer = AudioBuffer::new((0..100).map(|index| index as f32).collect(), 2, 1000);
		AudioAmplifier::new_maximizer(2.0).apply_to(&mut buffer);
		assert_eq!(buffer.data, (0..100).map(|index| index as f32 / 99.0 * 2.0).collect::<Vec<f32>>());
	}
}