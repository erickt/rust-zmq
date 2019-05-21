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

Installation
------------

Currently, rust-zmq requires ZeroMQ 3.2 or newer. For example, on
recent Debian-based distributions, you can use the following command
to get the prerequiste headers and library installed:

    apt install libzmq3-dev

If your OS of choice does not provide packages of a new-enough libzmq,
you will first have to install it from source; see
<https://github.com/zeromq/libzmq/releases>.

rust-zmq uses [cargo](https://crates.io) to install. Users should add this to
their `Cargo.toml` file:

    [dependencies]
    zmq = "0.8"

Install for developers:

    % git clone https://github.com/erickt/rust-zmq
    % cd rust-zmq
    % cargo build

The build normally uses `pkg-config` to find out about libzmq's
location. If that is not available, the environment variable
`LIBZMQ_PREFIX` (or alternatively, `LIBZMQ_LIB_DIR` and
`LIBZMQ_INCLUDE_DIR`) can be defined to avoid the invocation of
`pkg-config`.

Note for Windows users (re. dynamic linking of ZeroMQ):

- When building `libzmq` from sources, the library must be renamed 
  to `zmq.lib` from the auto named `libzmq-v***-mt-gd-*_*_*.lib`, 
  `libzmq.lib`, `libzmq-mt-*_*_*.lib`, etc. 
- The `LIBZMQ_BIN_DIR` environment variable must be defined and must 
  point to the folder containing the `*.dll` (dynamic link library) 
  referred to by `zmq.lib`. It must also be accessible via the 
  path for the session that invokes the Rust compiler. 
- The name of the `*.dll` in question depends on the build system 
  used for `libzmq` and can usually be seen when opening `zmq.lib` 
  in a text editor.
- See example helper functions [win_set_environment_vars_rust_zmq.bat](https://gist.github.com/hansieodendaal/a3811c33382476d15999088466fcfe29) 
  and [win_source_dll_path_rust_zmq.bat](https://gist.github.com/hansieodendaal/8cf5920e8bafa099334ad929c79062b7) (use at own risk!).

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
