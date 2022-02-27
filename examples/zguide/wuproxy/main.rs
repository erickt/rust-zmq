#![crate_name = "wuproxy"]

fn main() {
    let context = zmq2::Context::new();
    let frontend = context.socket(zmq2::XSUB).unwrap();
    let backend = context.socket(zmq2::XPUB).unwrap();

    frontend
        .connect("tcp://192.168.55.210:5556")
        .expect("failed connecting frontend");
    backend
        .bind("tcp://10.1.1.0:8100")
        .expect("failed binding backend");
    zmq2::proxy(&frontend, &backend).expect("failed proxying");
}
