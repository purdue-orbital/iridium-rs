use iridium_rs::tele_dongle::*;

fn main() {
	#[allow(unused_mut)]
	let mut dongle = TeleDongle::new().unwrap();

	loop {
		let buf = dongle.read_from_telem_endpoint().unwrap();

		dbg!(buf);
	}
}