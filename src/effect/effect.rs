use crate::{ AudioBuffer, AudioSettings };



pub trait AudioEffect {

	/// Apply the effect to an audio buffer.
	fn apply_to(&mut self, buffer:&mut AudioBuffer);

	/// Get the settings.
	fn settings(&self) -> &AudioSettings;

	/// Get the settings mutably.
	fn settings_mut(&mut self) -> &mut AudioSettings;
}