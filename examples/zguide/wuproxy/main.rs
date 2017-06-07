#![crate_name = "wuproxy"]

extern crate zmq;

fn main() {
    let context = zmq::Context::new();
    let frontend = context.socket(zmq::XSUB).unwrap();
    let backend = context.socket(zmq::XPUB).unwrap();

    frontend.connect("tcp://192.168.55.210:5556").expect("failed connecting frontend");
    backend.bind("tcp://10.1.1.0:8100").expect("failed binding backend");
    zmq::proxy(&frontend, &backend).expect("failed proxying");
}
