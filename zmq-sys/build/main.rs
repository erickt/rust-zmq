pub fn configure() {
    println!("cargo:rerun-if-changed=build/main.rs");
    println!("cargo:rerun-if-env-changed=PROFILE");

    // Note that by default `libzmq` builds without `libsodium` by instead
    // relying on `tweetnacl`. However since this `tweetnacl` [has never been
    // audited nor is ready for production](https://github.com/zeromq/libzmq/issues/3006),
    // we link against `libsodium` to enable `ZMQ_CURVE`.

    // get sodium lib and include paths from environment
    let sodium_paths = env::var("SODIUM_LIB")
        .and_then(|lib| env::var("SODIUM_INCLUDE").map(|inc| LibLocation::new(lib, inc)))
        .ok();
    zeromq_src::Build::new()
        .with_libsodium(None)
        .build();
}

fn main() {
    configure()
}
