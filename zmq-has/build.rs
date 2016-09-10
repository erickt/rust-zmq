extern crate pkg_config;
extern crate gcc;

use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    let host = env::var("HOST").unwrap();
    if target != host {
        println!("cargo:warning=You are cross compiling rust-zmq.\
         Can't compile zmq_has, you need to specify features explicitly!. ");
        println!("cargo:rustc-cfg=cross");
    }
    if let Some(prefix) = env::var("LIBZMQ_PREFIX").ok() {
        println!("cargo:rustc-link-search=native={}/lib", prefix);
        println!("cargo:include={}/include", prefix);
        println!("cargo:warning=You are specifying zmq prefix.\
         rust-zmq will only compile with libzmq versions 4.x. ");
    } else {
        match pkg_config::find_library("libzmq") {
            Ok(pkg) => {
                println!("{:?}", pkg);
                if &pkg.version[..3] != "4.1" && &pkg.version[..3] != "4.2" {
                    println!("cargo:warning=You are compiling rust-zmq \
                     with older version of libzmq (version {}).\
                     Can't compile zmq_has, you need to specify features explicitly!. ", &pkg.version[..3]);
                    println!("cargo:rustc-cfg=olderzmq");
                }
            },
            Err(e) => panic!("Unable to locate libzmq, err={:?}", e),
        }
    }
}

