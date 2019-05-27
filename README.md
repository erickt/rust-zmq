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

rust-zmq is available from [crates.io](https://crates.io). Users
should add this to their `Cargo.toml` file:

```toml
[dependencies]
zmq = "0.9"
```

As rust-zmq is a wrapper around `libzmq`, you need a build of `libzmq`
version 4.1 or newer, before attempting to build the `zmq`
crate. There are several options available:

## Dynamic linking using `pkg-config`

This is probably the preferred method when you are running a recent
Unix-like OS that has support for `pkg-config`. For example, on recent
Debian-based distributions, you can use the following command to get
the prerequiste headers and library installed:

```sh
apt install libzmq3-dev
```

If your OS of choice does not provide packages of a new-enough libzmq,
you can install it from source; see
<https://github.com/zeromq/libzmq/releases>, although in this case,
you may prefer a `vendored` build, which automates that, see below.

The build normally uses `pkg-config` to find out about libzmq's
location. If that is not available, the environment variable
`LIBZMQ_PREFIX` (or alternatively, `LIBZMQ_LIB_DIR` and
`LIBZMQ_INCLUDE_DIR`) can be defined to avoid the invocation of
`pkg-config`.

## Windows build

When building on Windows, using the MSCV toolchain, consider the
following when trying to link dynamically against `libzmq`:

- When building `libzmq` from sources, the library must be renamed
  to `zmq.lib` from the auto named `libzmq-v***-mt-gd-*_*_*.lib`,
  `libzmq.lib`, `libzmq-mt-*_*_*.lib`, etc.
- The folder containing the `*.dll` (dynamic link library)
  referred to by `zmq.lib` must be accessible via the path for
  the session that invokes the Rust compiler.
- The name of the `*.dll` in question depends on the build system
  used for `libzmq` and can usually be seen when opening `zmq.lib`
  in a text editor.

## Vendored build

Starting with the upcoming release `0.9.1` (or when building from
current `master`), you can enable the `vendored` feature flag to have
`libzmq` be built for you and statically linked into your binary
crate. In your `Cargo.toml`, you can give users the option to do so
using a dedicated feature flag:

```toml
[features]
vendored-zmq = ['zmq/vendored']
```

# Usage

`rust-zmq` is a pretty straight forward port of the C API into Rust:

```rust
fn main() {
    let ctx = zmq::Context::new();

    let mut socket = ctx.socket(zmq::REQ).unwrap();
    socket.connect("tcp://127.0.0.1:1234").unwrap();
    socket.send("hello world!", 0).unwrap();
}
```

You can find more usage examples in
https://github.com/erickt/rust-zmq/tree/master/examples.

# Contributing

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the
Apache-2.0 license, shall be dual licensed under the terms of both the
Apache License, Version 2.0 and the MIT license without any additional
terms or conditions.

See the [contribution guidelines] for what to watch out for when
submitting a pull request.

[contribution guidelines]: ./CONTRIBUTING.md
