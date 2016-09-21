extern crate zmq_has;
use zmq_has::zmq_capabilities;

fn main(){

    println!("zmq has these capablities: {:?}", zmq_capabilities());
}
