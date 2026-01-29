#[cfg(test)]
mod test {
	use crate::AudioBuffer;



	#[test]
	fn drain_data() {
		let mut buffer:AudioBuffer = AudioBuffer::new((0..100).map(|index| index as f32).collect(), 1, 1000);
		assert_eq!(buffer.data.len(), 100);
		
		assert_eq!(buffer.drain_data(0..10), (0..10).map(|index| index as f32).collect::<Vec<f32>>());
		assert_eq!(buffer.data.len(), 90);

		assert_eq!(buffer.drain_data(10..20), (0..10).map(|index| (20 + index) as f32).collect::<Vec<f32>>());
		assert_eq!(buffer.data.len(), 80);
	}
}