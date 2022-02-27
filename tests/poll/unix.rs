// Test whether `zmq2::poll()` works with `PollItem`s constructed from
// arbitrary FDs.

use nix::unistd;
use std::os::unix::io::RawFd;
use std::thread;

#[test]
fn test_pipe_poll() {
    let (pipe_read, pipe_write) = unistd::pipe().expect("pipe creation failed");
    let writer_thread = thread::spawn(move || {
        pipe_writer(pipe_write);
    });
    let pipe_item = zmq2::PollItem::from_fd(pipe_read, zmq2::POLLIN);
    assert!(pipe_item.has_fd(pipe_read));

    let mut poll_items = [pipe_item];
    assert_eq!(zmq2::poll(&mut poll_items, 1000).unwrap(), 1);
    assert_eq!(poll_items[0].get_revents(), zmq2::POLLIN);

    let mut buf = vec![0];
    assert_eq!(unistd::read(pipe_read, &mut buf).unwrap(), 1);
    assert_eq!(buf, b"X");

    writer_thread.join().unwrap();
}

fn pipe_writer(fd: RawFd) {
    unistd::write(fd, b"X").expect("pipe write failed");
}
