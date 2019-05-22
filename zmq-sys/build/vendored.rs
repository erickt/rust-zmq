pub fn configure() {
    // Whether the current profile is in debug.
    let wants_debug = cfg!(debug_assertions);

    println!("cargo:rerun-if-changed=build.rs");
    let artifacts = zeromq_src::Build::new().link_static(true).build_debug(wants_debug).build();
    artifacts.print_cargo_metadata();
}
