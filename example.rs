extern mod std;
extern mod zmq;

use std::io;
use std::str;
use std::task;

fn new_server(socket: zmq::Socket) {
    let msg = socket.recv_str(0).unwrap();
    io::println(fmt!("server received %?", msg));

    let msg = fmt!("hello %?", msg);
    io::println(fmt!("server sending %?", msg));

    match socket.send_str(msg, 0) {
        Ok(()) => { },
        Err(e) => fail!(e.to_str())
    }
}

fn new_client(socket: zmq::Socket) {
    io::println("starting client");

    socket.set_sndhwm(10).unwrap();
    socket.set_rcvhwm(10).unwrap();
    io::println(fmt!("rcvhwm: %?", socket.get_rcvhwm().unwrap()));
    io::println(fmt!("sndhwm: %?", socket.get_sndhwm().unwrap()));

    socket.set_identity("identity".as_bytes()).unwrap();

    let identity = socket.get_identity().unwrap();
    io::println(fmt!("identity: %?", str::from_utf8(identity)));

    let msg = "foo";
    io::println(fmt!("client sending %?", msg));
    socket.send_str(msg, 0).unwrap();

    let msg = socket.recv_str(0).unwrap();
    io::println(fmt!("client recieving %?", msg));
}

fn main() {
    let (major, minor, patch) = zmq::version();

    io::println(fmt!("version: %d %d %d", major, minor, patch));

    let ctx = zmq::Context::new();

    let server_socket = ctx.socket(zmq::REP).unwrap();
    let client_socket = ctx.socket(zmq::REQ).unwrap();

    // Connect the two sockets to each other.
    server_socket.bind("tcp://127.0.0.1:3456").unwrap();
    client_socket.connect("tcp://127.0.0.1:3456").unwrap();

    // We need to start the server in a separate thread as it blocks.
    let mut task = task::task();
    task.sched_mode(task::SingleThreaded);
    task.spawn_with(server_socket, new_server);

    new_client(client_socket);
}
