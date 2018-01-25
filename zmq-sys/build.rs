extern crate metadeps;
extern crate cmake;

use std::path::Path;
use std::{env, fs, io};
// use std::env::VarError;

use cmake::Config;

fn prefix_dir(env_name: &str, dir: &str) -> Option<String> {
    env::var(env_name).ok().or_else(|| {
        env::var("LIBZMQ_PREFIX").ok()
            .map(|prefix| Path::new(&prefix).join(dir))
            .and_then(|path| path.to_str().map(|p| p.to_owned()))
    })
}

fn is_directory_empty<P: AsRef<Path>>(p: P) -> Result<bool, io::Error> {
    let mut entries = fs::read_dir(p)?;
    Ok(entries.next().is_none())
}

// fn get_version() -> (major: str, minor: str, patch: str) {
// }

fn prepare_zmq() {
    let modules = vec!["libzmq"];

    for module in modules {
        if is_directory_empty(module).unwrap_or(true) {
            panic!(
                "Can't find module {}. You need to run `git submodule \
                 update --init --recursive` first to build the project.",
                module
            );
        }
    }
}

fn build_zmq(library: &str) {
    prepare_zmq();

    let dst = {
        let mut config = Config::new("libzmq");
        if cfg!(target_os = "macos") {
            config.cxxflag("-stdlib=libc++");
        }
        config
            .build_target(library)
            .uses_cxx11()
            .define("CMAKE_DEBUG_POSTFIX", "")      // Disable library name postfix
            .define("CMAKE_RELEASE_POSTFIX", "")    // Disable library name postfix
            .build()
    };

    let toolchain_version = "v141";
    let library_version = "4_2_4";
    let mut configuration_postfix = "s";
    let build_dir = format!("{}/build", dst.display());
    if cfg!(target_os = "windows") {
        let profile = match &*env::var("PROFILE").unwrap_or("debug".to_owned()) {
            "bench" | "release" => {
                "Release"
            }
            _ => {
                configuration_postfix = "sgd";
                "Debug"
            }
        };
        println!("cargo:rustc-link-search=native={}/lib/{}", build_dir, profile);
    } else {
        println!("cargo:rustc-link-search=native={}", build_dir);
    }

    // println!("cargo:rustc-link-lib=static={}", library);

    // libzmq-v141-mt-sgd-4_2_4
    println!("cargo:rustc-link-lib=static=libzmq-{}-mt-{}-{}", toolchain_version, configuration_postfix, library_version);
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
            if let Err(_e) = metadeps::probe() {
                // Last option, build from source:
                build_zmq("libzmq-static");
                
                // panic!("Unable to locate libzmq:\n{}", e);
            }
        }
    }
}
