#[macro_use]
mod common;

use std::io;
use std::net::TcpStream;
use zmq::*;

fn version_ge_4_2() -> bool {
    let (major, minor, _) = version();
    (major > 4) || (major == 4 && minor >= 2)
}

fn create_socketpair() -> (Socket, Socket) {
    let ctx = Context::default();

    let sender = ctx.socket(zmq::REQ).unwrap();
    let receiver = ctx.socket(zmq::REP).unwrap();

    // Don't block forever
    sender.set_sndtimeo(1000).unwrap();
    sender.set_rcvtimeo(1000).unwrap();
    if version_ge_4_2() {
        sender.set_connect_timeout(1000).unwrap();
    }
    receiver.set_sndtimeo(1000).unwrap();
    receiver.set_rcvtimeo(1000).unwrap();

    receiver.bind("tcp://127.0.0.1:*").unwrap();
    let ep = receiver.get_last_endpoint().unwrap().unwrap();
    sender.connect(&ep).unwrap();

    (sender, receiver)
}

test!(test_exchanging_messages, {
    let (sender, receiver) = create_socketpair();
    sender.send("foo", 0).unwrap();
    let msg = receiver.recv_msg(0).unwrap();
    assert_eq!(&msg[..], b"foo");
    assert_eq!(msg.as_str(), Some("foo"));
    assert_eq!(format!("{:?}", msg), "[102, 111, 111]");

    receiver.send("bar", 0).unwrap();
    let msg = sender.recv_msg(0).unwrap();
    assert_eq!(&msg[..], b"bar");
});

test!(test_exchanging_bytes, {
    let (sender, receiver) = create_socketpair();
    sender.send("bar", 0).unwrap();
    assert_eq!(receiver.recv_bytes(0).unwrap(), b"bar");

    receiver.send("a quite long string", 0).unwrap();
    let mut buf = [0_u8; 10];
    sender.recv_into(&mut buf, 0).unwrap(); // this should truncate the message
    assert_eq!(&buf[..], b"a quite lo");
});

test!(test_exchanging_strings, {
    let (sender, receiver) = create_socketpair();
    sender.send("bäz", 0).unwrap();
    assert_eq!(receiver.recv_string(0).unwrap().unwrap(), "bäz");

    // non-UTF8 strings -> get an Err with bytes when receiving
    receiver.send(b"\xff\xb7".as_ref(), 0).unwrap();
    let result = sender.recv_string(0).unwrap();
    assert_eq!(result, Err(vec![0xff, 0xb7]));
});

test!(test_exchanging_multipart, {
    let (sender, receiver) = create_socketpair();

    // convenience API
    sender.send_multipart(["foo", "bar"], 0).unwrap();
    assert_eq!(receiver.recv_multipart(0).unwrap(), vec![b"foo", b"bar"]);

    // manually
    receiver.send("foo", SNDMORE).unwrap();
    receiver.send("bar", 0).unwrap();
    let msg1 = sender.recv_msg(0).unwrap();
    assert!(msg1.get_more());
    assert!(sender.get_rcvmore().unwrap());
    assert_eq!(&msg1[..], b"foo");
    let msg2 = sender.recv_msg(0).unwrap();
    assert!(!msg2.get_more());
    assert!(!sender.get_rcvmore().unwrap());
    assert_eq!(&msg2[..], b"bar");
});

test!(test_polling, {
    let (sender, receiver) = create_socketpair();

    // no message yet
    assert_eq!(receiver.poll(POLLIN, 1000).unwrap(), 0);

    // send message
    sender.send("Hello!", 0).unwrap();
    let mut poll_items = vec![receiver.as_poll_item(POLLIN)];
    assert_eq!(poll(&mut poll_items, 1000).unwrap(), 1);
    assert_eq!(poll_items[0].get_revents(), POLLIN);
    assert!(poll_items[0].is_readable());
    assert!(!poll_items[0].is_writable());
    assert!(!poll_items[0].is_error());
    assert!(poll_items[0].has_socket(&receiver));
    assert!(!poll_items[0].has_fd(0));
});

