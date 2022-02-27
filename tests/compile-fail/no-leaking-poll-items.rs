fn main() {
    let context = zmq2::Context::new();
    let _poll_item = {
        let socket = context.socket(zmq2::PAIR).unwrap();
        socket.as_poll_item(zmq2::POLLIN)
    }; //~^ ERROR `socket` does not live long enough [E0597]
}
