extern crate zmq;

use std::thread;

macro_rules! t {
    ($e:expr) => (
        $e.unwrap_or_else(|e| { panic!("{} failed with {:?}", stringify!($e), e) })
    )
}

fn main() {
    let mut context = zmq::Context::new();
    let socket = t!(context.socket(zmq::REP));
    let s = &socket;
    let t = thread::spawn(move || {  //~ ERROR 15:13: 15:26: `*mut std::os::raw::c_void` cannot be shared between threads safely [E0277]
        t!(s.bind("tcp://127.0.0.1:12345"))
    });
    socket.send("ABC", 0);
    t.join().unwrap();
}
