use std::error::Error;
use file_ref::FileRef;



const GENERATED_EFFECT_SETTINGS_FILE:FileRef = FileRef::new_const("src/effect/effect_settings_generated.rs");
const GENERATED_EFFECT_SETTINGS_FILE_MIN_SIZE:u64 = 128;




fn main() -> Result<(), Box<dyn Error>> {

	if GENERATED_EFFECT_SETTINGS_FILE.exists() && GENERATED_EFFECT_SETTINGS_FILE.bytes_size() < GENERATED_EFFECT_SETTINGS_FILE_MIN_SIZE {
		GENERATED_EFFECT_SETTINGS_FILE.write(
			format!(
				"
					use super::{{ AudioSettingsSourceList, AudioSetting }};
					use byte_convertible::ByteConvertible;

					{}
				",
				(0..80).map(|arg_count| 
					format!(
						"
							impl<{}> AudioSettingsSourceList for ({}) {{
								fn to_settings(self) -> Vec<AudioSetting> {{
									vec![{}]
								}}
							}}
						",
						(0..arg_count).map(|arg_index| arg_index_to_name(arg_index)).map(|generic_name| format!("{generic_name}:ByteConvertible, ")).collect::<Vec<String>>().join(""),
						(0..arg_count).map(|arg_index| arg_index_to_name(arg_index)).map(|generic_name| format!("(&str, {generic_name}), ")).collect::<Vec<String>>().join(""),
						(0..arg_count).map(|arg_index| format!("AudioSetting::new(self.{arg_index}.0, self.{arg_index}.1)")).collect::<Vec<String>>().join(", ")
					)
				).collect::<Vec<String>>().join("\n")
			).trim().replace("\t", "")
		)?;
	}

	Ok(())
}



fn arg_index_to_name(arg_index:usize) -> String {
	if arg_index < 26 {
		(('A' as u8 + arg_index as u8) as char).to_string()
	} else {
		arg_index_to_name(arg_index / 26 - 1) + &arg_index_to_name(arg_index % 26)
	}
}