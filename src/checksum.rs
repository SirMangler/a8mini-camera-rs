use crate::constants::CRC16_TAB;

/// CRC16 Coding & Decoding 
/// - G(X) = X^16+X^12+X^5+1
pub fn crc16_calc(arr: &[u8], crc_init: u16) -> u16 {
	let mut crc16: u16 = crc_init;
	
	for &val in arr {
		let temp = (crc16 >> 8) as u8;
		let oldcrc16 = CRC16_TAB[(val ^ temp) as usize];
		crc16 = (crc16 << 8) ^ oldcrc16;
	}
	
	crc16
}


#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	pub fn test_crc16_calc() {
		// GROUND TRUTH: https://crccalc.com/?crc=&method=CRC-16/XMODEM&datatype=hex&outtype=hex
		
		assert_eq!(
			"0xed27", 
			format!(
				"{:#04x}", 
				// heartbeat packet
				crc16_calc(&[0x55, 0x66, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x59, 0x8B], 0)
			)
		);

		assert_eq!(
			"0x4e81", 
			format!(
				"{:#04x}", 
				// camera mode packet
				crc16_calc(&[0x55, 0x66, 0x01, 0x00, 0x00, 0x00, 0x00, 0x19, 0x5D, 0x57], 0)
			)
		);

		assert_eq!(
			"0xaa28", 
			format!(
				"{:#04x}", 
				// center camera packet
				crc16_calc(&[0x55, 0x66, 0x01, 0x01, 0x00, 0x00, 0x00, 0x08, 0x01, 0xD1, 0x12], 0)
			)
		);
	}
}