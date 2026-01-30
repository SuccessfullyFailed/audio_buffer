use crate::{ AudioBuffer, AudioEffect, AudioSettings };



const SETTING_VOLUME_MULTIPLIER:&str = "volume_multiplier";
const SETTING_VOLUME_TARGET:&str = "volume_target";



pub struct AudioAmplifier {
	settings:AudioSettings
}
impl AudioAmplifier {

	/// Create a new amplifier.
	pub fn new(volume_multiplier:f32) -> AudioAmplifier {
		AudioAmplifier {
			settings: AudioSettings::new((SETTING_VOLUME_MULTIPLIER, volume_multiplier))
		}
	}

	/// Create an amplifier that amplifies to the target volume.
	pub fn new_maximizer(target_volume:f32) -> AudioAmplifier {
		AudioAmplifier {
			settings: AudioSettings::new((SETTING_VOLUME_TARGET, target_volume))
		}
	}

	/// Amplify raw data by the given volume multiplier.
	fn amplify_raw_data(data:&mut [f32], volume_multiplier:f32) {
		if volume_multiplier != 1.0 {
			data.iter_mut().for_each(|sample| *sample *= volume_multiplier);
		}
	}
}
impl AudioEffect for AudioAmplifier {


	/// Apply the effect to an audio sample.
	fn apply_to(&mut self, sample:&mut AudioBuffer) {
		let mut volume_scale:f32 = 1.0;

		if let Some(target_volume) = self.settings.get::<f32>(SETTING_VOLUME_TARGET) {
			if let Some(current_max_volume) = sample.data().into_iter().max_by(|a, b| a.abs().partial_cmp(&b.abs()).unwrap()) {
				volume_scale *= target_volume / current_max_volume;
			}
		}
		if let Some(volume_multiplier) = self.settings.get::<f32>(SETTING_VOLUME_MULTIPLIER) {
			volume_scale *= volume_multiplier;
		}

		Self::amplify_raw_data(&mut sample.data, volume_scale);
	}

	/// Get the settings.
	fn settings(&self) -> &AudioSettings {
		&self.settings
	}

	/// Get the settings mutably.
	fn settings_mut(&mut self) -> &mut AudioSettings {
		&mut self.settings
	}
}


impl AudioBuffer {

	/// Return the audio with the volume multiplied by the given amount.
	pub fn amplified(mut self, volume_multiplier:f32) -> Self {
		self.amplify(volume_multiplier);
		self
	}

	/// Multiply the volume of the audio buffer by the given multiplier.
	pub fn amplify(&mut self, volume_multiplier:f32) {
		AudioAmplifier::amplify_raw_data(&mut self.data, volume_multiplier);
	}
}