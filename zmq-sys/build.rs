extern crate cmake;
extern crate glob;
extern crate metadeps;

use cmake::Config;
use glob::glob;
use std::env;
use std::path::Path;
use std::process::Command;

fn build_static_libzmq() {
    let target = env::var("TARGET").unwrap();

    if !Path::new("libzmq/.git").exists() {
        let _ = Command::new("git")
            .args(&["submodule", "update", "--init"])
            .status();
    }

    let mut cmake = Config::new("libzmq");

    if target.contains("msvc") {
        // We need to explicitly disable `/GL` flag, otherwise
        // we get linkage error.
        cmake.cxxflag("/GL-");
        // Fix warning C4530: "C++ exception handler used, but unwind
        // semantics are not enabled. Specify /EHsc"
        cmake.cxxflag("/EHsc");
    }

    let dst = cmake
        // When compiled on 64bit system then default libdir is `lib64`, with
        // this we ensure that libdir will be `lib` on all systems.
        .define("CMAKE_INSTALL_LIBDIR", "lib")
        .define("ENABLE_DRAFTS", "OFF")
        .define("BUILD_SHARED", "OFF")
        .define("BUILD_STATIC", "ON")
        .build();

    if target.contains("msvc") {
        // Everything expects to link to zmq.lib, but the windows
        // build outputs a bunch of libs depending on runtime,
        // so we need to copy the one we want to the expected name.
        //
        // We use a glob pattern here so we don't have to worry about
        // the actual Visual Studio or zmq versions.
        let file_pattern = match env::var("PROFILE").unwrap().as_str() {
            "debug" => "libzmq-v*-mt-sgd-*.lib",
            "release" | "bench" => "libzmq-v*-mt-s-*.lib",
            unknown => {
                // cmake crate defaults to release if profile is unknown
                eprintln!(
                    "Warning: unknown Rust profile={}; assuming release",
                    unknown
                );
                "libzmq-v*-mt-s-*.lib"
            }
        };

        let pattern = format!("{}", dst.join("lib").join(file_pattern).display());

        let found_path = glob(&pattern)
            .expect("Failed to read file glob pattern.")
            .next()
            .expect(
                "No appropriate file created by libzmq build. Build script is likely out of date.",
            )
            .unwrap();

        let expected_path = dst.join("lib").join("zmq.lib");
        std::fs::copy(&found_path, &expected_path).expect(&format!(
            "Unable to copy '{}' to '{}'",
            found_path.display(),
            expected_path.display()
        ));
    }

    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=zmq");

    if target.contains("msvc") {
        println!("cargo:rustc-link-lib=dylib=iphlpapi");
    } else {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    }
}

fn prefix_dir(env_name: &str, dir: &str) -> Option<String> {
    env::var(env_name).ok().or_else(|| {
        env::var("LIBZMQ_PREFIX")
            .ok()
            .map(|prefix| Path::new(&prefix).join(dir))
            .and_then(|path| path.to_str().map(|p| p.to_owned()))
    })
}

fn main() {
    if cfg!(feature = "static-libzmq") {
        return build_static_libzmq();
    }

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
