use iridium_rs::tele_dongle::telem::TeleDonglePacket;
use logos::Logos;
use serialport;
use anyhow::Result;

use std::time::Duration;

use iridium_rs::tele_dongle::*;
use iridium_rs::tele_dongle::message::Message;

const BAUD: u32 = 38400;

fn main() -> Result<()> {
	let mut dongle = TeleDongle::new()?;

	dbg!(&dongle);
	
	loop {
		let line = dongle.read_line();
		dbg!(&line);
		let mut lex = Message::lexer(&line);

		let data = lex.next().unwrap();

		if let Ok(x) = data {
			match x {
				Message::Error => continue,
				Message::Telemetry(telem) => {
					let packet: TeleDonglePacket = telem.into();
					let e = packet.parse_payload();

					dbg!(e);
				},
			}
		}
	}
}