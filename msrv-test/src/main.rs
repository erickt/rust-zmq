fn main() {
    let ctx = zmq2::Context::new();

    let socket = ctx.socket(zmq2::REQ).unwrap();
    socket.connect("tcp://127.0.0.1:1234").unwrap();
    socket.send("hello world!", 0).unwrap();
}
