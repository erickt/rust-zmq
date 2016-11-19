extern crate zmq;

use std::thread;
use std::str;

#[test]
fn test_inproc() {
    shared_context("inproc://pub");
}

#[test]
fn test_tcp() {
    shared_context("tcp://127.0.0.1:*");
}

fn shared_context(address: &str) {
    let ctx = zmq::Context::new();

    let mut push_socket = ctx.socket(zmq::PUSH).unwrap();
    push_socket.bind(address).unwrap();
    let endpoint = push_socket.get_last_endpoint().unwrap().unwrap();
    let worker1 = fork(&ctx, endpoint);

    push_socket.send("Message1".as_bytes(), 0).unwrap();

    worker1.join().unwrap();
}

fn fork(ctx: &zmq::Context, endpoint: String) -> thread::JoinHandle<()> {
    let w_ctx = ctx.clone();
    thread::spawn(move || { worker(&w_ctx, &endpoint); })
}

fn worker(ctx: &zmq::Context, endpoint: &str) {
    let mut pull_socket = connect_socket(ctx, zmq::PULL, endpoint).unwrap();

    let mut msg = zmq::Message::new().unwrap();
    pull_socket.recv(&mut msg, 0).unwrap();
    assert_eq!(&msg[..], "Message1".as_bytes());
}

fn connect_socket<'a>(ctx: &'a zmq::Context,
                      typ: zmq::SocketType,
                      address: &str) -> Result<zmq::Socket, zmq::Error> {
    ctx.socket(typ)
        .and_then(|mut socket| socket.connect(address).map(|_| socket))
}
