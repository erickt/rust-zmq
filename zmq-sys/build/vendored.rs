use std::env;

pub fn configure() {
    let wants_static =
        cfg!(feature = "static") || env::var("ZMQ_SYS_STATIC").unwrap_or_default() == "1";

    // Whether the current profile is in debug.
    let wants_debug = cfg!(debug_assertions);

    println!("cargo:rerun-if-changed=build.rs");
    let artifacts = zeromq_src::Build::new().link_static(wants_static).build_debug(wants_debug).build();
    artifacts.print_cargo_metadata();
}
