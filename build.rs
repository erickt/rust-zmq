extern crate zmq_pw_sys as zmq;

fn main() {
    use std::ffi::CString;

	for has in ["ipc", "pgm", "tipc", "norm", "curve", "gssapi"].into_iter() {
		if unsafe { zmq::zmq_has(CString::new(has.as_bytes()).unwrap().as_ptr()) } == 1 {
			println!("cargo:rustc-cfg=ZMQ_HAS_{}=\"1\"", has.to_uppercase());
		}
	}
}
