#![crate_name = "mtrelay"]

extern crate zmq;
use std::thread;

fn step1(context: &zmq::Context) {
    //connect to step 2 and tell it we're ready
    let xmitter = context.socket(zmq::PAIR).unwrap();
    xmitter
        .connect("inproc://step2")
        .expect("step 1 failed connecting");
    println!("Step 1 ready, signalling step 2");
    xmitter.send("READY", 0).expect("step 1 failed sending");
}

fn step2(context: &zmq::Context) {
    //bind inproc socket before starting step 1
    let receiver = context.socket(zmq::PAIR).unwrap();
    receiver
        .bind("inproc://step2")
        .expect("failed binding step 2");
    let ctx = context.clone();
    thread::spawn(move || step1(&ctx));

    //wait for signal and pass it on
    receiver.recv_msg(0).unwrap();

    //connect to step 3 and tell it we're ready
    let xmitter = context.socket(zmq::PAIR).unwrap();
    xmitter
        .connect("inproc://step3")
        .expect("step 2 failed connecting");
    println!("Step 2 ready, signalling step 3");
    xmitter.send("READY", 0).expect("step 2 failed sending");
}

fn main() {
    let context = zmq::Context::new();

    //bind inproc socket before starting step 2
    let receiver = context.socket(zmq::PAIR).unwrap();
    receiver
        .bind("inproc://step3")
        .expect("failed binding step 3");
    let ctx = context.clone();
    thread::spawn(move || step2(&ctx));
    //wait for signal and pass it on
    receiver.recv_msg(0).unwrap();
    println!("Test successful!");
}
