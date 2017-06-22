extern crate zmq_pw as zmq;
extern crate zmq_pw_sys as zmq_sys;

use zmq::*;
use zmq_sys::errno;

#[test]
fn from_raw_eintr() {
  let error = Error::from_raw(errno::EINTR);
  assert_eq!(error, Error::EINTR);
}
