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

# Installation

Currently, rust-zmq requires ZeroMQ 4.1 or newer. For example, on
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

The build normally uses `pkg-config` to find out about libzmq's
location. If that is not available, the environment variable
`LIBZMQ_PREFIX` (or alternatively, `LIBZMQ_LIB_DIR` and
`LIBZMQ_INCLUDE_DIR`) can be defined to avoid the invocation of
`pkg-config`.

Note for Windows users (regarding dynamic linking of ZeroMQ):

- When building `libzmq` from sources, the library must be renamed
  to `zmq.lib` from the auto named `libzmq-v***-mt-gd-*_*_*.lib`,
  `libzmq.lib`, `libzmq-mt-*_*_*.lib`, etc.
- The folder containing the `*.dll` (dynamic link library)
  referred to by `zmq.lib` must be accessible via the path for
  the session that invokes the Rust compiler.
- The name of the `*.dll` in question depends on the build system
  used for `libzmq` and can usually be seen when opening `zmq.lib`
  in a text editor.

# Usage

`rust-zmq` is a pretty straight forward port of the C API into Rust:

```rust
fn main() {
    let ctx = zmq::Context::new();

    let mut socket = ctx.socket(zmq::REQ).unwrap();
    socket.connect("tcp://127.0.0.1:1234").unwrap();
    socket.send_str("hello world!", 0).unwrap();
}
```

You can find more usage examples in
https://github.com/erickt/rust-zmq/tree/master/examples.

# Contributing

Install for contributing to rust-zmq:

    % git clone https://github.com/erickt/rust-zmq
    % cd rust-zmq
    % cargo build

Note that the `master` branch is currently in API-breaking mode while
we try to make the API more ergomic and flexible for the `0.9` release
series.

__This means that pull requests (e.g. bugfixes), which do not need to
break API should be submitted for the `release/v0.8` branch__. This
also applies to new features, if they can be implemented in an
API-compatible way, the pull request should also aim for
`release/v0.8`. Please submit an issue for missing features before you
start coding, so the suitable branch and other potential questions can
be clarified up-front.

The reason for using branches, and thus increasing overhead a bit for
all involved, is that it's not yet clear how long it will take to
reach a point in `master` that we feel comfortable with releasing as
0.9.0, as we'd like to have the core part of the API more-or-less
fixed by then. Using the `release/v0.8` branch we can deliver bugfixes
and smaller features in the meantime without forcing users to follow
master's changing API and to continuously adjust their code to API
changes.
