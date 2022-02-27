#![crate_name = "msgqueue"]

fn main() {
    let context = zmq2::Context::new();
    let frontend = context.socket(zmq2::ROUTER).unwrap();
    let backend = context.socket(zmq2::DEALER).unwrap();

    frontend
        .bind("tcp://*:5559")
        .expect("failed binding frontend");
    backend
        .bind("tcp://*:5560")
        .expect("failed binding backend");

    zmq2::proxy(&frontend, &backend).expect("failed to proxy");
}
