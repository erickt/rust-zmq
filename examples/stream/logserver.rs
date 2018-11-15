// Very basic example to listen tcp socket from zmq using STREAM sockets
// You can use telnet to send messages and they will be output to console
// ZMQ_STREAM socket will prepend socket identity on message, that's why we use recv_multipart here

use std::str;
extern crate zmq;

fn main() {
    println!("Hello, world!");

    let ctx = zmq::Context::new();

    let socket = ctx.socket(zmq::STREAM).unwrap();
    socket.bind("tcp://*:8888").unwrap();
    loop {
        let data = socket.recv_multipart(0).unwrap();
        println!(
            "Identity: {:?} Message : {}",
            data[0],
            str::from_utf8(&data[1]).unwrap()
        );
    }
}
