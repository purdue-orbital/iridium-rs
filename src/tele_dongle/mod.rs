use rusb::{Context, Device, DeviceHandle};

use std::time::Duration;

mod usb_stuff;

// pub struct TeleDongle <U: UsbContext> {
// 	context: U,
// 	device: Device<U>,
// 	handle: DeviceHandle<U>,
// 	endpoint: usb_stuff::Endpoint,
// 	had_kernel_driver: bool,
// }

#[allow(dead_code)]
pub struct TeleDongle {
	context: Context,
	device: Device<Context>,
	handle: DeviceHandle<Context>,
	endpoints_and_kernel_drivers: Vec<(usb_stuff::Endpoint, bool)>,
}

// impl<U: UsbContext> TeleDongle<U> {
impl TeleDongle {
	// TODO: CHECK!!!
	// const VID: u16 = 0xfffe;
	const VID: u16 = 65534;
	// const PID: u16 = 0x000c;
	const PID: u16 = 12;

	const CONFIG: u8 = 0;
	const TELEM_ENDPOINT: u8 = 0x83;

	const TIMEOUT: Duration = Duration::from_millis(250);
	const LONG_TIMEOUT: Duration = Duration::from_millis(1500);

	const SET_LINE_CODING_PAYLOAD_17: [u8; 7] = [0x00, 0xc2, 0x01, 0x00, 0x00, 0x00, 0x08];
	const SET_LINE_CODING_PAYLOAD_39: [u8; 7] = [0x80, 0x25, 0x00, 0x00, 0x00, 0x00, 0x08]; // also used for 45
	const BULK_49: [u8; 10] = [0x7e, 0x0a, 0x45, 0x20, 0x30, 0x0a, 0x6d, 0x20, 0x30, 0x0a];
	const BULK_63: [u8;  5] = [0x6d, 0x20, 0x32, 0x30, 0x0a]; // also used for 73, 81
	const BULK_65: [u8; 12] = [0x6d, 0x20, 0x30, 0x0a, 0x63, 0x20, 0x73, 0x0a, 0x66, 0x0a, 0x76, 0x0a];
	const BULK_75: [u8; 20] = [0x6d, 0x20, 0x30, 0x0a, 0x63, 0x20, 0x46, 0x20, 0x34, 0x33, 0x34, 0x35, 0x35, 0x30, 0x0a, 0x6d, 0x20, 0x32, 0x30, 0x0a];
	const BULK_77: [u8;  9] = [0x6d, 0x20, 0x30, 0x0a, 0x6d, 0x20, 0x32, 0x30, 0x0a];
	const BULK_79: [u8; 15] = [0x6d, 0x20, 0x30, 0x0a, 0x63, 0x20, 0x54, 0x20, 0x30, 0x0a, 0x6d, 0x20, 0x32, 0x30, 0x0a];

