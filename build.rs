use std::ffi::CString;
use std::os::raw::{c_char, c_int};

#[cfg(feature = "zmq_has")]
fn main() {
	for has in ["ipc", "pgm", "tipc", "norm", "curve", "gssapi"].into_iter() {
		if unsafe { zmq_has(CString::new(has.as_bytes()).unwrap().as_ptr()) } == 1 {
			println!("cargo:rustc-cfg=ZMQ_HAS_{}=\"1\"", has.to_uppercase());
		}
	}
}

#[cfg(not(feature = "zmq_has"))]
fn main() {
	
}

#[cfg(feature = "zmq_has")]
#[link(name = "zmq")]
extern "C" {
	fn zmq_has(capability: *const c_char) -> c_int;
}
