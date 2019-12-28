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

The `zmq` crate provides bindings for the `libzmq` library from the
[ZeroMQ](https://zeromq.org/) project. The API exposed by `zmq` should
be safe (in the usual Rust sense), but it follows the C API closely,
so it is not very idiomatic. Also, support for `libzmq` API in "draft"
state is considered out-of-scope for this crate; this includes
currently, as of libzmq 4.3.3:

- Newer, thread-safe socket types, such as `ZMQ_CLIENT` and
  `ZMQ_SERVER`.
- The "poller" API.

For a more modern, idiomatic approach to `libzmq` bindings, including
draft API features, have a look at
[`libzmq-rs`](https://github.com/jean-airoldie/libzmq-rs).

# Compatibility

The current 0.9 release series requires `libzmq` 4.1 or newer. New
release series of `zmq` may require newer `libzmq` versions.

Regarding the minimum Rust version required, `zmq` is CI-tested on
current stable, beta and nightly channels of Rust. Additionally, it is
made sure that the code still compiles on Rust 1.32.0. However, no
tests are run for that build, so use `zmq` on older Rust versions on
your own risk. It is however likely that it will just work anyways.

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

## Cross-compilation

When you have a cross-compiled version of `libzmq` installed, you
should be able to cross-compile rust-zmq, assuming a platform
supporting `pkg-config`. For example, assuming you have `libzmq`
compiled for the `i686-pc-windows-gnu` target installed in
`~/.local-w32`, the following should work:

```sh
PKG_CONFIG_PATH=$HOME/.local-w32/lib/pkgconfig \
PKG_CONFIG_ALLOW_CROSS=1 \
cargo build --target=i686-pc-windows-gnu --verbose
```

Cross compilation without `pkg-config` should work as well, but you
need set `LIBZMQ_PREFIX` as described above.

# Usage

`rust-zmq` is a pretty straight forward port of the C API into Rust:

```rust
fn main() {
    let ctx = zmq::Context::new();

    let socket = ctx.socket(zmq::REQ).unwrap();
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
