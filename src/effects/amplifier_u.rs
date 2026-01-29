#[cfg(test)]
mod test {
	use crate::AudioBuffer;



	#[test]
	fn amplify_simple() {
		assert_eq!(
			AudioBuffer::new((0..100).map(|index| index as f32).collect(), 1, 1000).amplified(2.0).data,
			(0..100).map(|index| index as f32 * 2.0).collect::<Vec<f32>>()
		);
	}

	#[test]
	fn amplify_negative() {
		assert_eq!(
			AudioBuffer::new((0..100).map(|index| index as f32).collect(), 1, 1000).amplified(-1.0).data,
			(0..100).map(|index| index as f32 * -1.0).collect::<Vec<f32>>()
		);
	}

	#[test]
	fn amplify_negative_switch() {
		assert_eq!(
			AudioBuffer::new((0..100).map(|index| index as f32 * if index & 1 == 1 { -1.0 } else { 1.0 }).collect(), 1, 1000).amplified(-5.0).data,
			(0..100).map(|index| index as f32 * if index & 1 == 1 { 5.0 } else { -5.0 }).collect::<Vec<f32>>()
		);
	}
}