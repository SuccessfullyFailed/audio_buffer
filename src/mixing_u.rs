#[cfg(test)]
mod test {
	use crate::AudioBuffer;



	#[test]
	fn add_sample_simple() {
		let original:AudioBuffer = AudioBuffer::new((0..100).map(|index| index as f32 * 0.001).collect(), 1, 1000);
		let addition:AudioBuffer = AudioBuffer::new((0..100).map(|index| index as f32 * 0.002).collect(), 1, 1000);
		let expected:AudioBuffer = AudioBuffer::new((0..100).map(|index| index as f32 * 0.003).collect(), 1, 1000);
		assert_eq!(original.combined_with(addition), expected);
	}

	#[test]
	fn add_larger_sample() {
		let original:AudioBuffer = AudioBuffer::new((0..100).map(|index| index as f32 * 0.001).collect(), 1, 1000);
		let addition:AudioBuffer = AudioBuffer::new((0..200).map(|index| index as f32 * 0.002).collect(), 1, 1000);
		let expected:AudioBuffer = AudioBuffer::new((0..200).map(|index| if index < 100 { index as f32 * 0.003 } else { index as f32 * 0.002 }).collect(), 1, 1000);
		assert_eq!(original.combined_with(addition), expected);
	}

	#[test]
	fn add_badly_sampled_sample() {
		let original:AudioBuffer = AudioBuffer::new((0..100).map(|index| index as f32 * 0.001).collect(), 1, 1000);
		let addition:AudioBuffer = AudioBuffer::new((0..200).map(|index| [index as f32 * 0.001; 3]).flatten().collect(), 3, 2000);
		let expected:AudioBuffer = AudioBuffer::new((0..100).map(|index| index as f32 * 0.003).collect(), 1, 1000);
		assert_eq!(original.combined_with(addition), expected);
	}

	#[test]
	fn add_loud_sample() {
		let original:AudioBuffer = AudioBuffer::new((0..100).map(|index|  index as f32 * 0.01 * if index % 2 == 0 { -1.0 } else { 1.0 }).collect(), 1, 1000);
		let addition:AudioBuffer = AudioBuffer::new((0..100).map(|index|  index as f32 * 0.02 * if index % 2 == 0 { -1.0 } else { 1.0 }).collect(), 1, 1000);
		let expected:AudioBuffer = AudioBuffer::new((0..100).map(|index| (index as f32 * 0.03 * if index % 2 == 0 { -1.0 } else { 1.0 }).min(1.0).max(-1.0)).collect(), 1, 1000);
		assert_eq!(original.combined_with(addition), expected);
	}

	#[test]
	fn add_multiple_simples() {
		let original:AudioBuffer = AudioBuffer::new((0..100).map(|index| index as f32 * 0.001).collect(), 1, 1000);
		let addition_a:AudioBuffer = AudioBuffer::new((0..100).map(|index| index as f32 * 0.002).collect(), 1, 1000);
		let addition_b:AudioBuffer = AudioBuffer::new((0..100).map(|index| index as f32 * 0.003).collect(), 1, 1000);
		let addition_c:AudioBuffer = AudioBuffer::new((0..100).map(|index| index as f32 * 0.004).collect(), 1, 1000);
		let expected:AudioBuffer = AudioBuffer::new((0..100).map(|index| index as f32 * 0.01).collect(), 1, 1000);
		assert_eq!(original.combined_with(vec![addition_a, addition_b, addition_c]), expected);
	}
}