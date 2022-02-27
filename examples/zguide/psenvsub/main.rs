#![crate_name = "psenvsub"]

fn main() {
    let context = zmq2::Context::new();
    let subscriber = context.socket(zmq2::SUB).unwrap();
    subscriber
        .connect("tcp://localhost:5563")
        .expect("failed connecting subscriber");
    subscriber.set_subscribe(b"B").expect("failed subscribing");

    loop {
        let envelope = subscriber
            .recv_string(0)
            .expect("failed receiving envelope")
            .unwrap();
        let message = subscriber
            .recv_string(0)
            .expect("failed receiving message")
            .unwrap();
        println!("[{}] {}", envelope, message);
    }
}
