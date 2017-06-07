#![crate_name = "rrclient"]

extern crate zmq;

fn main(){
    let context = zmq::Context::new();
    let requester = context.socket(zmq::REQ).unwrap();
    requester.connect("tcp://localhost:5559").expect("failed to connect requester");
    for request_nbr in 0..10{
        requester.send("Hello",0).unwrap();
        let message = requester.recv_msg(0).unwrap();
        println! ("Received reply {} {}", request_nbr, message.as_str().unwrap());

    }
}
