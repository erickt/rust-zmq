extern mod std;
extern mod zmq;

use std::comm::{stream, Chan};
use std::io;
use std::str;
use std::task;

fn new_server(ctx: zmq::Context, ch: &Chan<()>) {
    let socket = ctx.socket(zmq::REP).unwrap();
    socket.bind("tcp://127.0.0.1:3456").get();

    let msg = socket.recv_str(0).unwrap();
    io::println(fmt!("received %s", msg));

    match socket.send_str(fmt!("hello %s", msg), 0) {
        Ok(()) => { },
        Err(e) => fail!(e.to_str())
    }

    // Let the main thread know we're done.
    ch.send(());
}

fn new_client(ctx: zmq::Context) {
    io::println("starting client");

    let socket = ctx.socket(zmq::REQ).unwrap();

    socket.set_hwm(10u64).get();
    io::println(fmt!("hwm: %?", socket.get_hwm().get()));

    socket.set_identity(str::to_bytes("identity")).get();

    let identity = socket.get_identity().unwrap();
    io::println(fmt!("identity: %s", str::from_bytes(identity)));

    io::println("client connecting to server");

    socket.connect("tcp://127.0.0.1:3456").get();
    socket.send_str("foo", 0).get();

    io::println(socket.recv_str(0).unwrap());
}

fn main() {
    let (major, minor, patch) = zmq::version();

    io::println(fmt!("version: %d %d %d", major, minor, patch));

    let ctx = zmq::init(1).unwrap();

    // We need to start the server in a separate scheduler as it blocks.
    let (po, ch) = stream();
    do task::spawn_sched(task::SingleThreaded) { new_server(ctx, &ch) }

    new_client(ctx);

    // Wait for the server to shut down.
    po.recv();

    ctx.term().get();
}
