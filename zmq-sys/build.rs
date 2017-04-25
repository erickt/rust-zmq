extern crate metadeps;
extern crate vcpkg;

use std::env;
use std::fs;
use std::path::Path;

fn prefix_dir(env_name: &str, dir: &str) -> Option<String> {
    env::var(env_name).ok().or_else(|| {
        env::var("LIBZMQ_PREFIX").ok()
            .map(|prefix| Path::new(&prefix).join(dir))
            .and_then(|path| path.to_str().map(|p| p.to_owned()))
    })
}

fn main() {
    let lib_path = prefix_dir("LIBZMQ_LIB_DIR", "lib");
    let include = prefix_dir("LIBZMQ_INCLUDE_DIR", "include");

    match (lib_path, include) {
        (Some(lib_path), Some(include)) => {
            println!("cargo:rustc-link-search=native={}", lib_path);
            println!("cargo:include={}", include);
        }
        (Some(_), None) => {
            panic!("Unable to locate libzmq include directory.")
        }
        (None, Some(_)) => {
            panic!("Unable to locate libzmq library directory.")
        }
        (None, None) => {
            match vcpkg::Config::new()
                .lib_name("libzmq")
                .probe("zeromq") {
                Err(e) => println!("vcpkg did not find zeromq: {}", e),
                Ok(lib_config) => {
                    println!("cargo:rustc-link-lib=iphlpapi");

                    // generated binding expects to link to a lib called "zmq.lib" but the
                    // library that was found is "libzmq.lib", so make a copy as zmq.lib into
                    // into OUT_DIR. It is not necessary to create a copy of the DLL because
                    // the import library version of the .lib will try to load it as libzmq.dll
                    if lib_config.found_libs.len() != 1 {
                        panic!(format!("found {} libs, expecting 1", lib_config.found_libs.len()));
                    }

                    fs::copy(Path::new(&lib_config.found_libs[0]),
                             Path::new(&env::var_os("OUT_DIR").unwrap()).join("zmq.lib"))
                        .expect("Could not copy libzmq.lib to OUT_DIR/zmq.lib");

                    if lib_config.is_static {
                        println!("cargo:rustc-link-lib=static=zmq");
                    }

                    println!("cargo:rustc-link-search=native={}",
                             env::var("OUT_DIR").unwrap());

                    // emit a rustc-link-search line without native= so build.rs zmq crate
                    // can find libzmq.dll at build time. workaround for cargo bug
                    // https://github.com/rust-lang/cargo/issues/3957
                    for path in lib_config.dll_paths {
                        println!("cargo:rustc-link-search={}", path.to_str().unwrap());
                    }

                    return;
                }
            }

            if let Err(e) = metadeps::probe() {
                panic!("Unable to locate libzmq:\n{}", e);
            }
        }
    }
}
