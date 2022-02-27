// Test integration of zmq with a simple external event loop
//
// This excercises the `Socket::get_fd()` method in combination with
// `Socket::get_events()` to integrate with Unix `poll(2)` to check
// the basis for integration with external event loops works.

use log::debug;
use nix::poll::{self, PollFlags};

use super::with_connection;

test!(test_external_poll_inproc, {
    with_connection(
        "inproc://test-poll",
        zmq2::REQ,
        poll_client,
        zmq2::REP,
        poll_worker,
    );
});

test!(test_external_poll_ipc, {
    with_connection(
        "ipc:///tmp/zmq-tokio-test",
        zmq2::REQ,
        poll_client,
        zmq2::REP,
        poll_worker,
    );
});

test!(test_external_poll_tcp, {
    with_connection(
        "tcp://127.0.0.1:*",
        zmq2::REQ,
        poll_client,
        zmq2::REP,
        poll_worker,
    );
});

fn poll_client(_ctx: &zmq2::Context, socket: &zmq2::Socket) {
    // TODO: we should use `poll::poll()` here as well.
    for i in 0..10 {
        let payload = format!("message {}", i);
        socket.send(&payload, 0).unwrap();
        let reply = socket.recv_msg(0).unwrap();
        assert_eq!(payload.as_bytes(), &reply[..]);
    }
    socket.send("", 0).unwrap();
    let last = socket.recv_msg(0).unwrap();
    assert_eq!(b"", &last[..]);
}

/// Keeps track of the polling state for the event signalling FD of a
/// single socket.
struct PollState<'a> {
    socket: &'a zmq2::Socket,
    fds: [poll::PollFd; 1],
}

impl<'a> PollState<'a> {
    fn new(socket: &'a zmq2::Socket) -> Self {
        let fd = socket.get_fd().unwrap();
        PollState {
            socket,
            fds: [poll::PollFd::new(fd, PollFlags::POLLIN)],
        }
    }

    /// Wait for one of `events` to happen.
    fn wait(&mut self, events: zmq2::PollEvents) {
        while !(self.events().intersects(events)) {
            debug!("polling");
            let fds = &mut self.fds;
            poll::poll(fds, -1).unwrap();
            debug!("poll done, events: {:?}", fds[0].revents());
            match fds[0].revents() {
                Some(events) => {
                    if !events.contains(PollFlags::POLLIN) {
                        continue;
                    }
                }
                _ => continue,
            }
        }
    }

    fn events(&self) -> zmq2::PollEvents {
        self.socket.get_events().unwrap() as zmq2::PollEvents
    }
}

fn poll_worker(_ctx: &zmq2::Context, socket: &zmq2::Socket) {
    let mut reply = None;
    let mut state = PollState::new(&socket);
    loop {
        match reply.take() {
            None => {
                state.wait(zmq2::POLLIN);
                let msg = socket.recv_msg(zmq2::DONTWAIT).unwrap();
                reply = Some(msg);
            }
            Some(msg) => {
                state.wait(zmq2::POLLOUT);
                let done = msg.len() == 0;
                socket.send(msg, zmq2::DONTWAIT).unwrap();
                if done {
                    break;
                }
            }
        }
    }
}
