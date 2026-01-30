use crate::{ AudioBuffer, AudioSettings };



pub trait AudioEffect {

	/// Apply the effect to an audio sample.
	fn apply_to(&mut self, sample:&mut AudioBuffer);

	/// Get the settings.
	fn settings(&self) -> &AudioSettings;

	/// Get the settings mutably.
	fn settings_mut(&mut self) -> &mut AudioSettings;
}