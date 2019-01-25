#![cfg(test)]
extern crate compiletest_rs as compiletest;

#[test]
fn compile_fail() {
    let mut config = compiletest::Config {
        mode: compiletest::common::Mode::CompileFail,
        src_base: std::path::PathBuf::from("tests/compile-fail"),
        target_rustcflags: Some("-L target/debug -L target/debug/deps".to_string()),
        ..Default::default()
    };

    config.link_deps();
    config.clean_rmeta();

    compiletest::run_tests(&config);
}

#[test]
fn run_pass() {
    let mut config = compiletest::Config {
        mode: compiletest::common::Mode::RunPass,
        src_base: std::path::PathBuf::from("tests/run-pass"),
        target_rustcflags: Some("-L target/debug -L target/debug/deps".to_string()),
        ..Default::default()
    };

    config.link_deps();
    config.clean_rmeta();

    compiletest::run_tests(&config);
}
