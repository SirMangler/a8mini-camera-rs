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
	/// Connects to the A8 mini from port 8080 located at the default IP 192.168.144.25 and port number 37260.
	pub async fn connect() -> Result<A8Mini, Box<dyn Error>> {
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

	pub async fn send_command_blind(&self, command: control::A8MiniCommand) -> Result<(), Box<dyn Error>> {
		if self.command_socket.send(constants::COMMANDS[command as usize]).await? == 0 {
			return Err("No bytes sent.".into());
		}

		Ok(())
	}

	pub async fn send_command(&self, command: control::A8MiniCommand) -> Result<[u8; RECV_BUFF_SIZE], Box<dyn Error>> {
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
mod tests {}
