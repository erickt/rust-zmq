extern crate "pkg-config" as pkg_config;

fn main() {
    pkg_config::find_library("libzmq").unwrap();
    // pkg_config::find_library("libsodium").unwrap();
    println!("cargo:rustc-flags=-l stdc++");
}
