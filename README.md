Rust ZeroMQ bindings.

[![Travis Build Status](https://travis-ci.org/erickt/rust-zmq.png?branch=master)](https://travis-ci.org/erickt/rust-zmq)
[![Appveyor Build status](https://ci.appveyor.com/api/projects/status/xhytsx4jwyb9qk7m?svg=true)](https://ci.appveyor.com/project/erickt/rust-zmq)
[![Coverage Status](https://coveralls.io/repos/erickt/erickt-zmq/badge.svg?branch=master)](https://coveralls.io/r/erickt/erickt-zmq?branch=master)
[![Apache 2.0 licensed](https://img.shields.io/badge/license-Apache2.0-blue.svg)](./LICENSE-APACHE)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE-MIT)
[![crates.io](http://meritbadge.herokuapp.com/zmq)](https://crates.io/crates/zmq)

Installation
------------

Currently, rust-zmq requires ZeroMQ 4.1 or newer. For example, on
recent Debian-based distributions, you can use the following command
to get the prerequiste headers and library installed:

    apt install libzmq3-dev

If your OS of choice does not provide packages of a new-enough libzmq,
you will first have to install it from source; see
<https://github.com/zeromq/libzmq/releases>.

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
    let ctx = zmq::Context::new();

    let mut socket = ctx.socket(zmq::REQ).unwrap();
    socket.connect("tcp://127.0.0.1:1234").unwrap();
    socket.send_str("hello world!", 0).unwrap();
}
```

You can find more usage examples in
https://github.com/erickt/rust-zmq/tree/master/examples.
