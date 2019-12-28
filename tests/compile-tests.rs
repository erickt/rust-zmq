use std::env;
use std::path::PathBuf;

fn run_mode(mode: &'static str) {
    let mut config = compiletest_rs::Config::default();

    let cfg_mode = mode.parse().expect("Invalid mode");

    config.target_rustcflags = Some(format!(
        "-L target/{profile} -L target/{profile}/deps",
        profile = env!("BUILD_PROFILE")
    ));

    if let Ok(name) = env::var("TESTNAME") {
        config.filter = Some(name)
    }
    config.mode = cfg_mode;
    config.src_base = PathBuf::from(format!("tests/{}", mode));

    compiletest_rs::run_tests(&config);
}

#[test]
fn compile_test() {
    run_mode("compile-fail");
}
