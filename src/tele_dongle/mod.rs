use serialport;
use anyhow::Result;

use std::time::Duration;
use std::boxed::Box;
use std::io::{self, Read};

pub mod message;
pub mod telem;

#[derive(Debug)]
pub struct TeleDongle {
	pub port: Box<dyn serialport::SerialPort>,
	pub buf: Vec<u8>,
}

impl TeleDongle {
	const VID: u16 = 65534;
	const PID: u16 = 12;
	const BAUD: u32 = 38400;

	const TIMEOUT: Duration = Duration::from_millis(1500);

	const FREQUENCY: u32 = 434550; // TODO: check

	const BUF_LEN: usize = 100;

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

		let mut buf = vec![0_u8; 10_000];
		port.write(b"c E 0\n")?;
		port.write(format!("c F {}\n", Self::FREQUENCY).as_bytes())?;
		port.write(b"c T 0\n")?;
		port.write(b"m 20\n")?;


		let _ = port.read(&mut buf);

		Ok(Self {
			port,
			buf: Vec::with_capacity(Self::BUF_LEN)
		})
	}

	pub fn read_a_bit(&mut self) -> Option<String> {
		let mut byte: [u8; 1] = [0];

		if self.read(&mut byte).is_err() {
			return None;
		}

		if byte[0] == b'\n' {
			let s = String::from_utf8(self.buf.clone()).unwrap() ;
			self.buf.clear();
			return Some(s);
		} else {
			self.buf.push(byte[0]);
			return None;
		}
	}

	/// reads a line of output from the TeleDongle
	/// NOTE: this blocks untill a whole line is avaliable, which could be many seconds
	pub fn read_line(&mut self) -> String {
		let mut s = self.read_a_bit();

		while s.is_none() {
			s = self.read_a_bit();
		}

		s.unwrap()
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
