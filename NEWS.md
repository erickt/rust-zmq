# 0.9.0 (unreleased)

This release will focus on API corrections and improving
documentation.

## Incompatible API changes

- The deprecated `Constants` enum has been removed from the API.

- Message allocation, e.g. `Message::new()` directly returns `Message`
  instead of `Result<Message>` and will panic on allocation failure,
  as is customary in Rust. Reported in #118 and fixed by #130.

## New and improved functionality

- `Message` now implements `From` for various types that have an
  obvious byte-level representation. This is possible due to the
  message allocation API change (see above).

- `Message::send()` now works on `Into<Message>` types, obsoleting
  `send_msg()` and `send_str()`.

## Deprecations

- `Message::send_msg()` and `send_str()` are deprecated in favor of
  `Message::send()`.

# 0.8.2

## New and improved functionality

- Support for the `ZMQ_PROBE_ROUTER` and `ZMQ_ROUTER_MANDATORY` socket
  options.
- `zmq_disconnect` is now exposed as `Socket::disconnect`.

## Bug fixes

- Fix build on OpenBSD (issue #170).
- Account for OpenBSD not defining `EPROTO`.
- Fix build for 32-bit Windows.
- Handle `EINTR` in `Error::from_raw` (issue #174).
- Alignment of `zmq_msg_t` FFI type fixed.
- Fix `build.rs` to portably construct paths, instead of hard-coding
  slash as path separator.

# 0.8.1

This release fixes the remaining Windows-specific issues exposed by
our test suite, as well as improving API convenience a bit.

## New and improved functionality

- Should now work on Windows.
- `Message` now provides the `Eq` trait.
- `From<Error>` is now provided for `std::io::Error` (issue #136).
- There is now a type alias `PollEvents` mapping to `i16`. Use that
  instead of `i16` to refer to a set of poll events; in 0.9,
  `PollEvents` will become a separate type.
- `PollItem` now has methods `is_readable`, `is_writable` and
  `is_error`; use those in preference to using bit masking operations
  when applicable.
- New example zguide example `mspoller`.
- Some documentation improvements.
- There is now a Unix-specific test integrating `zmq` with Unix
  `poll(2)`.

## Incompatible API changes

There has been a minor API change that was deemed necessary for
cross-platform support and to fix broken parts of the API:

- There is now an internal `RawFd` type alias that maps to `RawFd` on
  Unixoids and `RawSocket` on Windows. `Socket::get_fd()` and
  `PollItem::from_fd()` now use that instead of `i64` and `c_int`,
  respectively.

# 0.8.0

This is a feature and bugfix release. The API has changed, partially
in slightly incompatible ways. Typical code, for some suitable value
of "typical", should still compile when it did with 0.7, but expect a
lot of warnings due to reduced mutability requirements for `Socket`
and `Context`.

Note that this release adds initial support for the Windows platform
(PR #124). While the code now compiles, the test suite is still broken
under Windows (issue #125).

Since these release notes have been assembled post-hoc, they are
highly likely to be incomplete.

## Incompatible API changes

The following methods of `Socket` changed:

- The deprecated `close()` method has been removed.
- `to_raw()` is now known as `into_raw()`.
- `borrow_raw()` is known as `as_mut_ptr()`, and takes a `&mut self`
  now.

## New and improved functionality

Note that the added `CURVE` and `GSSAPI` parts of the API are
conditional, depending on the compile-time detected capabilities of
libzmq.

- Most methods of `Socket` no longer require `&mut self`.
- `Context` now can be shared across threads
- rust-zmq should now work across a wider range of libzmq versions.
- More functions now have minimal documentation, but there is still
  lots of improvements to make in this area.
- Initial API coverage for encryption via the `Mechanism` and
  `CurveKeyPair` data types.
- Wrappers for the Z85 codec (`z85_encode` and `z85_decode`).
- New socket option accessors for:
  - `ZMQ_LAST_ENDPOINT`
  - `ZMQ_IMMEDIATE`
  - `ZMQ_PROBE_ROUTER`, `ZMQ_ROUTER_MANDATORY`
  - `ZMQ_RCVTIMEO`, `ZMQ_SNDTIMEO`
  - Various Kerberos (aka `GSSAPI`) and encryption-related (aka
    `CURVE`) options.
- New zguide examples `fileio3`, `msreader`, `rtdealer`, `lvcache`,
  `pathopub` and `pathosub`.
- There now is a test suite.

## Deprecations

`Constants` will be removed from public API in the next release; it
should not be needed in client code, since corresponding functionality
is provided in a higher-level form.

## Bugfixes

Yes, there have been bugs that were fixed; hopefully for the next
releases, a reasonably accurate list of those will be provided.

## Internals

Some redundancy in error handling and socket option handling has been
abstracted over using macros.
