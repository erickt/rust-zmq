extern crate zmq_has;
use zmq_has::zmq_capabilities;

fn main() {
	for has in zmq_capabilities().into_iter() {
			println!("cargo:rustc-cfg=ZMQ_HAS_{}=\"1\"", has);
	}
}