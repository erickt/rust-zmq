extern crate timebomb;
extern crate zmq;

#[macro_use]
mod common;

use std::io;
use std::net::TcpStream;
use zmq::*;

fn create_socketpair() -> (Socket, Socket) {
    let ctx = Context::default();

    let sender = ctx.socket(zmq::REQ).unwrap();
    let receiver = ctx.socket(zmq::REP).unwrap();

    // Don't block forever
    sender.set_sndtimeo(1000).unwrap();
    sender.set_rcvtimeo(1000).unwrap();
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
    let mut msg = Message::with_capacity(1);
    sender.recv(&mut msg, 0).unwrap();
    assert_eq!(&msg[..], b"bar");
});

test!(test_exchanging_bytes, {
    let (sender, receiver) = create_socketpair();
    sender.send("bar", 0).unwrap();
    assert_eq!(receiver.recv_bytes(0).unwrap(), b"bar");

    receiver.send("a quite long string", 0).unwrap();
    let mut buf = [0_u8; 10];
    sender.recv_into(&mut buf, 0).unwrap();  // this should truncate the message
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
    sender.send_multipart(&["foo", "bar"], 0).unwrap();
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
});

test!(test_raw_roundtrip, {
    let ctx = Context::new();
    let mut sock = ctx.socket(SocketType::REQ).unwrap();

    let ptr = sock.as_mut_ptr();  // doesn't consume the socket
    // NOTE: the socket will give up its context referecnce, but because we
    // still hold a reference in `ctx`, we won't get a deadlock.
    let raw = sock.into_raw();    // consumes the socket
    assert_eq!(ptr, raw);
    let _ = unsafe { Socket::from_raw(raw) };
});

test!(test_version, {
    let (major, _, _) = version();
    assert!(major == 3 || major == 4);
});

test!(test_zmq_error, {
    use std::error::Error as StdError;

    let ctx = Context::new();
    let sock = ctx.socket(SocketType::REP).unwrap();

    // cannot send from REP unless we received a message first
    let err = sock.send("...", 0).unwrap_err();
    assert_eq!(err, Error::EFSM);

    // ZMQ error strings might not be guaranteed, so we'll not check
    // against specific messages, but still check that formatting does
    // not segfault, for example, and gives the same strings.
    let desc = err.description();
    let display = format!("{}", err);
    let debug = format!("{:?}", err);
    assert_eq!(desc, display);
    assert_eq!(desc, debug);
});

test!(test_into_io_error, {
    let e: io::Error = Error::ENOENT.into();
    assert!(e.kind() == io::ErrorKind::NotFound);
});

#[cfg(ZMQ_HAS_CURVE = "1")]
test!(test_curve_keypair, {
    let keypair = CurveKeyPair::new().unwrap();
    assert!(keypair.public_key.len() == 32);
    assert!(keypair.secret_key.len() == 32);
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
    sock.set_maxmsgsize(512000).unwrap();
    assert_eq!(sock.get_maxmsgsize().unwrap(), 512000);
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

    sock.set_socks_proxy(Some("my_socks_server.com:10080")).unwrap();
    assert_eq!(sock.get_socks_proxy().unwrap().unwrap(), "my_socks_server.com:10080");

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

#[cfg(ZMQ_HAS_CURVE = "1")]
test!(test_getset_curve_server, {
    let ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_curve_server(true).unwrap();
    assert_eq!(sock.is_curve_server().unwrap(), true);
});

#[cfg(ZMQ_HAS_CURVE = "1")]
test!(test_getset_curve_publickey, {
    let ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    let key = z85_decode("FX5b8g5ZnOk7$Q}^)Y&?.v3&MIe+]OU7DTKynkUL").unwrap();
    sock.set_curve_publickey(&key).unwrap();
    assert_eq!(sock.get_curve_publickey().unwrap(), key);
});

#[cfg(ZMQ_HAS_CURVE = "1")]
test!(test_getset_curve_secretkey, {
    let ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    let key = z85_decode("s9N%S3*NKSU$6pUnpBI&K5HBd[]G$Y3yrK?mhdbS").unwrap();
    sock.set_curve_secretkey(&key).unwrap();
    assert_eq!(sock.get_curve_secretkey().unwrap(), key);
});

#[cfg(ZMQ_HAS_CURVE = "1")]
test!(test_getset_curve_serverkey, {
    let ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    let key = z85_decode("FX5b8g5ZnOk7$Q}^)Y&?.v3&MIe+]OU7DTKynkUL").unwrap();
    sock.set_curve_serverkey(&key).unwrap();
    assert_eq!(sock.get_curve_serverkey().unwrap(), key);
});


test!(test_getset_conflate, {
    let ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_conflate(true).unwrap();
    assert_eq!(sock.is_conflate().unwrap(), true);
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
    assert_eq!(Error::ENOENT, sender.disconnect("tcp://192.0.2.1:2233").unwrap_err());
});

#[cfg(ZMQ_HAS_GSSAPI = "1")]
test!(test_getset_gssapi_server, {
    let ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_gssapi_server(true).unwrap();
    assert_eq!(sock.is_gssapi_server().unwrap(), true);
});

#[cfg(ZMQ_HAS_GSSAPI = "1")]
test!(test_getset_gssapi_principal, {
    let ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_gssapi_principal("principal").unwrap();
    assert_eq!(sock.get_gssapi_principal().unwrap().unwrap(), "principal");
});

#[cfg(ZMQ_HAS_GSSAPI = "1")]
test!(test_getset_gssapi_service_principal, {
    let ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_gssapi_service_principal("principal").unwrap();
    assert_eq!(sock.get_gssapi_service_principal().unwrap().unwrap(), "principal");
});

#[cfg(ZMQ_HAS_GSSAPI = "1")]
test!(test_getset_gssapi_plaintext, {
    let ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_gssapi_plaintext(true).unwrap();
    assert_eq!(sock.is_gssapi_plaintext().unwrap(), true);
});

#[cfg(ZMQ_HAS_GSSAPI = "1")]
test!(test_getset_handshake_ivl, {
    let ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_handshake_ivl(50000).unwrap();
    assert_eq!(sock.get_handshake_ivl().unwrap(), 50000);
});

#[cfg(feature = "compiletest_rs")]
mod compile {
    extern crate compiletest_rs as compiletest;

    use std::path::PathBuf;

    fn run_mode(mode: &'static str) {
        let mut config = compiletest::default_config();
        let cfg_mode = mode.parse().ok().expect("Invalid mode");

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
