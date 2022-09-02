#[test]
fn test_has() {
    // Until we can clean up the API `has` must return Some(_), not matter
    // wether the capability is actually supported or not.
    assert!(zmq::has("ipc").is_some());
}
