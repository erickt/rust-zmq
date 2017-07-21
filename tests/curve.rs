extern crate zmq_pw as zmq;

#[macro_use]
mod common;

use zmq::{Context, CurveKeyPair, Message, Socket};

fn create_socketpair() -> (Socket, Socket) {
    let ctx = Context::default();
    let sender = ctx.socket(zmq::REQ).unwrap();
    let receiver = ctx.socket(zmq::REP).unwrap();
    let server_pair = CurveKeyPair::new().unwrap();
    let client_pair = CurveKeyPair::new().unwrap();

    // receiver socket acts as server, will accept connections
    receiver.set_curve_server(true).unwrap();
    receiver.set_curve_secretkey(&server_pair.secret_key).unwrap();

    // sender socket, acts as client
    sender.set_curve_serverkey(&server_pair.public_key).unwrap();
    sender.set_curve_publickey(&client_pair.public_key).unwrap();
    sender.set_curve_secretkey(&client_pair.secret_key).unwrap();

    receiver.bind("tcp://127.0.0.1:*").unwrap();
    let ep = receiver.get_last_endpoint().unwrap().unwrap();
    sender.connect(&ep).unwrap();
    (sender, receiver)
}

test!(test_curve_messages, {
    let (sender, receiver) = create_socketpair();
    sender.send("foo", 0).unwrap();
    let msg = receiver.recv_msg(0).unwrap();
    assert_eq!(&msg[..], b"foo");
    assert_eq!(msg.as_str(), Some("foo"));
    println!("this is it {0}", msg.as_str().unwrap());
    assert_eq!(format!("{:?}", msg), "[102, 111, 111]");
    receiver.send("bar", 0).unwrap();
    let mut msg = Message::with_capacity(1);
    sender.recv(&mut msg, 0).unwrap();
    assert_eq!(&msg[..], b"bar");
});
