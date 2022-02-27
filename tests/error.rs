use zmq2::*;
use zmq_sys2::errno;

#[test]
fn from_raw_eintr() {
    let error = Error::from_raw(errno::EINTR);
    assert_eq!(error, Error::EINTR);
}
