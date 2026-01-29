use bytes_parser::BytesParser;
use crate::AudioBuffer;
use file_ref::FileRef;
use std::error::Error;



const MAX_CHUNK_BYTES:u32 = 1024 * 1024 * 1024; // 1GB
const DEFAULT_SAMPLE_RATE:u32 = 48_000;
const DEFAULT_CHANEL_COUNT:usize = 2;
pub(super) const IS_BIG_ENDIAN:bool = false;


pub(super) const RIFF_IDENTIFIER:[u8; 4] = [0x52, 0x49, 0x46, 0x46];
pub(super) const WAVE_IDENTIFIER:[u8; 4] = [0x57, 0x41, 0x56, 0x45];
pub(super) const JUNK_IDENTIFIER:[u8; 4] = [0x4A, 0x55, 0x4E, 0x4B];
pub(super) const DATA_FORMAT_IDENTIFIER:[u8; 4] = [0x66, 0x6D, 0x74, 0x20];
pub(super) const SAMPLED_DATA_IDENTIFIER:[u8; 4] = [0x64, 0x61, 0x74, 0x61];



pub(super) struct DataFormat {
	audio_format:u16, // 1: PCM integer, 3: IEEE 754 float
	channel_count:u16,
	sample_rate:u32
}

impl AudioBuffer {

	/// Create an audio-buffer from a wav file.
	pub fn from_wav(file_path:&str) -> Result<AudioBuffer, Box<dyn Error>> {
		AudioBuffer::from_wav_bytes(FileRef::new(file_path).read_bytes()?)
	}

	/// Create an audio-buffer from wav-contents.
	pub(crate) fn from_wav_bytes(bytes:Vec<u8>) -> Result<AudioBuffer, Box<dyn Error>> {
		let mut parser:BytesParser = BytesParser::new(bytes, IS_BIG_ENDIAN);
		let mut data_format:Option<DataFormat> = None;
		let mut audio_data:Vec<f32> = Vec::new();
		
		// Parse Master RIFF and WAVE identifier.
		if parser.take::<[u8; 4]>()? != RIFF_IDENTIFIER {
			return Err("RIFF identifier not found.".into());
		}
		let _file_size:u32 = parser.take()?;
		if parser.take::<[u8; 4]>()? != WAVE_IDENTIFIER {
			return Err("WAVE identifier not found.".into());
		}

		// Keep parsing chunks as long as possible.
		while Self::parse_any_chunk(&mut parser, &mut data_format, &mut audio_data)? {}

		// Return full wav.
		Ok(AudioBuffer::new(
			audio_data,
			data_format.as_ref().map(|format| format.channel_count as usize).unwrap_or(DEFAULT_CHANEL_COUNT),
			data_format.as_ref().map(|format| format.sample_rate).unwrap_or(DEFAULT_SAMPLE_RATE)
		))
	}

	/// Store the audio buffer to a WAV.
	pub fn to_wav(&self, file_path:&str) -> Result<(), Box<dyn Error>> {
		
		// DataFormat block.
		let audio_format:u16 = 3;
		let channel_count:u16 = self.channel_count as u16;
		let sample_rate:u32 = self.sample_rate;
		let bits_per_sample:u16 = 4 * 8; // f32 has 4 bytes
		let bytes_per_block:u16 = self.channel_count as u16 * bits_per_sample / 8;
		let bytes_per_second:u32 = self.sample_rate * bytes_per_block as u32;
		let data_format_chunk:Vec<u8> = [
			DATA_FORMAT_IDENTIFIER.to_vec(),
			16_u32.to_le_bytes().to_vec(),
			audio_format.to_le_bytes().to_vec(),
			channel_count.to_le_bytes().to_vec(),
			sample_rate.to_le_bytes().to_vec(),
			bytes_per_second.to_le_bytes().to_vec(),
			bytes_per_block.to_le_bytes().to_vec(),
			bits_per_sample.to_le_bytes().to_vec()
		].into_iter().flatten().collect();

		// Audio data chunks.
		let block_size:u32 = (self.data.len() as u32 * 4).min(MAX_CHUNK_BYTES);
		let audio_data_chunks:Vec<Vec<u8>> = self.data.chunks(block_size as usize).map(|audio_chunk| [
			SAMPLED_DATA_IDENTIFIER.to_vec(),
			((audio_chunk.len() * 4) as u32).to_le_bytes().to_vec(),
			audio_chunk.iter().map(|item| item.to_le_bytes()).flatten().collect::<Vec<u8>>()
		]).flatten().collect();

		// Master riff chunk.
		let master_riff_chunk:Vec<u8> = [
			RIFF_IDENTIFIER.to_vec(),
			((
				data_format_chunk.len() +
				audio_data_chunks.iter().map(|chunk| chunk.len()).sum::<usize>() + 
				WAVE_IDENTIFIER.len()
			) as u32).to_le_bytes().to_vec(),
			WAVE_IDENTIFIER.to_vec()
		].into_iter().flatten().collect();

		// Combine chunks and write to file.
		let total_bytes:Vec<u8> = [
			vec![master_riff_chunk],
			vec![data_format_chunk],
			audio_data_chunks
		].into_iter().flatten().flatten().collect();
		FileRef::new(file_path).write_bytes(&total_bytes)
	}



