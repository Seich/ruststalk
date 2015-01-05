# Ruststalk
A Rust Beanstalkd client.

Ruststalk aims to provide a protocol abiding implementation of a beanstalkd client.

## Example

``` rust
let stream = TcpStream::connect("127.0.0.1", 11300).unwrap();
let mut c = Client::new(stream);

let _ = c.use_tube("ruststalk");
let _ = c.put(1i, 0i, 0i, "Hello World");

let result = c.reserve();

if  result.is_ok() {
	println!("{}", result.unwrap());
}
```	
## License
Licensed under the [MIT License](http://www.opensource.org/licenses/mit-license.php).
