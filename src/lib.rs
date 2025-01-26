#![allow(non_snake_case)]

use std::error::Error;
use constants::RECV_BUFF_SIZE;
use tokio::net::UdpSocket;

pub mod checksum;
pub mod constants;
pub mod control;

pub struct A8Mini {
	command_socket: UdpSocket,
	http_socket: UdpSocket,
}

impl A8Mini {
	pub async fn connect() -> Result<Self, Box<dyn Error>> {
		Ok(Self::connect_to(constants::CAMERA_IP, constants::CAMERA_COMMAND_PORT, constants::CAMERA_HTTP_PORT, "8080", "80").await?)
	}

	pub async fn connect_to(camera_ip: &str, camera_command_port: &str, camera_http_port: &str, local_command_port: &str, local_http_port: &str) -> Result<A8Mini, Box<dyn Error>> {
		let camera: A8Mini = A8Mini {
			command_socket: UdpSocket::bind(format!("0.0.0.0:{}", local_command_port)).await?,
			http_socket: UdpSocket::bind(format!("0.0.0.0:{}", local_http_port)).await?,
		};

		camera.command_socket.connect(format!("{}:{}", camera_ip, camera_command_port)).await?;
		camera.http_socket.connect(format!("{}:{}", camera_ip, camera_http_port)).await?;
		Ok(camera)
	}

	pub async fn send_command_blind<T: control::Command>(&self, command: T) -> Result<(), Box<dyn Error>> {
		if self.command_socket.send(command.to_bytes().as_slice()).await? == 0 {
			return Err("No bytes sent.".into());
		}

		Ok(())
	}

	pub async fn send_command<T: control::Command>(&self, command: T) -> Result<[u8; RECV_BUFF_SIZE], Box<dyn Error>> {
		self.send_command_blind(command).await?;
		let mut recv_buffer = [0; RECV_BUFF_SIZE];
		if self.command_socket.recv(&mut recv_buffer).await? == 0  {
			return Err("No bytes received.".into());
		}

		Ok(recv_buffer)
	}

	// pub async fn make_http_query_blind(&self) -> Result<String, Box<dyn Error>> {
		
	// }

	// pub async fn make_http_query(&self) -> Result<String, Box<dyn Error>> {
		
	// }
}



#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test]
	async fn test_send_command_blind() -> Result<(), Box<dyn Error>> {
		let cam: A8Mini = A8Mini::connect().await?;

		cam.send_command_blind(control::A8MiniSimpleCommand::RotateLeft).await?;


		Ok(())
	}
}
