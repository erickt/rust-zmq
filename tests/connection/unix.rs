// Test integration of zmq with a simple external event loop
//
// This excercises the `Socket::get_fd()` method in combination with
// `Socket::get_events()` to integrate with Unix `poll(2)` to check
// the basis for integration with external event loops works.

extern crate nix;
extern crate zmq;

use super::with_connection;
use self::nix::poll;

test!(test_external_poll_inproc, {
    with_connection("inproc://test-poll",
                    zmq::REQ, poll_client,
                    zmq::REP, poll_worker);
});

test!(test_external_poll_ipc, {
    with_connection("ipc:///tmp/zmq-tokio-test",
                    zmq::REQ, poll_client,
                    zmq::REP, poll_worker);
});

test!(test_external_poll_tcp, {
    with_connection("tcp://127.0.0.1:*",
                    zmq::REQ, poll_client,
                    zmq::REP, poll_worker);
});

fn poll_client(_ctx: zmq::Context, socket: zmq::Socket) {
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
    socket: &'a zmq::Socket,
    fds: [poll::PollFd; 1],
}

impl<'a> PollState<'a> {
    fn new(socket: &'a zmq::Socket) -> Self {
        let fd = socket.get_fd().unwrap();
        PollState {
            socket: socket,
            fds: [poll::PollFd::new(fd, poll::POLLIN, poll::EventFlags::empty())],
        }
    }

    /// Wait for one of `events` to happen.
    fn wait(&mut self, events: zmq::PollEvents) {
        while !(self.events().intersects(events)) {
            debug!("polling");
            let fds = &mut self.fds;
            poll::poll(fds, -1).unwrap();
            debug!("poll done, events: {:?}", fds[0].revents());
            match fds[0].revents() {
                Some(events) => {
                    if !events.contains(poll::POLLIN) {
                        continue;
                    }
                },
                _ => continue,
            }
        }
    }

    fn events(&self) -> zmq::PollEvents {
        self.socket.get_events().unwrap() as zmq::PollEvents
    }
}

fn poll_worker(_ctx: zmq::Context, socket: zmq::Socket) {
    let mut reply = None;
    let mut state = PollState::new(&socket);
    loop {
        match reply.take() {
            None => {
                state.wait(zmq::POLLIN);
                let msg = socket.recv_msg(zmq::DONTWAIT).unwrap();
                reply = Some(msg);
            },
            Some(msg) => {
                state.wait(zmq::POLLOUT);
                let done = msg.len() == 0;
                socket.send(msg, zmq::DONTWAIT).unwrap();
                if done {
                    break;
                }
            },
        }
    }
}