test!(test_raw_roundtrip, {
    let ctx = Context::new();
    let mut sock = ctx.socket(SocketType::REQ).unwrap();

    let ptr = sock.as_mut_ptr(); // doesn't consume the socket
                                 // NOTE: the socket will give up its context referecnce, but because we
                                 // still hold a reference in `ctx`, we won't get a deadlock.
    let raw = sock.into_raw(); // consumes the socket
    assert_eq!(ptr, raw);
    let _ = unsafe { Socket::from_raw(raw) };
});

// The `conflate` option limits the buffer size to one; let's see if we can get
// messages (unreliably) across the connection.
test!(test_conflating_receiver, {
    use std::sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    };

    let ctx = zmq::Context::new();
    let receiver = ctx.socket(zmq::PULL).unwrap();
    receiver.bind("tcp://127.0.0.1:*").unwrap();
    let receiver_endpoint = receiver.get_last_endpoint().unwrap().unwrap();

    let stop = Arc::new(AtomicBool::new(false));
    let sender_thread = {
        let stop = Arc::clone(&stop);
        std::thread::spawn(move || {
            let sender = ctx.socket(zmq::PUSH).unwrap();
            sender.connect(&receiver_endpoint).unwrap();
            while !stop.load(Ordering::SeqCst) {
                sender.send("bar", 0).expect("send failed");
            }
        })
    };

    receiver
        .set_conflate(true)
        .expect("could not set conflate option");
    for _ in 0..100 {
        let msg = receiver.recv_bytes(0).unwrap();
        assert_eq!(&msg[..], b"bar");
    }
    stop.store(true, Ordering::SeqCst);
    sender_thread.join().expect("could not join sender thread");
});

test!(test_version, {
    let (major, _, _) = version();
    assert!(major == 3 || major == 4);
});

test!(test_zmq_error, {
    let ctx = Context::new();
    let sock = ctx.socket(SocketType::REP).unwrap();

    // cannot send from REP unless we received a message first
    let err = sock.send("...", 0).unwrap_err();
    assert_eq!(err, Error::EFSM);

    // ZMQ error strings might not be guaranteed, so we'll not check
    // against specific messages, but still check that formatting does
    // not segfault, for example, and gives the same strings.
    let desc = err.message();
    let display = format!("{}", err);
    let debug = format!("{:?}", err);
    assert_eq!(desc, display);
    assert_eq!(desc, debug);
});

test!(test_into_io_error, {
    let e: io::Error = Error::ENOENT.into();
    assert!(e.kind() == io::ErrorKind::NotFound);
});

test!(test_get_socket_type, {
    let ctx = Context::new();

    let mut socket_types = vec![
        SocketType::PAIR,
        SocketType::PUB,
        SocketType::SUB,
        SocketType::REQ,
        SocketType::REP,
        SocketType::DEALER,
        SocketType::ROUTER,
        SocketType::PULL,
        SocketType::PUSH,
        SocketType::XPUB,
        SocketType::XSUB,
        SocketType::STREAM,
    ];
    for sock_type in socket_types.drain(..) {
        let sock = ctx.socket(sock_type).unwrap();
        assert_eq!(sock.get_socket_type().unwrap(), sock_type);
    }
});

test!(test_create_stream_socket, {
    let ctx = Context::new();
    let sock = ctx.socket(STREAM).unwrap();
    assert!(sock.bind("tcp://127.0.0.1:*").is_ok());
    let ep = sock.get_last_endpoint().unwrap().unwrap();
    let tcp = "tcp://";
    assert!(ep.starts_with(tcp));
    assert!(TcpStream::connect(&ep[tcp.len()..]).is_ok());
});