	pub fn new() -> anyhow::Result<Self> {
		let mut context = Context::new()?;

		// get the UBS stuff for the teledongle
		let (mut device, mut handle) = usb_stuff::open_device(&mut context, Self::VID, Self::PID)
			.expect("no teledongle found!!!");

		// find the endpoint thingy, should be 0x00 i think
		let endpoints = usb_stuff::find_readable_endpoints(&mut device)?;
		dbg!(&endpoints);
		// let endpoint = endpoints.first().expect("No Configurable endpoint found on device").clone();

		let endpoints_and_kernel_drivers: Vec<(usb_stuff::Endpoint, bool)> = endpoints.clone().iter().map(|each| {
			// check if there is a kernel driver, if there is, detatch it
			let flag = match handle.kernel_driver_active(each.iface) {
				Ok(true) => {
					handle.detach_kernel_driver(each.iface).expect("couldn't detatch kernel driver");
					true
				}
				_ => false,
			};

			(each.clone(), flag)
		}).collect();

		// set config
		// should iface = 1 and setting = 0??? (look at packet 13)
		handle.set_active_configuration(Self::CONFIG)?;

		// for each in endpoints.clone() {
		// 	handle.claim_interface(each.iface)?;
		// }

		// TEMP FIX
		handle.claim_interface(0)?;
		handle.claim_interface(1)?;

		// packet 11??
		// TODO check????
		handle.write_control(0x00, 0x09, 0x0100, 0, &[], Self::TIMEOUT)?;

		// IDK what this does....
		// i think its packet 13
		for each in endpoints.clone() {
			handle.set_alternate_setting(each.iface, each.setting)?;
		}

		// set control line state request (packet 17)
		// i think this is correct
		handle.write_control(0x21, 0x22, 0, 0, &[], Self::TIMEOUT)?;

		// set line coding request (packet 19)
		handle.write_control(0x21, 0x20, 0, 0, &Self::SET_LINE_CODING_PAYLOAD_17, Self::TIMEOUT)?;

		// FROM HERE ON i am keeping track of which packets haven't been implemented
		// skipping packets: 23, 24, 25, 28, 29, 31, 33, 35, 37

		// set line coding request (packet 39)
		handle.write_control(0x21, 0x20, 0, 0, &Self::SET_LINE_CODING_PAYLOAD_39, Self::TIMEOUT)?;

		// ignore packet: 41

		// set control line state request (packet 43)
		handle.write_control(0x21, 0x02, 3, 0, &[], Self::TIMEOUT)?;

		// set line coding request (packet 45)
		handle.write_control(0x21, 0x20, 0, 0, &Self::SET_LINE_CODING_PAYLOAD_39, Self::TIMEOUT)?;

		// packet 49
		handle.write_bulk(2, &Self::BULK_49, Self::TIMEOUT)?;

		// skipping packets: 51-61.
		// packet 62 is just info. same for 67-71

		// packet 63-66
		handle.write_bulk(2, &Self::BULK_63, Self::TIMEOUT)?;
		handle.write_bulk(2, &Self::BULK_65, Self::TIMEOUT)?;

		// packet 72's response is telemetry, but it's reply (packet 83) comes 30 seconds later soooo idk
		
		// packets 73-82
		handle.write_bulk(2, &Self::BULK_63, Self::TIMEOUT)?; // 73
		handle.write_bulk(2, &Self::BULK_75, Self::TIMEOUT)?; // 75
		handle.write_bulk(2, &Self::BULK_77, Self::TIMEOUT)?; // 77
		handle.write_bulk(2, &Self::BULK_79, Self::TIMEOUT)?; // 79
		handle.write_bulk(2, &Self::BULK_63, Self::TIMEOUT)?; // 81

		Ok(Self {
			context,
			device,
			handle,
			endpoints_and_kernel_drivers,
		})
	}

	pub fn read_from_telem_endpoint(&self) -> anyhow::Result<Vec<u8>> {
		let mut buf: Vec<u8> = vec![0; 1024*16];

		let response = self.handle.read_bulk(Self::TELEM_ENDPOINT, &mut buf, Self::LONG_TIMEOUT);

		match response {
			Ok(n) => {
				buf.truncate(n);
				Ok(buf)
			},
			Err(rusb::Error::Overflow) => {
				Ok(buf)
			},
			Err(_) => {
				dbg!(&response);
				buf.truncate(response?);
				Ok(buf)
			}
		}
	}

	// decodes 36 bytes worth of a hex string into bytes
	pub fn hex_string_to_bytes(input: &str) -> Vec<u8> {
		let mut buf: Vec<u8> = vec![0; 36];

		hex::decode_to_slice(input, &mut buf).expect("couldn't convert TELEM hex string to bytes");

		buf
	}
}

// TODO!!!! reimplement
// // impl<U: UsbContext> Drop for TeleDongle<U> {
// impl Drop for TeleDongle {
// 	fn drop(&mut self) {
// 		self.handle.release_interface(self.endpoint.iface).expect("couldn't release teledongle interface");

// 		// reattach kernel driver if needed
// 		if self.had_kernel_driver {
// 			self.handle.attach_kernel_driver(self.endpoint.iface).expect("couldn't attach kernel driver");
// 		}
// 	}
// }

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
