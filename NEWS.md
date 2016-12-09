# 0.8.0

This is feature and bugfix release. The API has changed, partially in
slightly incompatible ways. Typical code, for some suitable value of
"typical", should still compile when it did with 0.7, but expect a lot
of warnings due to reduced mutability requirements for `Socket` and
`Context`.

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
