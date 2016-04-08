extern crate pkg_config;

use std::env;

fn main() {
    if let Some(prefix) = env::var("LIBZMQ_PREFIX").ok() {
        println!("cargo:rustc-link-search=native={}/lib", prefix);
        println!("cargo:include={}/include", prefix);
    } else {
        match pkg_config::find_library("libzmq") {
            Ok(pkg) => println!("{:?}", pkg),
            Err(e) => panic!("Unable to locate libzmq, err={:?}", e),
        }
    }
}
