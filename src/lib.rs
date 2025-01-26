#![allow(non_snake_case)]

pub mod checksum;
pub mod constants;
pub mod control;

pub struct A8Mini {}

impl A8Mini {
	pub fn connect() -> Result<(), std::io::Error> {
		Ok(())
	}
}



#[cfg(test)]
mod tests {}
