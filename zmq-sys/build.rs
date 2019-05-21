extern crate metadeps;

use std::env;
use std::path::Path;

fn verify_windows_build_environment() {
    let target = env::var("TARGET").ok().unwrap();
    println!("cargo:target={:?}", target);
    if target.contains("x86_64-pc-windows") {
        let dll_path = prefix_dir("LIBZMQ_BIN_DIR", "bin");
        match &dll_path {
            Some(dll_path) => {
                println!("cargo:dll_path={}", &dll_path);
            }
            None => {
                panic!("Windows environment error, unable to locate libzmq dll_path (LIBZMQ_BIN_DIR environment variable missing).\n")
            }
        }
        let user_path = env::var("PATH").ok().unwrap();
        if !user_path.contains(&dll_path.unwrap()) {
            println!("cargo:dll_path (LIBZMQ_BIN_DIR) NOT contained in user_path\n");
            panic!("Windows environment not setup correctly!");
        }
    }
}

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
            if let Err(e) = metadeps::probe() {
                panic!("Unable to locate libzmq include and library directory:\n{}", e);
            }
        }
    }

    verify_windows_build_environment();
}
