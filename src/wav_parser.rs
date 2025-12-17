use bytes_parser::BytesParser;
use file_ref::FileRef;
use std::error::Error;



const RIFF_IDENTIFIER:[u8; 4] = [0x52, 0x49, 0x46, 0x46];
const WAVE_IDENTIFIER:[u8; 4] = [0x57, 0x41, 0x56, 0x45];
const JUNK_IDENTIFIER:[u8; 4] = [0x4A, 0x55, 0x4E, 0x4B];
const DATA_FORMAT_IDENTIFIER:[u8; 4] = [0x66, 0x6D, 0x74, 0x20];
const SAMPLED_DATA_IDENTIFIER:[u8; 4] = [0x64, 0x61, 0x74, 0x61];



struct DataFormat {
	block_size:u32,
	audio_format:u16,  // 1: PCM integer, 3: IEEE 754 float
	channel_count:u16,
	sample_rate:u32
}
enum AudioData { I16(Vec<i16>), F32(Vec<f32>) }



pub struct Wav {
	parser:BytesParser,

	data_format:Option<DataFormat>,
	audio_chunks:Vec<AudioData>
}
impl Wav {

	/* FILE METHODS METHODS */

	/// Parse a Wav from a file.
	pub fn from_file(file_path:&str) -> Result<Wav, Box<dyn Error>> {
		
		// Create parser.
		let mut wav:Wav = Wav {
			parser: BytesParser::new(
				FileRef::new(file_path).read_bytes()?,
				false
			),

			data_format: None,
			audio_chunks: Vec::new()
		};

		// Parse Master RIFF and WAVE identifier.
		if wav.parser.take::<[u8; 4]>()? != RIFF_IDENTIFIER {
			return Err("RIFF identifier not found.".into());
		}
		let _file_size:u32 = wav.parser.take()?;
		if wav.parser.take::<[u8; 4]>()? != WAVE_IDENTIFIER {
			return Err("WAVE identifier not found.".into());
		}

		// Keep parsing chunks as long as possible.
		while wav.parse_any_chunk()? {}

		// Return full wav.
		Ok(wav)
	}

	/// Save the Wav to a file.
	pub fn to_file(&self, file_path:&str) -> Result<(), Box<dyn Error>> {

		let data_format_chunk:Vec<u8> = self.data_format.as_ref().map(|data_format| {
			let bits_per_sample:u16 = match data_format.audio_format { 1 => 2, 3 => 4, _ => 0 } * 8; // u16, f32, ?
			let bytes_per_block:u16 = data_format.channel_count * bits_per_sample / 8;
			let bytes_per_second:u32 = data_format.sample_rate * bytes_per_block as u32;
			[
				DATA_FORMAT_IDENTIFIER.to_vec(),
				data_format.block_size.to_le_bytes().to_vec(),
				data_format.audio_format.to_le_bytes().to_vec(),
				data_format.channel_count.to_le_bytes().to_vec(),
				data_format.sample_rate.to_le_bytes().to_vec(),
				bytes_per_second.to_le_bytes().to_vec(),
				bytes_per_block.to_le_bytes().to_vec(),
				bits_per_sample.to_le_bytes().to_vec()
			].into_iter().flatten().collect()
		}).unwrap_or_default();

		let audio_data_chunks:Vec<Vec<u8>> = self.audio_chunks.iter().map(|audio_chunk|
			[
				SAMPLED_DATA_IDENTIFIER.to_vec(),
				match audio_chunk {
					AudioData::I16(items) => [
						((items.len() * 2) as u32).to_le_bytes().to_vec(),
						items.iter().map(|item| item.to_le_bytes()).flatten().collect::<Vec<u8>>()
					],
					AudioData::F32(items) => [
						((items.len() * 4) as u32).to_le_bytes().to_vec(),
						items.iter().map(|item| item.to_le_bytes()).flatten().collect::<Vec<u8>>()
					]
				}.into_iter().flatten().collect()
			]
		).flatten().collect();

		let master_riff_chunk:Vec<u8> = [
			RIFF_IDENTIFIER.to_vec(),
			((
				data_format_chunk.len() +
				audio_data_chunks.iter().map(|chunk| chunk.len()).sum::<usize>() + 
				WAVE_IDENTIFIER.len()
			) as u32).to_le_bytes().to_vec(),
			WAVE_IDENTIFIER.to_vec()
		].into_iter().flatten().collect();

		let total_bytes:Vec<u8> = [
			vec![master_riff_chunk],
			vec![data_format_chunk],
			audio_data_chunks
		].into_iter().flatten().flatten().collect();

		FileRef::new(file_path).write_bytes(&total_bytes)
	}



	/* PARSING METHODS */

	/// Try to parse any chunk. Returns true if a chunk was successfully parsed and added.
	fn parse_any_chunk(&mut self) -> Result<bool, Box<dyn Error>> {
		Ok(
			self.parse_junk_chunk()? ||
			self.parse_data_format_chunk()? ||
			self.parse_sampled_data()?
		)
	}

	/// Try to parse a junk chunk. Returns true if a junk chunk was parsed and found.
	fn parse_junk_chunk(&mut self) -> Result<bool, Box<dyn Error>> {
		if self.parser.take_bytes_conditional(4, |bytes| bytes == JUNK_IDENTIFIER)?.is_some() {
			let junk_size:u32 = self.parser.take()?;
			self.parser.skip(junk_size as usize);
			Ok(true)
		} else {
			Ok(false)
		}
	}

	/// Try to parse the Main RIFF. Returns true if the chunk was parsed and added.
	fn parse_data_format_chunk(&mut self) -> Result<bool, Box<dyn Error>> {
		if self.parser.take_bytes_conditional(4, |bytes| bytes == DATA_FORMAT_IDENTIFIER)?.is_some() {
			self.data_format = Some(DataFormat {
				block_size: self.parser.take()?,
				audio_format: self.parser.take()?,
				channel_count: self.parser.take()?,
				sample_rate: self.parser.take()?
			});
			let _bytes_per_second:u32 = self.parser.take()?;
			let _bytes_per_block:u16 = self.parser.take()?;
			let _bits_per_sample:u16 = self.parser.take()?;
			Ok(true)
		} else {
			Ok(false)
		}
	}

	/// Try to parse actual audio data. Returns true if the chunk was parsed and added.
	fn parse_sampled_data(&mut self) -> Result<bool, Box<dyn Error>> {
		if self.parser.take_bytes_conditional(4, |bytes| bytes == SAMPLED_DATA_IDENTIFIER)?.is_some() {

			// Get audio format.
			let audio_format:Option<u16> = self.data_format.as_ref().map(|data_format| data_format.audio_format);
			if audio_format.is_none() {
				return Err("Could not parse Wav data as the audio format is unknown.".into());
			}
			let audio_format:u16 = audio_format.unwrap();

			// Parse and store the audio data.
			let chunk_size:u32 = self.parser.take()?;
			let data_bytes:Vec<u8> = self.parser.take_bytes(chunk_size as usize)?;
			self.audio_chunks.push(
				match audio_format {
					1 => AudioData::I16(data_bytes.chunks(2).map(|bytes| i16::from_le_bytes((*bytes).try_into().unwrap())).collect()),
					3 => AudioData::F32(data_bytes.chunks(4).map(|bytes| f32::from_le_bytes((*bytes).try_into().unwrap())).collect()),
					_ => return Err(format!("Could not parse audio. Unknown audio format ID: {audio_format}").into())
				}
			);
			
			// Return success.
			Ok(true)
		} else {
			Ok(false)
		}
	}
}