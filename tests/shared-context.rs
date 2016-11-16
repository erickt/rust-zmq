extern crate zmq;

use std::thread;
use std::str;

#[test]
fn test_shared_context() {
    let ctx = zmq::Context::new();

    let address = "inproc://pub";
    let mut push_socket = ctx.socket(zmq::PUSH).unwrap();
    push_socket.bind(address).unwrap();
    let worker1 = fork(&ctx);

    push_socket.send("Message1".as_bytes(), 0).unwrap();

    worker1.join().unwrap();
}

fn fork(ctx: &zmq::Context) -> thread::JoinHandle<()> {
    let w_ctx = ctx.clone();
    thread::spawn(move || { worker(&w_ctx); })
}

fn worker(ctx: &zmq::Context) {
    let mut pull_socket = connect_socket(ctx, zmq::PULL, "inproc://pub").unwrap();

    let mut msg = zmq::Message::new().unwrap();
    pull_socket.recv(&mut msg, 0).unwrap();
    assert_eq!(&msg[..], "Message1".as_bytes());
}

fn connect_socket<'a>(ctx: &'a zmq::Context,
                      typ: zmq::SocketType,
                      address: &str) -> Result<zmq::Socket, zmq::Error> {
    ctx.socket(typ)
        .and_then(|mut socket| match socket.connect(address) {
            Ok(()) => Ok(socket),
            Err(e) => Err(e)
        })
}
