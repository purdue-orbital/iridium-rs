use logos::{Logos, Lexer};

#[derive(Logos, Debug, PartialEq)]
pub enum Message {
	#[token("ERROR")]
	Error,

	// regex to match a telem string
	#[regex("TELEM [0-9a-fA-F]{72}", Message::telem)]
	Telemetry(Vec<u8>),
}

impl Message {
	const LEN: usize = 36;

	// decodes 36 bytes worth of a hex string into bytes
	fn hex_string_to_bytes(input: &str) -> Vec<u8> {
		let mut buf: Vec<u8> = vec![0; Self::LEN];

		hex::decode_to_slice(input, &mut buf).expect("couldn't convert TELEM hex string to bytes");

		buf
	}

	fn telem(lex: &mut Lexer<Self>) -> Option<Vec<u8>> {
		let slice = lex.slice();
		let s = &slice[6..];

		Some(Self::hex_string_to_bytes(s))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::tele_dongle::telem::TeleDonglePacket;

	use bytes::{BufMut, BytesMut};
	use std::fs;

	#[test]
	fn hexstring_test_01() {
		let mut real_ans = BytesMut::with_capacity(36);
		real_ans.put_u64(0x224c14ac021409e4_u64);
		real_ans.put_u64(0xff66850100c80a54_u64);
		real_ans.put_u64(0x055500da000300fa_u64);
		real_ans.put_u64(0xff0100f901390126_u64);
		real_ans.put_u32(0x056e86fe_u32);

		let input_bytes = include_bytes!("telem_1_example.bin").to_vec();
		let input_string = String::from_utf8(input_bytes.clone()).unwrap();
		let ans = Message::hex_string_to_bytes(&input_string);

		assert_eq!(ans, real_ans.to_vec());
	}

	#[test]
	fn read_file() {
		let contents = fs::read_to_string("2024-01-17-serial-5196-flight-0004-via-5196.telem").unwrap();

		let mut lex = Message::lexer(&contents);
		let mut record = vec![0_u64; 256];

		for each in lex {
			if each.is_err() {
			} else {
				match each.unwrap() {
					Message::Telemetry(arr) => {
						let packet: TeleDonglePacket = arr.into();

						println!("{}", packet.packet_type);
						dbg!(packet.parse_payload());

						record[packet.packet_type as usize] += 1;
					},
					_ => panic!()
				}
			}
		}

		for pair in record.iter().enumerate() {
			if pair.1 != &0 {
				// dbg!(pair);
			}
		}

		assert!(false);
	}
}
