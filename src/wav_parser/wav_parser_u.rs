#[cfg(test)]
mod tests {
	use crate::{ AudioBuffer, wav_parser::wav_parser::{ DATA_FORMAT_IDENTIFIER, DataFormat, IS_BIG_ENDIAN, JUNK_IDENTIFIER, RIFF_IDENTIFIER, SAMPLED_DATA_IDENTIFIER, WAVE_IDENTIFIER } };
	use bytes_parser::BytesParser;


	const RIFF_HEADER_SIZE: usize = 12; // "RIFF" + size + "WAVE".
	const FMT_CHUNK_SIZE: usize = 8 + 18; // Id + size + payload.
	const JUNK_CHUNK_SIZE: usize = 8 + 3 + 1; // Id + size + payload + pad.
	const DATA1_CHUNK_SIZE: usize = 8 + 4;
	const DATA2_CHUNK_SIZE: usize = 8 + 2 + 2; // Includes 1 byte padding, which should move the cursor two bytes.



	fn flawed_wav_bytes() -> Vec<u8> {
		let bytes:[&[u8]; 24] = [
			&RIFF_IDENTIFIER,
			&(65_u32).to_le_bytes(), // Temporary template.
			&WAVE_IDENTIFIER,

			// Data format chunk (usually 16 bytes, but this one is 18).
			&DATA_FORMAT_IDENTIFIER,
			&(18_u32).to_le_bytes(),
			&(1_u16).to_le_bytes(),		// PCM
			&(1_u16).to_le_bytes(),		// Mono
			&(48000_u32).to_le_bytes(),
			&(96000_u32).to_le_bytes(),
			&(2_u16).to_le_bytes(),		// Block align
			&(16_u16).to_le_bytes(),	// Bits
			&(0_u16).to_le_bytes(),		// Padding bytes

			// JUNK chunk with odd size (forced padding).
			&JUNK_IDENTIFIER,
			&(3_u32).to_le_bytes(),
			&[1, 2, 3],
			&[0],				// Padding byte

			// First data chunk (2 samples).
			&SAMPLED_DATA_IDENTIFIER,
			&(4_u32).to_le_bytes(),
			&(0_i16).to_le_bytes(),
			&(16384_i16).to_le_bytes(),

			// Second data chunk (1 sample).
			&SAMPLED_DATA_IDENTIFIER,
			&(3_u32).to_le_bytes(),
			&(-16384_i16).to_le_bytes(),
			&[0]				// Padding byte
		];
		bytes.into_iter().flatten().cloned().collect()
	}

	#[test]
	fn correctly_parses_wav() {
		let bytes:Vec<u8> = flawed_wav_bytes();
		let mut parser:BytesParser = BytesParser::new(bytes, IS_BIG_ENDIAN);
		let mut data_format:Option<DataFormat> = None;
		let mut audio:Vec<f32> = Vec::new();

		// Parse RIFF.
		assert_eq!(parser.take::<[u8;4]>().unwrap(), *b"RIFF");
		let _ = parser.take::<u32>().unwrap();
		assert_eq!(parser.take::<[u8;4]>().unwrap(), *b"WAVE");
		assert_eq!(parser.cursor(), RIFF_HEADER_SIZE);

		// Parse DataFormat.
		assert!(AudioBuffer::parse_any_chunk(&mut parser, &mut data_format, &mut audio).unwrap());
		assert_eq!(parser.cursor(), RIFF_HEADER_SIZE + FMT_CHUNK_SIZE);

		// Parse Junk (odd padded).
		assert!(AudioBuffer::parse_any_chunk(&mut parser, &mut data_format, &mut audio).unwrap());
		assert_eq!(parser.cursor(), RIFF_HEADER_SIZE + FMT_CHUNK_SIZE + JUNK_CHUNK_SIZE);

		// Parse first data chunk.
		assert!(AudioBuffer::parse_any_chunk(&mut parser, &mut data_format, &mut audio).unwrap());
		assert_eq!(parser.cursor(), RIFF_HEADER_SIZE + FMT_CHUNK_SIZE + JUNK_CHUNK_SIZE + DATA1_CHUNK_SIZE);

		// Parse second data chunk (odd padded).
		assert!(AudioBuffer::parse_any_chunk(&mut parser, &mut data_format, &mut audio).unwrap());
		assert_eq!(parser.cursor(), RIFF_HEADER_SIZE + FMT_CHUNK_SIZE + JUNK_CHUNK_SIZE + DATA1_CHUNK_SIZE + DATA2_CHUNK_SIZE);

		// Do not error when the data runs out.
		assert!(!AudioBuffer::parse_any_chunk(&mut parser, &mut data_format, &mut audio).unwrap());

		// Audio length correctness.
		assert_eq!(audio.len(), 3);
	}

}