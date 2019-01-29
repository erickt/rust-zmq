#[cfg(windows)]
extern crate cmake;
#[cfg(windows)]
use cmake::Config;

#[cfg(windows)]
extern crate glob;
#[cfg(windows)]
use glob::glob;

#[cfg(not(windows))]
extern crate metadeps;
#[cfg(not(windows))]
use std::env;

use std::path::Path;
use std::process::Command;

#[cfg(windows)]
fn main() {
    if !Path::new("libzmq/.git").exists() {
        let _ = Command::new("git")
            .args(&["submodule", "update", "--init"])
            .status();
    }

    let dst = Config::new("libzmq").build();

    // Everything expects to link to zmq.lib, but the windows
    // build outputs a bunch of libs depending on runtime,
    // so we need to copy the one we want to the expected name.
    //
    // We use a glob pattern here so we don't have to worry about
    // the actual Visual Studio or zmq versions.
    let pattern = format!("{}\\libzmq-v???-mt-sgd-*.lib", dst.join("lib").display());
    let found_path = glob(&pattern)
        .expect("Failed to read file glob pattern.")
        .next()
        .expect("No appropriate file created by libzmq build. Build script is likely out of date.")
        .unwrap();

    let expected_path = dst.join("lib\\zmq.lib");
    std::fs::copy(&found_path, &expected_path).expect(&format!(
        "Unable to copy '{}' to '{}'",
        found_path.display(),
        expected_path.display()
    ));

    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=zmq");
    println!("cargo:rustc-link-lib=dylib=iphlpapi");
}

#[cfg(not(windows))]
fn prefix_dir(env_name: &str, dir: &str) -> Option<String> {
    env::var(env_name).ok().or_else(|| {
        env::var("LIBZMQ_PREFIX")
            .ok()
            .map(|prefix| Path::new(&prefix).join(dir))
            .and_then(|path| path.to_str().map(|p| p.to_owned()))
    })
}

#[cfg(not(windows))]
fn main() {
    let lib_path = prefix_dir("LIBZMQ_LIB_DIR", "lib");
    let include = prefix_dir("LIBZMQ_INCLUDE_DIR", "include");

    match (lib_path, include) {
        (Some(lib_path), Some(include)) => {
            println!("cargo:rustc-link-search=native={}", lib_path);
            println!("cargo:include={}", include);
        }
        (Some(_), None) => panic!("Unable to locate libzmq include directory."),
        (None, Some(_)) => panic!("Unable to locate libzmq library directory."),
        (None, None) => {
            if let Err(e) = metadeps::probe() {
                panic!("Unable to locate libzmq:\n{}", e);
            }
        }
    }
}
