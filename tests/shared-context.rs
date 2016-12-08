extern crate timebomb;
extern crate zmq;

use std::thread;
use std::str;
use timebomb::timeout_ms;

#[test]
fn test_inproc() {
    timeout_ms(|| {
        shared_context("inproc://pub");
    }, 10000);
}

#[test]
fn test_tcp() {
    timeout_ms(|| {
        shared_context("tcp://127.0.0.1:*");
    }, 10000);
}

fn shared_context(address: &str) {
    let ctx = zmq::Context::new();

    let push_socket = ctx.socket(zmq::PUSH).unwrap();
    push_socket.bind(address).unwrap();
    let endpoint = push_socket.get_last_endpoint().unwrap().unwrap();
    let worker1 = fork(&ctx, endpoint);

    push_socket.send(b"Message1", 0).unwrap();

    worker1.join().unwrap();
}

fn fork(ctx: &zmq::Context, endpoint: String) -> thread::JoinHandle<()> {
    let w_ctx = ctx.clone();
    thread::spawn(move || { worker(&w_ctx, &endpoint); })
}

fn worker(ctx: &zmq::Context, endpoint: &str) {
    let pull_socket = connect_socket(ctx, zmq::PULL, endpoint).unwrap();

    let msg = pull_socket.recv_msg(0).unwrap();
    assert_eq!(&msg[..], b"Message1");
}

fn connect_socket(ctx: &zmq::Context,
                  typ: zmq::SocketType,
                  address: &str) -> Result<zmq::Socket, zmq::Error> {
    ctx.socket(typ).and_then(|socket| socket.connect(address).map(|_| socket))
}