test!(test_getset_maxmsgsize, {
    let ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_maxmsgsize(512_000).unwrap();
    assert_eq!(sock.get_maxmsgsize().unwrap(), 512_000);
});

test!(test_getset_sndhwm, {
    let ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_sndhwm(500).unwrap();
    assert_eq!(sock.get_sndhwm().unwrap(), 500);
});

test!(test_getset_rcvhwm, {
    let ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_rcvhwm(500).unwrap();
    assert_eq!(sock.get_rcvhwm().unwrap(), 500);
});

test!(test_getset_affinity, {
    let ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_affinity(1024).unwrap();
    assert_eq!(sock.get_affinity().unwrap(), 1024);
});

test!(test_getset_identity, {
    let ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_identity(b"moo").unwrap();
    assert_eq!(sock.get_identity().unwrap(), b"moo");
});

test!(test_subscription, {
    let ctx = Context::new();
    let sock = ctx.socket(SUB).unwrap();
    assert!(sock.set_subscribe(b"/channel").is_ok());
    assert!(sock.set_unsubscribe(b"/channel").is_ok());
});

test!(test_set_req_relaxed, {
    let ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    assert!(sock.set_req_relaxed(true).is_ok());
    assert!(sock.set_req_relaxed(false).is_ok());
});

test!(test_set_req_correlate, {
    let ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    assert!(sock.set_req_correlate(true).is_ok());
    assert!(sock.set_req_correlate(false).is_ok());
});

test!(test_getset_rate, {
    let ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_rate(200).unwrap();
    assert_eq!(sock.get_rate().unwrap(), 200);
});

test!(test_getset_recovery_ivl, {
    let ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_recovery_ivl(100).unwrap();
    assert_eq!(sock.get_recovery_ivl().unwrap(), 100);
});

test!(test_getset_sndbuf, {
    let ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_sndbuf(100).unwrap();
    assert_eq!(sock.get_sndbuf().unwrap(), 100);
});

test!(test_getset_rcvbuf, {
    let ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_rcvbuf(100).unwrap();
    assert_eq!(sock.get_rcvbuf().unwrap(), 100);
});

test!(test_getset_tos, {
    let ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_tos(100).unwrap();
    assert_eq!(sock.get_tos().unwrap(), 100);
});

test!(test_getset_linger, {
    let ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_linger(100).unwrap();
    assert_eq!(sock.get_linger().unwrap(), 100);
});

test!(test_getset_reconnect_ivl, {
    let ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_reconnect_ivl(100).unwrap();
    assert_eq!(sock.get_reconnect_ivl().unwrap(), 100);
});

test!(test_getset_reconnect_ivl_max, {
    let ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_reconnect_ivl_max(100).unwrap();
    assert_eq!(sock.get_reconnect_ivl_max().unwrap(), 100);
});

test!(test_getset_backlog, {
    let ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_backlog(50).unwrap();
    assert_eq!(sock.get_backlog().unwrap(), 50);
});

test!(test_getset_multicast_hops, {
    let ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_multicast_hops(20).unwrap();
    assert_eq!(sock.get_multicast_hops().unwrap(), 20);
});

test!(test_getset_rcvtimeo, {
    let ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_rcvtimeo(5000).unwrap();
    assert_eq!(sock.get_rcvtimeo().unwrap(), 5000);
});

test!(test_getset_sndtimeo, {
    let ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_sndtimeo(5000).unwrap();
    assert_eq!(sock.get_sndtimeo().unwrap(), 5000);
});

test!(test_getset_ipv6, {
    let ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();

    sock.set_ipv6(true).unwrap();
    assert!(sock.is_ipv6().unwrap());

    sock.set_ipv6(false).unwrap();
    assert!(!sock.is_ipv6().unwrap());
});

