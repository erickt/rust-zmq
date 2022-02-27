use std::thread;

macro_rules! t {
    ($e:expr) => (
        $e.unwrap_or_else(|e| { panic!("{} failed with {:?}", stringify!($e), e) })
    )
}

fn main() {
    let mut context = zmq2::Context::new();
    let socket = t!(context.socket(zmq2::REP));
    let s = &socket;
    let t = thread::spawn(move || {
        t!(s.bind("tcp://127.0.0.1:12345"))
    });
    socket.send("ABC", 0);
    t.join().unwrap();
}
