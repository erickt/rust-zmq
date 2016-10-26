extern crate zmq;
use zmq::*;

#[cfg(ZMQ_HAS_CURVE = "1")]
#[test]
fn test_curve_keypair() {
    let keypair = CurveKeyPair::new().unwrap();
    assert!(keypair.public_key.len() == 40);
    assert!(keypair.secret_key.len() == 40);
}

#[test]
fn test_get_socket_type() {
    let mut ctx = Context::new();

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
        SocketType::XSUB
    ];
    for sock_type in socket_types.drain(..) {
        let sock = ctx.socket(sock_type).unwrap();
        assert_eq!(sock.get_socket_type().unwrap(), sock_type);
    }
}

#[test]
fn test_getset_maxmsgsize() {
    let mut ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_maxmsgsize(512000).unwrap();
    assert_eq!(sock.get_maxmsgsize().unwrap(), 512000);
}

#[test]
fn test_getset_sndhwm() {
    let mut ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_sndhwm(500).unwrap();
    assert_eq!(sock.get_sndhwm().unwrap(), 500);
}

#[test]
fn test_getset_rcvhwm() {
    let mut ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_rcvhwm(500).unwrap();
    assert_eq!(sock.get_rcvhwm().unwrap(), 500);
}

#[test]
fn test_getset_affinity() {
    let mut ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_affinity(1024).unwrap();
    assert_eq!(sock.get_affinity().unwrap(), 1024);
}

#[test]
fn test_getset_identity() {
    let mut ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_identity("moo".as_bytes()).unwrap();
    assert_eq!(sock.get_identity().unwrap().unwrap(), "moo");
}

#[test]
fn test_subscription() {
    let mut ctx = Context::new();
    let sock = ctx.socket(SUB).unwrap();
    assert!(sock.set_subscribe("/channel".as_bytes()).is_ok());
    assert!(sock.set_unsubscribe("/channel".as_bytes()).is_ok());
}

#[test]
fn test_getset_rate() {
    let mut ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_rate(200).unwrap();
    assert_eq!(sock.get_rate().unwrap(), 200);
}

#[test]
fn test_getset_recovery_ivl() {
    let mut ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_recovery_ivl(100).unwrap();
    assert_eq!(sock.get_recovery_ivl().unwrap(), 100);
}

#[test]
fn test_getset_sndbuf() {
    let mut ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_sndbuf(100).unwrap();
    assert_eq!(sock.get_sndbuf().unwrap(), 100);
}

#[test]
fn test_getset_rcvbuf() {
    let mut ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_rcvbuf(100).unwrap();
    assert_eq!(sock.get_rcvbuf().unwrap(), 100);
}

#[test]
fn test_getset_tos() {
    let mut ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_tos(100).unwrap();
    assert_eq!(sock.get_tos().unwrap(), 100);
}

#[test]
fn test_getset_linger() {
    let mut ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_linger(100).unwrap();
    assert_eq!(sock.get_linger().unwrap(), 100);
}

#[test]
fn test_getset_reconnect_ivl() {
    let mut ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_reconnect_ivl(100).unwrap();
    assert_eq!(sock.get_reconnect_ivl().unwrap(), 100);
}

#[test]
fn test_getset_reconnect_ivl_max() {
    let mut ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_reconnect_ivl_max(100).unwrap();
    assert_eq!(sock.get_reconnect_ivl_max().unwrap(), 100);
}

#[test]
fn test_getset_backlog() {
    let mut ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_backlog(50).unwrap();
    assert_eq!(sock.get_backlog().unwrap(), 50);
}

#[test]
fn test_getset_multicast_hops() {
    let mut ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_multicast_hops(20).unwrap();
    assert_eq!(sock.get_multicast_hops().unwrap(), 20);
}

#[test]
fn test_getset_rcvtimeo() {
    let mut ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_rcvtimeo(5000).unwrap();
    assert_eq!(sock.get_rcvtimeo().unwrap(), 5000);
}

#[test]
fn test_getset_sndtimeo() {
    let mut ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_sndtimeo(5000).unwrap();
    assert_eq!(sock.get_sndtimeo().unwrap(), 5000);
}

#[test]
fn test_getset_ipv6() {
    let mut ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();

    sock.set_ipv6(true).unwrap();
    assert!(sock.is_ipv6().unwrap());

    sock.set_ipv6(false).unwrap();
    assert!(sock.is_ipv6().unwrap() == false);
}

#[test]
fn test_getset_socks_proxy() {
    let mut ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();

    sock.set_socks_proxy(Some("my_socks_server.com:10080")).unwrap();
    assert_eq!(sock.get_socks_proxy().unwrap().unwrap(), "my_socks_server.com:10080");

    sock.set_socks_proxy(None).unwrap();
    assert_eq!(sock.get_socks_proxy().unwrap().unwrap(), "");
}

#[test]
fn test_getset_keepalive() {
    let mut ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();

    sock.set_tcp_keepalive(-1).unwrap();
    assert_eq!(sock.get_tcp_keepalive().unwrap(), -1);

    sock.set_tcp_keepalive(0).unwrap();
    assert_eq!(sock.get_tcp_keepalive().unwrap(), 0);

    sock.set_tcp_keepalive(1).unwrap();
    assert_eq!(sock.get_tcp_keepalive().unwrap(), 1);
}

#[test]
fn test_getset_keepalive_cnt() {
    let mut ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();

    sock.set_tcp_keepalive_cnt(-1).unwrap();
    assert_eq!(sock.get_tcp_keepalive_cnt().unwrap(), -1);

    sock.set_tcp_keepalive_cnt(500).unwrap();
    assert_eq!(sock.get_tcp_keepalive_cnt().unwrap(), 500);
}

