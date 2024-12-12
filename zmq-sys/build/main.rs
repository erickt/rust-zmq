use std::env;

pub fn configure() {
    println!("cargo:rerun-if-changed=build/main.rs");
    println!("cargo:rerun-if-env-changed=PROFILE");

    // Note that by default `libzmq` builds without `libsodium` by instead
    // relying on `tweetnacl`. However since this `tweetnacl` [has never been
    // audited nor is ready for production](https://github.com/zeromq/libzmq/issues/3006),
    // we link against `libsodium` to enable `ZMQ_CURVE`.
    let maybe_libsodium = if cfg!(feature = "libsodium") {
        let lib_dir = env::var("DEP_SODIUM_LIB")
            .expect("build metadata `DEP_SODIUM_LIB` required");
        let include_dir = env::var("DEP_SODIUM_INCLUDE")
            .expect("build metadata `DEP_SODIUM_INCLUDE` required");

        Some(zeromq_src::LibLocation::new(lib_dir, include_dir))
    } else {
        None
    };

    zeromq_src::Build::new()
        .with_libsodium(maybe_libsodium)
        .build();
}

fn main() {
    configure()
}
