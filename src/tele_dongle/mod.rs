use rusb::{Context, Device, DeviceHandle, UsbContext};

mod usb_stuff;

// pub struct TeleDongle <U: UsbContext> {
// 	context: U,
// 	device: Device<U>,
// 	handle: DeviceHandle<U>,
// 	endpoint: usb_stuff::Endpoint,
// 	had_kernel_driver: bool,
// }

pub struct TeleDongle {
	context: Context,
	device: Device<Context>,
	handle: DeviceHandle<Context>,
	endpoint: usb_stuff::Endpoint,
	had_kernel_driver: bool,
}

// impl<U: UsbContext> TeleDongle<U> {
impl TeleDongle {
	// TODO: CHECK!!!
	const VID: u16 = 0xfffe;
	const PID: u16 = 0x000c;
	const CONFIG: u8 = 0;

	pub fn new() -> anyhow::Result<Self> {
		let mut context = Context::new()?;

		// get the UBS stuff for the teledongle
		let (mut device, mut handle) = usb_stuff::open_device(&mut context, Self::VID, Self::PID)
			.expect("no teledongle found!!!");

		// find the endpoint thingy, should be 0x00 i think
		let endpoints = usb_stuff::find_readable_endpoints(&mut device)?;
		dbg!(&endpoints);
		let endpoint = endpoints.first().expect("No Configurable endpoint found on device").clone();

		let had_kernel_driver = match handle.kernel_driver_active(endpoint.iface) {
			Ok(true) => {
				handle.detach_kernel_driver(endpoint.iface)?;
				true
			}
			_ => false,
		};

		// set config
		handle.set_active_configuration(Self::CONFIG)?;
		handle.claim_interface(endpoint.iface)?;
		// IDK what this does....
		// handle.set_alternate_setting(endpoint.iface, endpoint.setting)?;

		Ok(Self {
			context,
			device,
			handle,
			endpoint,
			had_kernel_driver,
		})
	}
}

// impl<U: UsbContext> Drop for TeleDongle<U> {
impl Drop for TeleDongle {
	fn drop(&mut self) {
		self.handle.release_interface(self.endpoint.iface).expect("couldn't release teledongle interface");

		if self.had_kernel_driver {
			self.handle.attach_kernel_driver(self.endpoint.iface).expect("couldn't attach kernel driver");
		}
	}
}