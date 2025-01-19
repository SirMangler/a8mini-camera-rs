use crate::constants::CRC16_TAB;

/// CRC16 Coding & Decoding 
/// - G(X) = X^16+X^12+X^5+1
pub fn CRC16_CALC(arr: &[u8], crc_init: u16) -> u16 {
  let mut crc16: u16 = crc_init;

  let mut oldcrc16: u16;
  let mut temp: u8;
  
  for val in arr {
    temp = (crc16 >> 8) as u8;
    oldcrc16 = CRC16_TAB[(val ^ temp) as usize];
    crc16 = (crc16 << 8) ^ oldcrc16;
  }
  
  crc16
}

#[cfg(test)]
mod tests {
  #[test]
  pub fn test_crc16_calc() {
    // TODO
  }
}