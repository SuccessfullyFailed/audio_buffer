use crate::{ AudioBuffer, AudioEffect, AudioSettings };



const SETTING_MIN_VOLUME:&str = "lower_threshold";
const SETTING_MAX_VOLUME:&str = "upper_limit";
const SETTING_SHIFT_DURATION:&str = "zero_to_max_duration_ms";
const SETTING_BATCH_SIZE:&str = "batch_size";



pub struct NoiseGate {
	settings:AudioSettings,
	cursor:f32,

	cache:Option<NoiseGateCache>
}
impl NoiseGate {

	/// Create a new noise-gate. Dampens quiet and sudden spikes in audio while keeping continuous sounds.
	pub fn new(lower_threshold:f32, upper_limit:f32, zero_to_max_duration_ms:u64, batch_size:u32) -> NoiseGate {
		NoiseGate {
			settings: AudioSettings::new((
				(SETTING_MIN_VOLUME, lower_threshold),
				(SETTING_MAX_VOLUME, upper_limit),
				(SETTING_SHIFT_DURATION, zero_to_max_duration_ms),
				(SETTING_BATCH_SIZE, batch_size)
			)),
			cursor: 0.0,
			cache: None
		}
	}
}
impl AudioEffect for NoiseGate {

	/// Apply the effect to an audio buffer.
	fn apply_to(&mut self, buffer:&mut AudioBuffer) {

		// Update cache.
		let update_cache:bool = self.cache.as_ref().map(|cache| cache.sample_rate != buffer.sample_rate).unwrap_or(true);
		if update_cache {
			let batch_size:u32 = self.settings.get_or(SETTING_BATCH_SIZE, 100);
			self.cache = Some(NoiseGateCache {
				sample_rate: buffer.sample_rate,
				lower_threshold: self.settings.get_or(SETTING_MIN_VOLUME, 0.0),
				upper_limit: self.settings.get_or(SETTING_MAX_VOLUME, 1.0),
				batch_size,
				shift_per_batch: {
					let zero_to_max_duration_ms:u64 = self.settings.get_or(SETTING_SHIFT_DURATION, 100);
					let zero_to_max_sample_count:f32 = (zero_to_max_duration_ms as f32 / 1000.0) * buffer.sample_rate as f32;
					let shift_per_sample:f32 = 1.0 / zero_to_max_sample_count;
					let shift_per_batch:f32 = shift_per_sample * batch_size as f32;
					shift_per_batch
				}
			});
		}
		
		// Apply noise-gate effect using cache.
		if let Some(cache) = &self.cache {
			if cache.batch_size == 0 {
				return;
			}
			for batch in buffer.data.chunks_mut(cache.batch_size as usize) {

				// Modify cursor based on batch peak.
				let batch_max:f32 = batch.iter().max_by(|a, b| a.abs().partial_cmp(&b.abs()).unwrap()).unwrap_or(&0.0).abs();
				if batch_max > cache.lower_threshold && self.cursor < 1.0 {
					self.cursor = (self.cursor + cache.shift_per_batch).min(1.0);
				}
				if batch_max < cache.lower_threshold && self.cursor >= 0.0 {
					self.cursor = (self.cursor - cache.shift_per_batch).max(0.0);
				}

				// Calculate and apply scale for batch.
				let peak:f32 = self.cursor * batch_max;
				let scale:f32 = if peak < cache.upper_limit { self.cursor } else { 1.0 / batch_max * cache.upper_limit };
				for sample in batch {
					*sample *= scale;
				}
			}
		}
	}

	/// Get the settings.
	fn settings(&self) -> &AudioSettings {
		&self.settings
	}

	/// Get the settings mutably.
	fn settings_mut(&mut self) -> &mut AudioSettings {
		self.cache = None; // Assume settings are changed and cache is inaccurate.
		&mut self.settings
	}
}



struct NoiseGateCache {
	sample_rate:u32,
	lower_threshold:f32,
	upper_limit:f32,
	batch_size:u32,
	shift_per_batch:f32
}