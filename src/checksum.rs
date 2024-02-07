#[allow(dead_code)]
pub fn u16_summation(bin: &[u8]) -> u16 {
	let mut total: u16 = 0;

	for each in bin {
		total = total.wrapping_add(*each as u16);
	}

	total
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn checksum_test_01() {
		let ans = u16_summation(b"hello");

		assert_eq!(0x0214, ans);
	}
}
