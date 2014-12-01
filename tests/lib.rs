extern crate ruststalk;
use ruststalk::Client;
use std::io::TcpStream;

#[test]
fn foo() {
	let stream = TcpStream::connect("127.0.0.1", 11300).unwrap();
	let mut c = Client::new(stream);
	let p = c.put("Hello World");

	if p.is_ok() {
		println!("Inserted.");
	}
}