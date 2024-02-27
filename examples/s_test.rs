use serialport;
use anyhow::Result;

use std::time::Duration;

use iridium_rs::tele_dongle::*;

const BAUD: u32 = 38400;

fn main() -> Result<()> {
	let mut dongle = TeleDongle::new()?;

	dbg!(&dongle);
	
	loop {
		println!("{}", dongle.read_line());
	}
}