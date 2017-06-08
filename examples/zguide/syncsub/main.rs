#![crate_name = "syncsub"]

extern crate zmq;
use std::time::Duration;
use std::thread;

fn main() {
    let context = zmq::Context::new();

    //first connect our subscriber
    let subscriber = context.socket(zmq::SUB).unwrap();
    subscriber
        .connect("tcp://localhost:5561")
        .expect("failed connecting subscriber");
    subscriber
        .set_subscribe("".as_bytes())
        .expect("failed setting subscription");
    thread::sleep(Duration::from_millis(1));

    //second sync with publisher
    let syncclient = context.socket(zmq::REQ).unwrap();
    syncclient
        .connect("tcp://localhost:5562")
        .expect("failed connect syncclient");
    syncclient.send("", 0).expect("failed sending sync request");
    syncclient.recv_msg(0).expect("failed receiving sync reply");

    //third get our updates and report how many we got
    let mut update_nbr = 0;
    loop {
        let message = subscriber
            .recv_string(0)
            .expect("failed receiving update")
            .unwrap();
        if message == "END" {
            break;
        }
        update_nbr += 1;
    }
    println!("Received {} updates", update_nbr);
}