#[test]
fn test_getset_keepalive_idle() {
    let mut ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();

    sock.set_tcp_keepalive_idle(-1).unwrap();
    assert_eq!(sock.get_tcp_keepalive_idle().unwrap(), -1);

    sock.set_tcp_keepalive_idle(500).unwrap();
    assert_eq!(sock.get_tcp_keepalive_idle().unwrap(), 500);
}

#[test]
fn test_getset_tcp_keepalive_intvl() {
    let mut ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();

    sock.set_tcp_keepalive_intvl(-1).unwrap();
    assert_eq!(sock.get_tcp_keepalive_intvl().unwrap(), -1);

    sock.set_tcp_keepalive_intvl(500).unwrap();
    assert_eq!(sock.get_tcp_keepalive_intvl().unwrap(), 500);
}

#[test]
fn test_getset_immediate() {
    let mut ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();

    sock.set_immediate(true).unwrap();
    assert!(sock.is_immediate().unwrap());

    sock.set_immediate(false).unwrap();
    assert!(sock.is_immediate().unwrap() == false);
}

#[test]
fn test_getset_plain_server() {
    let mut ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();

    sock.set_plain_server(true).unwrap();
    assert!(sock.is_plain_server().unwrap());

    sock.set_plain_server(false).unwrap();
    assert!(sock.is_plain_server().unwrap() == false);
}

#[test]
fn test_getset_plain_username() {
    let mut ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();

    sock.set_plain_username(Some("billybob")).unwrap();
    assert_eq!(sock.get_plain_username().unwrap().unwrap(), "billybob");
    assert_eq!(sock.get_mechanism().unwrap(), Mechanism::ZMQ_PLAIN);

    sock.set_plain_username(None).unwrap();
    assert!(sock.get_mechanism().unwrap() == Mechanism::ZMQ_NULL);
}

#[test]
fn test_getset_plain_password() {
    let mut ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();

    sock.set_plain_password(Some("m00c0w")).unwrap();
    assert_eq!(sock.get_plain_password().unwrap().unwrap(), "m00c0w");
    assert_eq!(sock.get_mechanism().unwrap(), Mechanism::ZMQ_PLAIN);

    sock.set_plain_password(None).unwrap();
    assert!(sock.get_mechanism().unwrap() == Mechanism::ZMQ_NULL);
}

#[test]
fn test_getset_zap_domain() {
    let mut ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_zap_domain("test_domain").unwrap();
    assert_eq!(sock.get_zap_domain().unwrap().unwrap(), "test_domain");
}

#[cfg(ZMQ_HAS_CURVE = "1")]
#[test]
fn test_getset_curve_server() {
    let mut ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_curve_server(true).unwrap();
    assert_eq!(sock.is_curve_server().unwrap(), true);
}

#[cfg(ZMQ_HAS_CURVE = "1")]
#[test]
fn test_getset_curve_publickey() {
    let mut ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_curve_publickey("FX5b8g5ZnOk7$Q}^)Y&?.v3&MIe+]OU7DTKynkUL").unwrap();
    assert_eq!(sock.get_curve_publickey().unwrap().unwrap(), "FX5b8g5ZnOk7$Q}^)Y&?.v3&MIe+]OU7DTKynkUL");
}

#[cfg(ZMQ_HAS_CURVE = "1")]
#[test]
fn test_getset_curve_secretkey() {
    let mut ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_curve_secretkey("s9N%S3*NKSU$6pUnpBI&K5HBd[]G$Y3yrK?mhdbS").unwrap();
    assert_eq!(sock.get_curve_secretkey().unwrap().unwrap(), "s9N%S3*NKSU$6pUnpBI&K5HBd[]G$Y3yrK?mhdbS");
}

#[cfg(ZMQ_HAS_CURVE = "1")]
#[test]
fn test_getset_curve_serverkey() {
    let mut ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_curve_serverkey("FX5b8g5ZnOk7$Q}^)Y&?.v3&MIe+]OU7DTKynkUL").unwrap();
    assert_eq!(sock.get_curve_serverkey().unwrap().unwrap(), "FX5b8g5ZnOk7$Q}^)Y&?.v3&MIe+]OU7DTKynkUL");
}

#[test]
fn test_getset_conflate() {
    let mut ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_conflate(true).unwrap();
    assert_eq!(sock.is_conflate().unwrap(), true);
}

#[cfg(ZMQ_HAS_GSSAPI = "1")]
#[test]
fn test_getset_gssapi_server() {
    let mut ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_gssapi_server(true).unwrap();
    assert_eq!(sock.is_gssapi_server().unwrap(), true);
}

#[cfg(ZMQ_HAS_GSSAPI = "1")]
#[test]
fn test_getset_gssapi_principal() {
    let mut ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_gssapi_principal("principal").unwrap();
    assert_eq!(sock.get_gssapi_principal().unwrap().unwrap(), "principal");
}

#[cfg(ZMQ_HAS_GSSAPI = "1")]
#[test]
fn test_getset_gssapi_service_principal() {
    let mut ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_gssapi_service_principal("principal").unwrap();
    assert_eq!(sock.get_gssapi_service_principal().unwrap().unwrap(), "principal");
}

#[cfg(ZMQ_HAS_GSSAPI = "1")]
#[test]
fn test_getset_gssapi_plaintext() {
    let mut ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_gssapi_plaintext(true).unwrap();
    assert_eq!(sock.is_gssapi_plaintext().unwrap(), true);
}

#[cfg(ZMQ_HAS_GSSAPI = "1")]
#[test]
fn test_getset_handshake_ivl() {
    let mut ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    sock.set_handshake_ivl(50000).unwrap();
    assert_eq!(sock.get_handshake_ivl().unwrap(), 50000);
}
