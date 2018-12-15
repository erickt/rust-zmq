extern crate zmq;

#[test]
fn test_has() {
    if cfg!(ZMQ_HAS_ZMQ_HAS) {
        // It doesn't matter whether the feature is supported or not, it must return Some(_)
        assert!(zmq::has("ipc").is_some());
    }
}