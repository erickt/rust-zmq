#![crate_name = "monitor"]



extern crate zmq;
use std::str;

/// Read one event off the monitor socket; return the SocketEvent value.
fn get_monitor_event(monitor: &mut zmq::Socket)
    -> Result<zmq::SocketEvent, zmq::Error>
{
    let mut msg = zmq::Message::new();
    monitor.recv(&mut msg, 0)?;
    let event= ((msg[1] as u16) << 8) | msg[0] as u16;

    assert!(monitor.get_rcvmore()?,
            "Monitor socket should have two messages per event");

    // the address, we'll ignore it
    monitor.recv(&mut msg, 0)?;

    Ok(zmq::SocketEvent::from_raw(event))
}

/// Send a series of pings between the client and the server.
/// The messages should round trip from the client to the server
/// and back again.
fn bounce(client: &mut zmq::Socket, server: &mut zmq::Socket) {
    let data = "12345678ABCDEFGH12345678abcdefgh";

    //  Send message from client to server
    client.send(data.as_bytes(), zmq::SNDMORE).unwrap();
    client.send(data.as_bytes(), 0).unwrap();

    //  Receive message at server side
    let mut recv_data = server.recv_bytes(0).unwrap();
    assert_eq!(str::from_utf8(&recv_data).unwrap(), data);
    assert!(server.get_rcvmore().unwrap());

    recv_data = server.recv_bytes(0).unwrap();
    assert_eq!(str::from_utf8(&recv_data).unwrap(), data);
    assert!(!server.get_rcvmore().unwrap());

    //  Send message from client to server
    server.send(&recv_data, zmq::SNDMORE).unwrap();
    server.send(&recv_data, 0).unwrap();

    //  Receive the two parts at the client side
    recv_data = client.recv_bytes(0).unwrap();
    assert_eq!(str::from_utf8(&recv_data).unwrap(), data);
    assert!(client.get_rcvmore().unwrap());

    recv_data = client.recv_bytes(0).unwrap();
    assert_eq!(str::from_utf8(&recv_data).unwrap(), data);
    assert!(!client.get_rcvmore().unwrap());
}

/// Close the given socket with LINGER set to 0
fn close_zero_linger(socket: &mut zmq::Socket) {
    socket.set_linger(0).unwrap();
    drop(socket);
}

fn main() {
    let ctx = zmq::Context::new();

    let mut client = ctx.socket(zmq::DEALER).unwrap();
    let mut server = ctx.socket(zmq::DEALER).unwrap();

    let err = client.monitor("tcp://127.0.0.1:9999", 0).expect_err(
        "Socket monitoring only works over inproc://");
    assert_eq!(zmq::Error::EPROTONOSUPPORT, err);

    assert!(client.monitor("inproc://monitor-client",
                           zmq::SocketEvent::ALL as i32).is_ok());
    assert!(server.monitor("inproc://monitor-server",
                           zmq::SocketEvent::ALL as i32).is_ok());

    let mut client_mon = ctx.socket(zmq::PAIR).unwrap();
    let mut server_mon = ctx.socket(zmq::PAIR).unwrap();

    // Connect these to the inproc endpoints so they'll get events
    client_mon.connect("inproc://monitor-client").unwrap();
    server_mon.connect("inproc://monitor-server").unwrap();

    // Now do a basic ping test
    server.bind("tcp://127.0.0.1:9998").unwrap();
    client.connect("tcp://127.0.0.1:9998").unwrap();
    bounce(&mut client, &mut server);

    // Close client and server
    close_zero_linger(&mut client);
    close_zero_linger(&mut server);

    // Now collect and check events from both sockets
    let mut event = get_monitor_event(&mut client_mon).unwrap();
    println!("got client monitor event {:?}", event);
    if event == zmq::SocketEvent::CONNECT_DELAYED {
        event = get_monitor_event(&mut client_mon).unwrap();
        println!("got client monitor event {:?}", event);
    }
    assert_eq!(zmq::SocketEvent::CONNECTED, event);

    event = get_monitor_event(&mut client_mon).unwrap();
    assert_eq!(zmq::SocketEvent::MONITOR_STOPPED, event);
    println!("got client monitor event {:?}", event);

    // This is the flow of server events
    event = get_monitor_event(&mut server).unwrap();
    println!("got server monitor event {:?}", event);
    assert_eq!(zmq::SocketEvent::LISTENING, event);

    event = get_monitor_event(&mut server).unwrap();
    println!("got server monitor event {:?}", event);
    assert_eq!(zmq::SocketEvent::ACCEPTED, event);

    event = get_monitor_event(&mut server).unwrap();
    println!("got server monitor event {:?}", event);
    assert_eq!(zmq::SocketEvent::CLOSED, event);

    event = get_monitor_event(&mut server).unwrap();
    println!("got server monitor event {:?}", event);
    assert_eq!(zmq::SocketEvent::MONITOR_STOPPED, event);

    // Close down the sockets
    close_zero_linger(&mut client_mon);
    close_zero_linger(&mut server_mon);
}
