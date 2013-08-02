extern mod zmq;

fn main() {
    let (major, minor, patch) = zmq::version();
    printfln!("Current 0MQ version is %d.%d.%d", major, minor, patch);
}
