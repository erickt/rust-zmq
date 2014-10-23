Rust ZeroMQ bindings.

[![Build Status](https://travis-ci.org/erickt/rust-zmq.png?branch=master)](https://travis-ci.org/erickt/rust-zmq)

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

Usage
-----

`rust-zmq` is a pretty straight forward port of the C API into Rust:

	extern crate zmq;
	
	fn main() {
		let mut ctx = zmq::Context::new();
	
		let mut socket = match ctx.socket(zmq::REQ) {
		  Ok(socket) => { socket },
		  Err(e) => { fail!(e.to_string()) }
		};
	
		match socket.connect("tcp://127.0.0.1:1234") {
		  Ok(()) => (),
		  Err(e) => fail!(e.to_string())
		}
	
		match socket.send_str("hello world!", 0) {
		  Ok(()) => (),
		  Err(e) => fail!(e.to_string())
		}
	}

You can find more usage examples in
https://github.com/erickt/rust-zmq/tree/master/examples.
