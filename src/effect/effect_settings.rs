use byte_convertible::ByteConvertible;



pub struct AudioSettings(Vec<AudioSetting>);
impl AudioSettings {

	/* CONSTRUCTOR METHODS */

	/// Create a new settings set.
	pub fn new<T:AudioSettingsSourceList>(settings:T) -> AudioSettings {
		AudioSettings(settings.to_settings())
	}



	/* SETTING GETTER METHODS */

	/// Get a setting by name.
	pub fn get<T:ByteConvertible>(&self, name:&str) -> Option<T> {
		self.0.iter().find(|setting| setting.name == name).map(|setting| T::from_bytes(&setting.value)).flatten()
	}

	/// Get a setting by name or return the given value.
	pub fn get_or<T:ByteConvertible>(&self, name:&str, or:T) -> T {
		self.get(name).unwrap_or(or)
	}

	/// Set a setting by name.
	pub fn set<T:ByteConvertible>(&mut self, name:&str, value:T) {
		match self.0.iter_mut().find(|setting| setting.name == name) {
			Some(setting) => setting.value = value.as_bytes(),
			None => self.0.push(AudioSetting::new(name, value))
		}
	}

	/// Set multiple settings by name.
	pub fn set_m<T:AudioSettingsSourceList>(&mut self, settings:T) {
		for setting in settings.to_settings(){
			self.set(&setting.name, setting.value);
		}
	}
}
impl Default for AudioSettings {
	fn default() -> Self {
		AudioSettings(Vec::new())
	}
}



pub struct AudioSetting {
	name:String,
	value:Vec<u8>
}
impl AudioSetting {

	/// Create a new setting.
	pub fn new<T:ByteConvertible>(name:&str, value:T) -> AudioSetting {
		AudioSetting {
			name: name.to_string(),
			value: value.as_bytes()
		}
	}
}



pub trait AudioSettingsSourceList {
	fn to_settings(self) -> Vec<AudioSetting>;
}
impl<T:ByteConvertible> AudioSettingsSourceList for (&str, T) {
	fn to_settings(self) -> Vec<AudioSetting> {
		vec![AudioSetting::new(self.0, self.1)]
	}
}
impl<T:ByteConvertible> AudioSettingsSourceList for Vec<(&str, T)> {
	fn to_settings(self) -> Vec<AudioSetting> {
		self.into_iter().map(|(name, value)| AudioSetting::new(name, value)).collect()
	}
}