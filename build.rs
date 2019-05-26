use std::env;

fn main() {
    use std::ffi::CString;

    println!(
        "cargo:rustc-env=BUILD_PROFILE={}",
        env::var("PROFILE").unwrap()
    );

    for has in ["ipc", "pgm", "tipc", "norm", "curve", "gssapi"].into_iter() {
        if unsafe { zmq_sys::zmq_has(CString::new(has.as_bytes()).unwrap().as_ptr()) } == 1 {
            println!("cargo:rustc-cfg=ZMQ_HAS_{}=\"1\"", has.to_uppercase());
        }
    }
}
