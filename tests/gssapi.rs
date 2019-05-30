#[macro_use]
mod common;

use zmq::Context;

test_capability!(test_getset_gssapi_server, "gssapi", {
    let ctx = Context::new();
    let sock = ctx.socket(zmq::REQ).unwrap();
    sock.set_gssapi_server(true).unwrap();
    assert_eq!(sock.is_gssapi_server().unwrap(), true);
});

test_capability!(test_getset_gssapi_principal, "gssapi", {
    let ctx = Context::new();
    let sock = ctx.socket(zmq::REQ).unwrap();
    sock.set_gssapi_principal("principal").unwrap();
    assert_eq!(sock.get_gssapi_principal().unwrap().unwrap(), "principal");
});

test_capability!(test_getset_gssapi_service_principal, "gssapi", {
    let ctx = Context::new();
    let sock = ctx.socket(zmq::REQ).unwrap();
    sock.set_gssapi_service_principal("principal").unwrap();
    assert_eq!(
        sock.get_gssapi_service_principal().unwrap().unwrap(),
        "principal"
    );
});

test_capability!(test_getset_gssapi_plaintext, "gssapi", {
    let ctx = Context::new();
    let sock = ctx.socket(zmq::REQ).unwrap();
    sock.set_gssapi_plaintext(true).unwrap();
    assert_eq!(sock.is_gssapi_plaintext().unwrap(), true);
});
