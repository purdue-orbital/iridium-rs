use serialport;
use anyhow::Result;

use std::time::Duration;

use iridium_rs::tele_dongle::*;

const BAUD: u32 = 38400;

fn main() -> Result<()> {
	let mut dongle = TeleDongle::new()?;

	dbg!(&dongle);
	
	loop {
		let mut buf = vec![0_u8; 1];

		match dongle.port.read_exact(&mut buf) {
			Err(_) => {
				eprintln!("ERROR");
			},
			_ => {
				let s = String::from_utf8(buf).unwrap_or_default();
				print!("{}", s);
			}
		}
	}
}