use rusb::{Device, DeviceHandle, UsbContext};

#[derive(Debug, Clone)]
pub struct Endpoint {
	pub config: u8,
	pub iface: u8,
	pub setting: u8,
	pub address: u8,
}

pub fn open_device<T: UsbContext>(
	context: &mut T,
	vid: u16,
	pid: u16,
) -> Option<(Device<T>, DeviceHandle<T>)> {
	let devices = match context.devices() {
		Ok(d) => d,
		Err(_) => return None,
	};

	for device in devices.iter() {
		let device_desc = match device.device_descriptor() {
			Ok(d) => d,
			Err(_) => continue,
		};

		if (device_desc.vendor_id() == vid) && (device_desc.product_id() == pid) {
			match dbg!(device.open()) {
				Ok(handle) => return Some((device, handle)),
				Err(_) => continue,
			}
		}
	}

	None
}

/// returns all readable endpoints for given usb device and descriptor
pub fn find_readable_endpoints<T: UsbContext>(device: &mut Device<T>) -> anyhow::Result<Vec<Endpoint>> {
	let device_desc = device.device_descriptor()?;
	let mut endpoints = vec![];
	for n in 0..device_desc.num_configurations() {
		let config_desc = match device.config_descriptor(n) {
			Ok(c) => c,
			Err(_) => continue,
		};
		// println!("{:#?}", config_desc);
		for interface in config_desc.interfaces() {
			for interface_desc in interface.descriptors() {
				// println!("{:#?}", interface_desc);
				for endpoint_desc in interface_desc.endpoint_descriptors() {
					// println!("{:#?}", endpoint_desc);
					endpoints.push(Endpoint {
						config: config_desc.number(),
						iface: interface_desc.interface_number(),
						setting: interface_desc.setting_number(),
						address: endpoint_desc.address(),
					});
				}
			}
		}
	}

	Ok(endpoints)
}