test!(test_getset_socks_proxy, {
    let ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();

    sock.set_socks_proxy(Some("my_socks_server.com:10080"))
        .unwrap();
    assert_eq!(
        sock.get_socks_proxy().unwrap().unwrap(),
        "my_socks_server.com:10080"
    );

    sock.set_socks_proxy(None).unwrap();
    assert_eq!(sock.get_socks_proxy().unwrap().unwrap(), "");
});

test!(test_getset_keepalive, {
    let ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();

    sock.set_tcp_keepalive(-1).unwrap();
    assert_eq!(sock.get_tcp_keepalive().unwrap(), -1);

    sock.set_tcp_keepalive(0).unwrap();
    assert_eq!(sock.get_tcp_keepalive().unwrap(), 0);

    sock.set_tcp_keepalive(1).unwrap();
    assert_eq!(sock.get_tcp_keepalive().unwrap(), 1);
});

test!(test_getset_keepalive_cnt, {
    let ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();

    sock.set_tcp_keepalive_cnt(-1).unwrap();
    assert_eq!(sock.get_tcp_keepalive_cnt().unwrap(), -1);

    sock.set_tcp_keepalive_cnt(500).unwrap();
    assert_eq!(sock.get_tcp_keepalive_cnt().unwrap(), 500);
});

test!(test_getset_keepalive_idle, {
    let ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();

    sock.set_tcp_keepalive_idle(-1).unwrap();
    assert_eq!(sock.get_tcp_keepalive_idle().unwrap(), -1);

    sock.set_tcp_keepalive_idle(500).unwrap();
    assert_eq!(sock.get_tcp_keepalive_idle().unwrap(), 500);
});

test!(test_getset_tcp_keepalive_intvl, {
    let ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();

    sock.set_tcp_keepalive_intvl(-1).unwrap();
    assert_eq!(sock.get_tcp_keepalive_intvl().unwrap(), -1);

    sock.set_tcp_keepalive_intvl(500).unwrap();
    assert_eq!(sock.get_tcp_keepalive_intvl().unwrap(), 500);
});

test!(test_getset_immediate, {
    let ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();

    sock.set_immediate(true).unwrap();
    assert!(sock.is_immediate().unwrap());

    sock.set_immediate(false).unwrap();
    assert!(!sock.is_immediate().unwrap());
});

test!(test_getset_plain_server, {
    let ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();

    sock.set_plain_server(true).unwrap();
    assert!(sock.is_plain_server().unwrap());

    sock.set_plain_server(false).unwrap();
    assert!(!sock.is_plain_server().unwrap());
});

test!(test_getset_plain_username, {
    let ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();

    sock.set_plain_username(Some("billybob")).unwrap();
    assert_eq!(sock.get_plain_username().unwrap().unwrap(), "billybob");
    assert_eq!(sock.get_mechanism().unwrap(), Mechanism::ZMQ_PLAIN);

    sock.set_plain_username(None).unwrap();
    assert!(sock.get_mechanism().unwrap() == Mechanism::ZMQ_NULL);
});

test!(test_getset_plain_password, {
    let ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();

    sock.set_plain_password(Some("m00c0w")).unwrap();
    assert_eq!(sock.get_plain_password().unwrap().unwrap(), "m00c0w");
    assert_eq!(sock.get_mechanism().unwrap(), Mechanism::ZMQ_PLAIN);

    sock.set_plain_password(None).unwrap();
    assert!(sock.get_mechanism().unwrap() == Mechanism::ZMQ_NULL);
});

test!(test_zmq_set_xpub_verbose, {
    let ctx = Context::new();
    let xpub = ctx.socket(XPUB).unwrap();
    let sub = ctx.socket(SUB).unwrap();

    xpub.bind("inproc://set_xpub_verbose").unwrap();
    xpub.set_xpub_verbose(true).unwrap();

    sub.connect("inproc://set_xpub_verbose").unwrap();
    for _ in 0..2 {
        sub.set_subscribe(b"topic").unwrap();

        let event = xpub.recv_msg(0).unwrap();
        assert_eq!(event[0], 1);
        assert_eq!(&event[1..], b"topic");
    }
});

