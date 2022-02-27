#[macro_use]
mod common;

use zmq2::{z85_decode, Context, CurveKeyPair, Socket};

fn create_socketpair() -> (Socket, Socket) {
    let ctx = Context::default();
    let sender = ctx.socket(zmq2::REQ).unwrap();
    let receiver = ctx.socket(zmq2::REP).unwrap();
    let server_pair = CurveKeyPair::new().unwrap();
    let client_pair = CurveKeyPair::new().unwrap();

    // receiver socket acts as server, will accept connections
    receiver.set_curve_server(true).unwrap();
    receiver
        .set_curve_secretkey(&server_pair.secret_key)
        .unwrap();

    // sender socket, acts as client
    sender.set_curve_serverkey(&server_pair.public_key).unwrap();
    sender.set_curve_publickey(&client_pair.public_key).unwrap();
    sender.set_curve_secretkey(&client_pair.secret_key).unwrap();

    receiver.bind("tcp://127.0.0.1:*").unwrap();
    let ep = receiver.get_last_endpoint().unwrap().unwrap();
    sender.connect(&ep).unwrap();
    (sender, receiver)
}

test_capability!(test_curve_messages, "curve", {
    let (sender, receiver) = create_socketpair();
    sender.send("foo", 0).unwrap();
    let msg = receiver.recv_msg(0).unwrap();
    assert_eq!(&msg[..], b"foo");
    assert_eq!(msg.as_str(), Some("foo"));
    println!("this is it {0}", msg.as_str().unwrap());
    assert_eq!(format!("{:?}", msg), "[102, 111, 111]");
    receiver.send("bar", 0).unwrap();
    let msg = sender.recv_msg(0).unwrap();
    assert_eq!(&msg[..], b"bar");
});

test_capability!(test_curve_keypair, "curve", {
    let keypair = CurveKeyPair::new().unwrap();
    assert!(keypair.public_key.len() == 32);
    assert!(keypair.secret_key.len() == 32);
});

test_capability!(test_getset_curve_server, "curve", {
    let ctx = Context::new();
    let sock = ctx.socket(zmq2::REQ).unwrap();
    sock.set_curve_server(true).unwrap();
    assert_eq!(sock.is_curve_server().unwrap(), true);
});

test_capability!(test_getset_curve_publickey, "curve", {
    let ctx = Context::new();
    let sock = ctx.socket(zmq2::REQ).unwrap();
    let key = z85_decode("FX5b8g5ZnOk7$Q}^)Y&?.v3&MIe+]OU7DTKynkUL").unwrap();
    sock.set_curve_publickey(&key).unwrap();
    assert_eq!(sock.get_curve_publickey().unwrap(), key);
});

test_capability!(test_getset_curve_secretkey, "curve", {
    let ctx = Context::new();
    let sock = ctx.socket(zmq2::REQ).unwrap();
    let key = z85_decode("s9N%S3*NKSU$6pUnpBI&K5HBd[]G$Y3yrK?mhdbS").unwrap();
    sock.set_curve_secretkey(&key).unwrap();
    assert_eq!(sock.get_curve_secretkey().unwrap(), key);
});

test_capability!(test_getset_curve_serverkey, "curve", {
    let ctx = Context::new();
    let sock = ctx.socket(zmq2::REQ).unwrap();
    let key = z85_decode("FX5b8g5ZnOk7$Q}^)Y&?.v3&MIe+]OU7DTKynkUL").unwrap();
    sock.set_curve_serverkey(&key).unwrap();
    assert_eq!(sock.get_curve_serverkey().unwrap(), key);
});
