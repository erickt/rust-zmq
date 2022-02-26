Rust ZeroMQ bindings.

[![Travis Build Status](https://travis-ci.org/erickt/rust-zmq.png?branch=master)](https://travis-ci.org/erickt/rust-zmq)
[![Appveyor Build status](https://ci.appveyor.com/api/projects/status/xhytsx4jwyb9qk7m?svg=true)](https://ci.appveyor.com/project/erickt/rust-zmq)
[![Coverage Status](https://coveralls.io/repos/erickt/erickt-zmq/badge.svg?branch=master)](https://coveralls.io/r/erickt/erickt-zmq?branch=master)
[![Apache 2.0 licensed](https://img.shields.io/badge/license-Apache2.0-blue.svg)](./LICENSE-APACHE)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE-MIT)
[![crates.io](http://meritbadge.herokuapp.com/zmq)](https://crates.io/crates/zmq)
[![docs](https://docs.rs/zmq/badge.svg)](https://docs.rs/zmq)

[Documentation](https://docs.rs/crate/zmq/)

[Release Notes](https://github.com/erickt/rust-zmq/tree/master/NEWS.md)

# About

The `zmq2` crate provides bindings for the `libzmq` library from the
[ZeroMQ](https://zeromq.org/) project. This project is a fork of the 
[https://github.com/erickt/rust-zmq](rust-zmq) project, with the intent
of keeping it actively maintained.

This project removes the `cmake` dependency of this project, as well as
update the dependencies. It has also removed the `pkgconfig` build in
favor of always building a vendored version of this library.

# Compatibility

The aim of this fork is to track latest zmq releases as close as possible, 
while in the beginning aming to be a drop in replacement of the original
`zmq` library. Though over time we'll most likely abandon that, in favor
of our own library features.

# Usage

`zmq2` is a pretty straight forward port of the C API into Rust:

```rust
fn main() {
    let ctx = zmq2::Context::new();

    let socket = ctx.socket(zmq2::REQ).unwrap();
    socket.connect("tcp://127.0.0.1:1234").unwrap();
    socket.send("hello world!", 0).unwrap();
}
```

You can find more usage examples in
https://github.com/Traverse-Research/zmq2/tree/master/examples.

# Contributing

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the
Apache-2.0 license, shall be dual licensed under the terms of both the
Apache License, Version 2.0 and the MIT license without any additional
terms or conditions.

See the [contribution guidelines] for what to watch out for when
submitting a pull request.

[contribution guidelines]: ./CONTRIBUTING.md
