extern crate ruststalk;
use ruststalk::Client;
use std::io::TcpStream;

#[test]
fn client_should_insert_new_jobs_into_the_queue() {
	let stream = TcpStream::connect("127.0.0.1", 11300).unwrap();
	let mut c = Client::new(stream);

	assert!(c.put(1i, 0i, 0i, "Hello World").is_ok());
}


#[test]
fn client_should_change_tube() {
	let stream = TcpStream::connect("127.0.0.1", 11300).unwrap();
	let mut c = Client::new(stream);

	assert!(c.use_tube("newTube").is_ok())
}

#[test]
fn client_should_reserve_jobs() {
	let stream = TcpStream::connect("127.0.0.1", 11300).unwrap();
	let mut c = Client::new(stream);

	assert!(c.reserve().is_ok());
}

#[test]
fn client_should_reserve_jobs_with_a_timeout() {
	let stream = TcpStream::connect("127.0.0.1", 11300).unwrap();
	let mut c = Client::new(stream);

	assert!(c.reserve_with_timeout(4).is_ok());
}

#[test]
fn client_should_delete_jobs() {
	let stream = TcpStream::connect("127.0.0.1", 11300).unwrap();
	let mut c = Client::new(stream);

	assert!(c.delete(1).is_ok());
}

#[test]
fn client_should_release_jobs() {
	let stream = TcpStream::connect("127.0.0.1", 11300).unwrap();
	let mut c = Client::new(stream);

	assert!(c.release(1, 1, 4).is_ok());
}

#[test]
fn client_should_bury_jobs() {
	let stream = TcpStream::connect("127.0.0.1", 11300).unwrap();
	let mut c = Client::new(stream);
	
	assert!(c.bury(1, 1).is_ok());
}

#[test]
fn client_should_touch_jobs() {
	let stream = TcpStream::connect("127.0.0.1", 11300).unwrap();
	let mut c = Client::new(stream);
	
	assert!(c.touch(1).is_ok());
}

#[test]
fn client_should_watch_tubes() {
	let stream = TcpStream::connect("127.0.0.1", 11300).unwrap();
	let mut c = Client::new(stream);
	
	assert!(c.watch("test").is_ok());
}

#[test]
fn client_should_show_stats() {
	let stream = TcpStream::connect("127.0.0.1", 11300).unwrap();
	let mut c = Client::new(stream);
	
	assert!(c.stats().is_ok());
}