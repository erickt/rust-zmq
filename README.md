Rust ZeroMQ bindings.

[![Travis Build Status](https://travis-ci.org/erickt/rust-zmq.png?branch=master)](https://travis-ci.org/erickt/rust-zmq)
[![Appveyor Build status](https://ci.appveyor.com/api/projects/status/xhytsx4jwyb9qk7m?svg=true)](https://ci.appveyor.com/project/erickt/rust-zmq)
[![Coverage Status](https://coveralls.io/repos/erickt/erickt-zmq/badge.svg?branch=master)](https://coveralls.io/r/erickt/erickt-zmq?branch=master)
[![Apache 2.0 licensed](https://img.shields.io/badge/license-Apache2.0-blue.svg)](./LICENSE-APACHE)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE-MIT)
[![crates.io](http://meritbadge.herokuapp.com/zmq)](https://crates.io/crates/zmq)

Installation
------------

Currently, rust-zmq requires ZeroMQ 4.1. For example, on recent
Debian-based distributions, you can use the following command to get
the prerequiste headers and library installed:

    apt install libzmq3-dev

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

```rust
extern crate zmq;

fn main() {
	let mut ctx = zmq::Context::new();

	let mut socket = match ctx.socket(zmq::REQ) {
	  Ok(socket) => { socket },
	  Err(e) => { panic!(e) }
	};

	match socket.connect("tcp://127.0.0.1:1234") {
	  Ok(()) => (),
	  Err(e) => panic!(e)
	}

	match socket.send_str("hello world!", 0) {
	  Ok(()) => (),
	  Err(e) => panic!(e)
	}
}
```

You can find more usage examples in
https://github.com/erickt/rust-zmq/tree/master/examples.
