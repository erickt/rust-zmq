extern crate zmq;

use std::thread;
use std::sync::Arc;
use std::str;

fn main() {
	// create a context, wrap in an arc to make it shareable
	let ctx = Arc::new(zmq::Context::new());
	
	let address = "tcp://*:5555";
	let mut push_socket = ctx.socket(zmq::PUSH)
				.and_then(|mut socket| match socket.bind(address) {
					Ok(()) => Ok(socket),
					Err(e) => Err(e)
				}).unwrap();
	let w1_ctx = ctx.clone();
	let worker1 = thread::spawn( move || { worker(&w1_ctx); });
	
	push_socket.send("Message1".as_bytes(), 0).unwrap();

	let res1 = worker1.join();
}

fn worker(ctx: &zmq::Context) {
	let mut pull_socket = connect_socket(ctx, zmq::PULL, "tcp://localhost:5555").unwrap();
	
	let mut msg = zmq::Message::new().unwrap();
	match pull_socket.recv(&mut msg, 0) {
		Ok(()) => println!("{}", str::from_utf8(&*msg).unwrap()),
		Err(e) => panic!(e)
	}
}

fn connect_socket<'a>(ctx: &'a zmq::Context, typ: zmq::SocketType, address: &str) -> Result<zmq::Socket, zmq::Error> {
	ctx.socket(typ)
		.and_then(|mut socket| match socket.connect(address) {
			Ok(()) => Ok(socket),
			Err(e) => Err(e)
		})
}
