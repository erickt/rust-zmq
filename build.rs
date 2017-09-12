extern crate zmq_sys as zmq;

#[cfg(feature = "zmq_has")]
fn main() {
    use std::ffi::CString;

	for has in ["ipc", "pgm", "tipc", "norm", "curve", "gssapi"].into_iter() {
		if unsafe { zmq::zmq_has(CString::new(has.as_bytes()).unwrap().as_ptr()) } == 1 {
			println!("cargo:rustc-cfg=ZMQ_HAS_{}=\"1\"", has.to_uppercase());
		}
	}
}

#[cfg(not(feature = "zmq_has"))]
fn main() {
    use std::mem::size_of;
    use std::os::raw::c_int;

    const ZMQ_CURVE_SERVER: c_int = 47;
    const ZMQ_GSSAPI_SERVER: c_int = 62;
    const ZMQ_REQ: c_int = 3;

    // As long as we support pre-4.1 versions of libzmq, we can't use zmq_has()
    // here because that would make the build script fail to link.
    //
    // Fortunately, the relevant capabilities can be easily probed by setting
    // socket options.

    unsafe {
        let ctx = zmq::zmq_ctx_new();
        assert!(!ctx.is_null());

        for &(opt, feature) in &[(ZMQ_CURVE_SERVER, "curve"),
                                 (ZMQ_GSSAPI_SERVER, "gssapi")] {
            let sock = zmq::zmq_socket(ctx, ZMQ_REQ);
            assert!(!sock.is_null());
            let mut one: c_int = 1;
            let rc = zmq::zmq_setsockopt(sock, opt, &mut one as *mut c_int as *mut _,
                                         size_of::<c_int>());
            if rc == -1 {
                assert!(zmq::zmq_errno() == zmq::errno::EINVAL);
            } else {
                println!("cargo:rustc-cfg=ZMQ_HAS_{}=\"1\"", feature.to_uppercase());
            }
        }

        // Determine if we can wrap zmq_has() in the crate.
        let mut major = 0;
        let mut minor = 0;
        let mut _patch = 0;
        zmq::zmq_version(&mut major, &mut minor, &mut _patch);
        if major >= 4 && minor >= 1 {
            println!("cargo:rust-cfg=ZMQ_HAS_ZMQ_HAS=\"1\"");
        }
    }
}
