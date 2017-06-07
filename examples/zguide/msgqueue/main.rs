#![crate_name = "msgqueue"]

extern crate zmq;

fn main() {
    let context = zmq::Context::new();
    let frontend = context.socket(zmq::ROUTER).unwrap();
    let backend = context.socket(zmq::DEALER).unwrap();

    frontend.bind("tcp://*:5559").expect("failed binding frontend");
    backend.bind("tcp://*:5560").expect("failed binding backend");

    zmq::proxy(&frontend, &backend).expect("failed to proxy");
}
