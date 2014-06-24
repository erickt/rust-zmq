Rust Zeromq bindings.

[![Build Status](https://travis-ci.org/erickt/rust-zmq.png?branch=master)](https://travis-ci.org/erickt/rust-zmq)

To build, just run `rustc lib.rs`. rust-zmq is a pretty straight forward
port of the C API into Rust:

	extern crate zmq;
	
	#[link(name = "zmq")] // link against the C library
	extern {}
	
	fn main() {
		let mut ctx = zmq::Context::new();
	
		let mut socket = match ctx.socket(zmq::REQ) {
		  Ok(socket) => { socket },
		  Err(e) => { fail!(e.to_str()) }
		};
	
		match socket.connect("tcp://127.0.0.1:1234") {
		  Ok(()) => (),
		  Err(e) => fail!(e.to_str())
		}
	
		match socket.send_str("hello world!", 0) {
		  Ok(()) => (),
		  Err(e) => fail!(e.to_str())
		}
	}


Installation
------------

rust-zmq uses [cargo](https://crates.io) to install. Users should add this to
their `Cargo.toml` file:

    [dependencies.zmq]
    git = "https://github.com/erickt/rust-zmq.git"

Install for developers:

    % git clone https://github.com/erickt/rust-zmq
    % cd rust-zmq
    % cargo build
