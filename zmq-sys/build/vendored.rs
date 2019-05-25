use std::env;

pub fn configure() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=PROFILE");

    let wants_debug = env::var("PROFILE").unwrap() == "debug";

    let artifacts = zeromq_src::Build::new().link_static(true).build_debug(wants_debug).build();
    artifacts.print_cargo_metadata();
}
