#![crate_name = "version"]

fn main() {
    let (major, minor, patch) = zmq2::version();
    println!("Current 0MQ version is {}.{}.{}", major, minor, patch);
}
