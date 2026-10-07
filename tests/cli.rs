//! Runs the binary on the original C++ project's sample input and checks the
//! output against the C++ output, ignoring timings and the fixed typo.

use std::io::Write;
use std::process::{Command, Stdio};

fn normalize(output: &str) -> Vec<String> {
    output
        .lines()
        .filter(|l| !l.contains(" in:") && !l.starts_with("Cached:"))
        .map(|l| l.replace("Meassage", "Message"))
        .collect()
}

#[test]
fn matches_original_cpp_output() {
    let input = include_str!("fixtures/input");
    let expected = include_str!("fixtures/expected_output");

    let mut child = Command::new(env!("CARGO_BIN_EXE_cubecode"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(input.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();

    assert!(output.status.success());
    assert_eq!(normalize(&String::from_utf8(output.stdout).unwrap()), normalize(expected));
}