test!(test_zmq_xpub_welcome_msg, {
    let ctx = Context::new();
    let xpub = ctx.socket(XPUB).unwrap();

    xpub.bind("inproc://xpub_welcome_msg").unwrap();
    xpub.set_xpub_welcome_msg(Some("welcome")).unwrap();

    let sub = ctx.socket(SUB).unwrap();
    sub.set_subscribe(b"").unwrap();
    sub.connect("inproc://xpub_welcome_msg").unwrap();

    let from_pub = xpub.recv_bytes(0).unwrap();
    assert_eq!(from_pub, b"\x01");

    let from_xsub = sub.recv_bytes(0).unwrap();
    assert_eq!(from_xsub, b"welcome");
});

test!(test_getset_zap_domain, {
    let ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_zap_domain("test_domain").unwrap();
    assert_eq!(sock.get_zap_domain().unwrap().unwrap(), "test_domain");
});

test!(test_get_fd, {
    let ctx = Context::new();
    let sock_a = ctx.socket(REQ).unwrap();
    let sock_b = ctx.socket(REQ).unwrap();

    let mut fds_a: Vec<_> = (0..10).map(|_| sock_a.get_fd()).collect();
    fds_a.dedup();
    assert_eq!(fds_a.len(), 1);

    let mut fds_b: Vec<_> = (0..10).map(|_| sock_b.get_fd()).collect();
    fds_b.dedup();
    assert_eq!(fds_b.len(), 1);

    assert_ne!(fds_a[0], fds_b[0]);
});

test!(test_ctx_nohang, {
    // Test that holding on to a socket keeps the context it was
    // created from from being destroyed. Destroying the context while
    // a socket is still open would block, thus hanging this test in
    // the failing case.
    let sock = {
        let ctx = Context::new();
        ctx.socket(REQ).unwrap()
    };
    assert_eq!(sock.get_socket_type(), Ok(REQ));
});

test!(test_getset_conflate, {
    let ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_conflate(true).unwrap();
    assert!(sock.is_conflate().unwrap());
});

test!(test_disconnect, {
    // Make a connected socket pair
    let (sender, receiver) = create_socketpair();
    // Now disconnect them
    let ep = receiver.get_last_endpoint().unwrap().unwrap();
    sender.disconnect(&ep).unwrap();
    // And check that the message can no longer be sent
    assert_eq!(Error::EAGAIN, sender.send("foo", DONTWAIT).unwrap_err());
});

test!(test_disconnect_err, {
    let (sender, _) = create_socketpair();
    // Check that disconnect propagates errors. The endpoint is not connected.
    assert_eq!(
        Error::ENOENT,
        sender.disconnect("tcp://192.0.2.1:2233").unwrap_err()
    );
});

test!(test_getset_handshake_ivl, {
    let ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_handshake_ivl(50000).unwrap();
    assert_eq!(sock.get_handshake_ivl().unwrap(), 50000);
});

test!(test_getset_connect_timeout, {
    if version_ge_4_2() {
        let ctx = Context::new();
        let sock = ctx.socket(REQ).unwrap();
        sock.set_connect_timeout(5000).unwrap();
        assert_eq!(sock.get_connect_timeout().unwrap(), 5000);
    }
});

#[cfg(feature = "compiletest_rs")]
mod compile {
    extern crate compiletest_rs as compiletest;

    use std::path::PathBuf;

    fn run_mode(mode: &'static str) {
        let mut config = compiletest::Config::default();
        let cfg_mode = mode.parse().expect("Invalid mode");

        config.mode = cfg_mode;
        config.src_base = PathBuf::from(format!("tests/{}", mode));
        config.target_rustcflags = Some("-L target/debug -L target/debug/deps".to_string());

        compiletest::run_tests(&config);
    }

    #[test]
    fn expected_failures() {
        run_mode("compile-fail");
    }
}
