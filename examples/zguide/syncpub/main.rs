#![crate_name = "syncpub"]

extern crate zmq;

fn main() {
    let context = zmq::Context::new();

    //socket to talk to clients
    let publisher = context.socket(zmq::PUB).unwrap();
    publisher.set_sndhwm(1100000).expect("failed setting hwm");
    publisher
        .bind("tcp://*:5561")
        .expect("failed binding publisher");

    //socket to receive signals
    let syncservice = context.socket(zmq::REP).unwrap();
    syncservice
        .bind("tcp://*:5562")
        .expect("failed binding syncservice");

    //get syncronization from subscribers
    println!("Waiting for subscribers");
    for _ in 0..10 {
        syncservice.recv_msg(0).expect("failed receiving sync");
        syncservice.send("", 0).expect("failed sending sync");
    }
    //now broadcast 1M updates followed by end
    println!("Broadcasting messages");
    for _ in 0..1000000 {
        publisher.send("Rhubarb", 0).expect("failed broadcasting");
    }
    publisher.send("END", 0).expect("failed sending end");
}
