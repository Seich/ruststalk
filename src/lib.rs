use std::io::TcpStream;
use std::str;

pub struct Client {
	stream: TcpStream
}

impl Client {
	pub fn new(stream: TcpStream) -> Client {
		Client {
			stream: stream
		}
	}

	pub fn put(&mut self, data: &str) -> Result<String, &str> {
		write!(self.stream,
			"put {pri} {delay} {ttr} {bytes}\r\n{data}\r\n", 
			pri = 1i,
			delay = 0i,
			ttr = 0i,
			bytes = data.len(),
			data = data
		);

		let mut buf = [0, ..4096];
		let result = self.stream.read(&mut buf);
		
		match result {
			Err(e) => fail!(e),
			Ok(_) => {
				let s = match str::from_utf8(buf) {
					Some(e) => e.replace("\n", "").replace("\r", ""),
					None => fail!("Invalid UTF-8 sequence"),
				};

				return Ok(s.to_string());
			}
		}
	}
}