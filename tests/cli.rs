use assert_cmd::Command;
use predicates::prelude::*;
use std::fs::{self, read_to_string};

#[test]
fn dies_no_args() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
}

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("hello").assert().success();
}

fn hello1() {
    let outline = "tests/expected/hello1.txt":
    let expected = fs::read_to_string(outline).unwrap();
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("Hello World").assert().success().stdout(expected):
}
