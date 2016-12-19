//! These are all tests using PUSH/PULL sockets created from a shared
//! context to connect two threads. As a compile-time test, this
//! creates one socket from a context, and passes this context to the
//! child thread, along with the endpoint address to connect to. The
//! second socket is the created in the child thread.

extern crate timebomb;
extern crate zmq;

use std::thread;
use std::str;
use timebomb::timeout_ms;

// TODO: factor out `test!` and use it here

#[test]
fn test_inproc() {
    timeout_ms(|| {
        run("inproc://pub", check_recv);
    }, 10000);
}

#[test]
fn test_tcp() {
    timeout_ms(|| {
        run("tcp://127.0.0.1:*", check_recv);
    }, 10000);
}

#[test]
fn test_poll_inproc() {
    timeout_ms(|| {
        run("inproc://pub", check_poll);
    }, 10000);
}

#[test]
fn test_poll_tcp() {
    timeout_ms(|| {
        run("tcp://127.0.0.1:*", check_poll);
    }, 10000);
}

fn run<F>(address: &str, worker: F)
    where F: Fn(zmq::Context, zmq::Socket) + Send + 'static
{
    let ctx = zmq::Context::new();

    let push_socket = ctx.socket(zmq::PUSH).unwrap();
    push_socket.bind(address).unwrap();
    let endpoint = push_socket.get_last_endpoint().unwrap().unwrap();

    let child = {
        let w_ctx = ctx.clone();
        thread::spawn(move || {
            let pull_socket = connect_socket(&w_ctx, zmq::PULL, &endpoint).unwrap();
            worker(w_ctx, pull_socket);
        })
    };

    push_socket.send("Message1", 0).unwrap();

    child.join().unwrap();
}

fn check_poll(ctx: zmq::Context, pull_socket: zmq::Socket) {
    {
        let mut poll_items = vec![pull_socket.as_poll_item(zmq::POLLIN)];
        assert_eq!(zmq::poll(&mut poll_items, 1000).unwrap(), 1);
        assert_eq!(poll_items[0].get_revents(), zmq::POLLIN);
    }

    check_recv(ctx, pull_socket);
}

fn check_recv(_ctx: zmq::Context, pull_socket: zmq::Socket) {
    let msg = pull_socket.recv_msg(0).unwrap();
    assert_eq!(&msg[..], b"Message1");
}

fn connect_socket(ctx: &zmq::Context,
                  typ: zmq::SocketType,
                  address: &str) -> Result<zmq::Socket, zmq::Error> {
    ctx.socket(typ).and_then(|socket| socket.connect(address).map(|_| socket))
}
