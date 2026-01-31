#[cfg(test)]
mod test {
	use crate::{ NoiseGate, AudioBuffer, AudioEffect };



	#[test]
	fn noise_gate_ramp_lower_threshold_accurate() {
		let mut calculated_buffer:AudioBuffer = AudioBuffer::new((0..100).map(|index| index as f32).collect(), 1, 100);
		let mut noise_gate:NoiseGate = NoiseGate::new(20.0, 1000.0, 1, 1);
		noise_gate.apply_to(&mut calculated_buffer);

		let expected_buffer:AudioBuffer = AudioBuffer::new((0..100).map(|index| if index <= 20 { 0.0 } else { index as f32 }).collect::<Vec<f32>>(), 1, 100);

		calculated_buffer.assert_similar(&expected_buffer, 0.01);
	}

	#[test]
	fn noise_gate_ramp_upper_limit_accurate() {
		let mut calculated_buffer:AudioBuffer = AudioBuffer::new((0..100).map(|index| index as f32).collect(), 1, 100);
		let mut noise_gate:NoiseGate = NoiseGate::new(0.0, 50.0, 1, 1);
		noise_gate.apply_to(&mut calculated_buffer);

		let expected_buffer:AudioBuffer = AudioBuffer::new((0..100).map(|index| (index as f32).min(50.0)).collect::<Vec<f32>>(), 1, 100);

		calculated_buffer.assert_similar(&expected_buffer, 0.01);
	}

	#[test]
	fn noise_gate_ramp_upper_limit_handles_zero() {
		let mut calculated_buffer:AudioBuffer = AudioBuffer::new((0..100).map(|index| index as f32).collect(), 1, 100);
		let mut noise_gate:NoiseGate = NoiseGate::new(0.0, 0.0, 1, 1);
		noise_gate.apply_to(&mut calculated_buffer);

		let expected_buffer:AudioBuffer = AudioBuffer::new(vec![0.0; 100], 1, 100);

		calculated_buffer.assert_similar(&expected_buffer, 0.01);
	}

	#[test]
	fn noise_gate_ramp_duration_accurate() {
		let mut calculated_buffer:AudioBuffer = AudioBuffer::new(vec![1.0; 200], 1, 100);
		let mut noise_gate:NoiseGate = NoiseGate::new(0.0, 1.0, 1000, 1);
		noise_gate.apply_to(&mut calculated_buffer);

		let expected_buffer:AudioBuffer = AudioBuffer::new((0..200).map(|index| ((index + 1) as f32 / 100.0).min(1.0)).collect::<Vec<f32>>(), 1, 100);

		calculated_buffer.assert_similar(&expected_buffer, 0.01);
	}

	#[test]
	fn noise_gate_ramp_duration_handles_zero() {
		let mut calculated_buffer:AudioBuffer = AudioBuffer::new(vec![1.0; 10], 1, 100);
		let mut noise_gate:NoiseGate = NoiseGate::new(0.0, 1.0, 0, 1);
		noise_gate.apply_to(&mut calculated_buffer);

		let expected_buffer:AudioBuffer = AudioBuffer::new(vec![1.0; 10], 1, 100);

		calculated_buffer.assert_similar(&expected_buffer, 0.01);
	}

	#[test]
	fn noise_gate_batch_size_accurate() {
		let mut calculated_buffer:AudioBuffer = AudioBuffer::new(vec![1.0; 100], 1, 100);
		let mut noise_gate:NoiseGate = NoiseGate::new(0.0, 1000.0, 1000, 25);
		noise_gate.apply_to(&mut calculated_buffer);

		let expected_buffer:AudioBuffer = AudioBuffer::new((0..100).map(|index| ((index + 1) as f32 / 25.0).ceil() * 0.25).collect::<Vec<f32>>(), 1, 100);

		calculated_buffer.assert_similar(&expected_buffer, 0.01);
	}

	#[test]
	fn noise_gate_batch_size_handle_zero() {
		let mut calculated_buffer:AudioBuffer = AudioBuffer::new(vec![1.0; 100], 1, 100);
		let mut noise_gate:NoiseGate = NoiseGate::new(0.0, 1000.0, 1000, 0);
		noise_gate.apply_to(&mut calculated_buffer);

		let expected_buffer:AudioBuffer = AudioBuffer::new(vec![1.0; 100], 1, 100);

		calculated_buffer.assert_similar(&expected_buffer, 0.01);
	}
}