	/* PARSING METHODS */

	/// Try to parse any chunk. Returns true if a chunk was successfully parsed and added.
	pub(super) fn parse_any_chunk(parser:&mut BytesParser, data_format:&mut Option<DataFormat>, audio_data:&mut Vec<f32>) -> Result<bool, Box<dyn Error>> {
		if let Ok(identifier) = parser.take::<[u8; 4]>() {
			let block_size:u32 = parser.take::<u32>()?;
			let mut sub_parser:BytesParser = BytesParser::new(parser.take_bytes(block_size as usize)?, IS_BIG_ENDIAN);
			if block_size & 1 == 1 {
				parser.skip(1);
			}
			match identifier {
				DATA_FORMAT_IDENTIFIER => Self::parse_data_format_chunk(&mut sub_parser, data_format),
				SAMPLED_DATA_IDENTIFIER => Self::parse_sampled_data(&mut sub_parser, data_format, audio_data),
				JUNK_IDENTIFIER => Ok(true),
				_ => Ok(false)
			}
		} else {
			Ok(false)
		}
	}

	/// Try to parse the Main RIFF. Returns true if the chunk was parsed and added.
	fn parse_data_format_chunk(parser:&mut BytesParser, data_format:&mut Option<DataFormat>) -> Result<bool, Box<dyn Error>> {
		let audio_format:u16 = parser.take()?; // 1: u16, 3: f32
		let channel_count:u16 = parser.take()?;
		let sample_rate:u32 = parser.take()?;
		*data_format = Some(DataFormat { audio_format, channel_count, sample_rate });
		Ok(true)
	}

	/// Try to parse actual audio data. Returns true if the chunk was parsed and added.
	fn parse_sampled_data(parser:&mut BytesParser, data_format:&mut Option<DataFormat>, audio_data:&mut Vec<f32>) -> Result<bool, Box<dyn Error>> {
		const I16_TO_F32_SCALE:f32 = 1.0 / i16::MAX as f32;


		// Get audio format.
		let audio_format:Option<u16> = data_format.as_ref().map(|data_format| data_format.audio_format);
		if audio_format.is_none() {
			return Err("Could not parse Wav data as the audio format is unknown.".into());
		}
		let audio_format:u16 = audio_format.unwrap();
		let audio_format_sample_size:usize = match audio_format { 1 => 2, 3 => 4, _ => 1 };

		// Parse and store the audio data.
		let mut data_bytes:Vec<u8> = parser.take_remaining_bytes();
		while data_bytes.len() % audio_format_sample_size != 0 {
			data_bytes.remove(data_bytes.len() - 1);
		}
		match audio_format {
			1 => audio_data.extend(data_bytes.chunks(audio_format_sample_size).map(|bytes| i16::from_le_bytes((*bytes).try_into().unwrap()) as f32 * I16_TO_F32_SCALE).collect::<Vec<f32>>()),
			3 => audio_data.extend(data_bytes.chunks(audio_format_sample_size).map(|bytes| f32::from_le_bytes((*bytes).try_into().unwrap())).collect::<Vec<f32>>()),
			_ => return Err(format!("Could not parse audio. Unknown audio format ID: {audio_format}").into())
		}
		
		// Return success.
		Ok(true)
	}
}