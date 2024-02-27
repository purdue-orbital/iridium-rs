use bytes::{Buf, Bytes};
use std::io::{Cursor, Seek, SeekFrom};

#[derive(Debug)]
#[allow(dead_code)]
pub struct TeleDonglePacket {
	/// not really usefull tbh, but yeah
	pub length: u8,
	pub serial_number: u16,
	pub tick: u16,
	pub packet_type: u8,
	pub payload: Bytes,
	/// raw RSSI value. use `rssi_dBm()` to get the actual RSSI
	pub rssi: u8,
	lqi: u8,
	checksum: u8,
}

impl TeleDonglePacket {
	#[allow(non_snake_case)]
	pub fn rssi_dBm(&self) -> f64 {
		(self.rssi as f64) / 2.0 - 74.0
	}

	fn crc(bin: &[u8]) -> u8 {
		let mut sum: u64 = 0x5a;
	
		for b in &bin[1..35] {
		   // println!("{:#04x}", b);
		   sum += *b as u64;
		}

		sum = sum % 256;

		sum as u8
	}

	pub fn parse_payload(&self) -> todo!() {
		
	}
}

impl From<Vec<u8>> for TeleDonglePacket {
	fn from(value: Vec<u8>) -> Self {
		let payload_len = value.len() - 9;
		let crc = Self::crc(&value);
		let mut cursor = Cursor::new(value);

		cursor.seek(SeekFrom::Start(0)).unwrap();
		let length = cursor.get_u8();
		
		let serial_number = cursor.get_u16_ne();
		let tick = cursor.get_u16_ne();
		let packet_type = cursor.get_u8();
		
		cursor.seek(SeekFrom::End(-1)).unwrap();
		let checksum = cursor.get_u8();

		// check that no data was messed up between the dongle and computer
		assert_eq!(checksum, crc);

		cursor.seek(SeekFrom::End(-2)).unwrap();
		let lqi = cursor.get_u8();

		cursor.seek(SeekFrom::End(-3)).unwrap();
		let rssi = cursor.get_u8();

		cursor.seek(SeekFrom::Start(1)).unwrap();
		let payload = cursor.copy_to_bytes(payload_len);

		Self {
			length,
			payload,
			serial_number,
			tick,
			packet_type,
			rssi,
			lqi,
			checksum,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use bytes::{BufMut, BytesMut};

	#[test]
	#[ignore]
	fn dongle_packet_test() {
		let mut bin = BytesMut::with_capacity(36);
		bin.put_u64(0x224c14ac021409e4_u64);
		bin.put_u64(0xff66850100c80a54_u64);
		bin.put_u64(0x055500da000300fa_u64);
		bin.put_u64(0xff0100f901390126_u64);
		bin.put_u32(0x056e86fe_u32);

		let bin = bin.to_vec();
		let packet: TeleDonglePacket = bin.into();

		dbg!(&packet);



		assert!(false)
	}
}