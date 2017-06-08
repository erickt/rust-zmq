#![crate_name = "rrbroker"]

extern crate zmq;

fn main() {
    let context = zmq::Context::new();
    let frontend = context.socket(zmq::ROUTER).unwrap();
    let backend = context.socket(zmq::DEALER).unwrap();

    frontend.bind("tcp://*:5559").expect("failed binding frontend");
    backend.bind("tcp://*:5560").expect("failed binding backend");

    loop {
        let mut items = [
            frontend.as_poll_item(zmq::POLLIN),
            backend.as_poll_item(zmq::POLLIN),
        ];
        zmq::poll(&mut items, -1).unwrap();

        if items[0].is_readable() {
            loop {
                let message = frontend.recv_msg(0).unwrap();
                let more = message.get_more();
                backend.send(message, if more { zmq::SNDMORE } else { 0 }).unwrap();
                if !more {
                    break;
                }
            }
        }
        if items[1].is_readable() {
            loop {
                let message = backend.recv_msg(0).unwrap();
                let more = message.get_more();
                frontend.send(message, if more { zmq::SNDMORE } else { 0 }).unwrap();
                if !more {
                    break;
                }
            }
        }
    }
}
