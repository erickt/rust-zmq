/*!
 * Weather update client
 * Connects SUB socket to tcp://localhost:5556
 * Collects weather updates and find avg temp in zipcode
 */

extern mod zmq;

fn atoi(s: &str) -> int {
    from_str(s).unwrap()
}

fn main() {
    println("Collecting updates from weather server...");

    let context = zmq::Context::new();
    let subscriber = context.socket(zmq::SUB).unwrap();
    assert!(subscriber.connect("tcp://localhost:5556").is_ok());

    let args = std::os::args();
    let filter = if args.len() > 1 { args[1] } else { ~"10001" };
    assert!(subscriber.set_subscribe(filter.as_bytes()).is_ok());

    let mut total_temp = 0;

    100.times( ||{
        let string = subscriber.recv_str(0).unwrap();
        let chks = string.split(' ').to_owned_vec();
        let (_zipcode, temperature, _relhumidity) = (atoi(chks[0]), atoi(chks[1]), atoi(chks[2]));
        total_temp += temperature;
    });

    println!("Average temperature for zipcode '{}' was {}F", filter, (total_temp / 100) as int);
}
