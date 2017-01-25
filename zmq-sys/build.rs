extern crate metadeps;

use std::env;
use std::path::Path;

fn main() {
    let lib_path = env::var("LIBZMQ_LIB_DIR").ok().or_else(|| {
        env::var("LIBZMQ_PREFIX").ok()
            .map(|prefix| Path::new(&prefix).join("lib"))
            .and_then(|path| path.to_str().map(|p| p.to_owned()))
    });

    let include = env::var("LIBZMQ_INCLUDE_DIR").ok().or_else(|| {
        env::var("LIBZMQ_PREFIX").ok()
            .map(|prefix| Path::new(&prefix).join("include"))
            .and_then(|path| path.to_str().map(|p| p.to_owned()))
    });

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
            if let Err(e) = metadeps::probe() {
                panic!("Unable to locate libzmq:\n{}", e);
            }
        }
    }
}
