#[test]
fn context_io_threads() {
    let ctx = zmq2::Context::new();

    assert_eq!(ctx.get_io_threads().unwrap(), zmq_sys2::ZMQ_IO_THREADS_DFLT as i32);

    ctx.set_io_threads(0).unwrap();
    assert_eq!(ctx.get_io_threads().unwrap(), 0);

    ctx.set_io_threads(7).unwrap();
    assert_eq!(ctx.get_io_threads().unwrap(), 7);

    assert!(ctx.set_io_threads(-1).is_err());
}
