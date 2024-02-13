use serialport;
use anyhow::Result;

use std::time::Duration;
use std::boxed::Box;
use std::io;

pub mod commands;

#[derive(Debug)]
pub struct TeleDongle {
	pub port: Box<dyn serialport::SerialPort>
}

impl TeleDongle {
	const VID: u16 = 65534;
	const PID: u16 = 12;
	const BAUD: u32 = 38400;
	const TIMEOUT: Duration = Duration::from_millis(150);

	pub fn new() -> Result<Self> {
		let mut ports = serialport::available_ports().expect("No ports found!");

		ports.retain(|port| {
			match &port.port_type {
				serialport::SerialPortType::UsbPort(info) => {
					(info.vid == Self::VID) && (info.pid == Self::PID)
				},
				_ => false
			}
		});

		let port_info = ports.first().expect("no teledongle detected!!!");

		let mut port = serialport::new(port_info.port_name.clone(), Self::BAUD)
			.timeout(Self::TIMEOUT)
			.open().expect("couldn't open port");

		// let mut buf = vec![0_u8; 10_000];
		port.write(b"~\nE 0\nm 0\n")?;
		port.write(b"c s\nc T 1\nm 20\nm 20\nc s\n")?;

		// let _ = port.read(&mut buf);

		Ok(Self {
			port
		})
	}

	// decodes 36 bytes worth of a hex string into bytes
	pub fn hex_string_to_bytes(input: &str) -> Vec<u8> {
		let mut buf: Vec<u8> = vec![0; 36];

		hex::decode_to_slice(input, &mut buf).expect("couldn't convert TELEM hex string to bytes");

		buf
	}
}

impl io::Write for TeleDongle {
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		self.port.write(buf)
	}

	fn flush(&mut self) -> io::Result<()> {
		self.port.flush()
	}
}

impl io::Read for TeleDongle {
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
		self.port.read(buf)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use bytes::{BufMut, BytesMut};

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
		let ans = TeleDongle::hex_string_to_bytes(&input_string);

		assert_eq!(ans, real_ans.to_vec());
	}
}
