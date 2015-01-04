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

	fn run_command(&mut self, command: String) -> Result<String, &str> {
		let _ = self.stream.write(command.as_bytes());

		let mut buffer = [0, ..4096];
		let result = self.stream.read(&mut buffer);

		match result {
			Err(e) => return Err(e.desc),
			Ok(_) => {
				let s = str::from_utf8(buffer).unwrap();

				return Ok(s.to_string());
			}
		}
	}

	// Producer commands
	pub fn put(&mut self, priority: int, delay: int, ttr: int, data: &str) -> Result<String, &str> {
		let cmd = format!("put {priority} {delay} {ttr} {bytes}\r\n{data}\r\n", 
			priority = priority,
			delay = delay,
			ttr = ttr,
			bytes = data.len(),
			data = data
		);

		return self.run_command(cmd);
	}

	pub fn use_tube(&mut self, tube: &str) -> Result<String, &str> {
		let cmd = format!("use {tube}\r\n", tube = tube);
		return self.run_command(cmd);
	}

	// Worker commands
	pub	fn reserve(&mut self) -> Result<String, &str> {
		let cmd = "reserve\r\n";
		return self.run_command(cmd.to_string());
	}

	pub	fn reserve_with_timeout(&mut self, seconds: int) -> Result<String, &str> {
		let cmd = format!("reserve-with-timeout {seconds}\r\n", seconds=seconds);
		return self.run_command(cmd.to_string());
	}

	pub fn delete(&mut self, id: int) -> Result<String, &str> {
		let cmd = format!("delete {id}\r\n", id=id);
		return self.run_command(cmd.to_string());
	}

	pub fn release(&mut self, id: int, priority: int, delay: int) -> Result<String, &str> {
		let cmd = format!("release {id} {priority} {delay}\r\n", 
			id=id, 
			priority=priority, 
			delay=delay
		);

		return self.run_command(cmd.to_string());
	}

	pub fn bury(&mut self, id: int, priority: int) -> Result<String, &str> {
		let cmd = format!("bury {id} {priority}\r\n", 
			id=id, 
			priority=priority
		);

		return self.run_command(cmd.to_string());
	}

	pub fn touch(&mut self, id: int) -> Result<String, &str> {
		let cmd = format!("touch {id}\r\n", id=id);
		return self.run_command(cmd.to_string());
	}

	pub fn watch(&mut self, tube: &str) -> Result<String, &str> {
		let cmd = format!("watch {tube}\r\n", tube=tube);
		return self.run_command(cmd.to_string());
	}

	pub fn ignore(&mut self, tube: &str) -> Result<String, &str> {
		let cmd = format!("ignore {tube}\r\n", tube=tube);
		return self.run_command(cmd.to_string());
	}

	pub fn peek(&mut self, id: int) -> Result<String, &str> {
		let cmd = format!("peek {id}\r\n", id=id);
		return self.run_command(cmd.to_string());
	}

	pub fn peek_ready(&mut self) -> Result<String, &str> {
		let cmd = format!("peek-ready\r\n");
		return self.run_command(cmd.to_string());
	}

	pub fn peek_delayed(&mut self) -> Result<String, &str> {
		let cmd = format!("peek-delayed\r\n");
		return self.run_command(cmd.to_string());
	}

	pub fn peek_buried(&mut self) -> Result<String, &str> {
		let cmd = format!("peek-buried\r\n");
		return self.run_command(cmd.to_string());
	}

	pub fn kick(&mut self, bound: int) -> Result<String, &str> {
		let cmd = format!("kick {bound}\r\n", bound=bound);
		return self.run_command(cmd.to_string());
	}

	pub fn stats_job(&mut self, id: int) -> Result<String, &str> {
		let cmd = format!("stats-job {id}\r\n", id=id);
		return self.run_command(cmd.to_string());
	}

	pub fn stats_tube(&mut self, tube: &str) -> Result<String, &str> {
		let cmd = format!("stats-tube {tube}\r\n", tube=tube);
		return self.run_command(cmd.to_string());
	}

	pub fn stats(&mut self) -> Result<String, &str> {
		let cmd = format!("stats\r\n");
		return self.run_command(cmd.to_string());
	}

	pub fn list_tubes(&mut self) -> Result<String, &str> {
		let cmd = format!("list-tubes\r\n");
		return self.run_command(cmd.to_string());
	}

	pub fn list_tube_used(&mut self) -> Result<String, &str> {
		let cmd = format!("list-tube-used\r\n");
		return self.run_command(cmd.to_string());
	}

	pub fn list_tubes_watched(&mut self) -> Result<String, &str> {
		let cmd = format!("list-tubes-watched\r\n");
		return self.run_command(cmd.to_string());
	}

}