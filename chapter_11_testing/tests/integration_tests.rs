use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_main_success() {
    let mut command = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    command.write_stdin(["3", "1", "2", "3"].join("\n"));
    command
        .assert()
        .success()
        .stdout(predicate::str::contains("1 + 2 + 3 = 6"));
}

#[test]
fn test_main_failure_invalid_numbers_count() {
    let mut command = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    command.write_stdin(["0"].join("\n"));
    command
        .assert()
        .failure()
        .stderr(predicate::str::contains("`numbers_count` should not be 0."));
}

#[test]
fn test_main_failure_invalid_number() {
    let mut command = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    command.write_stdin(["1", "a"].join("\n"));
    command.assert().failure().stderr(predicate::str::contains(
        "Failed to convert `number0` as an `i32`.",
    ));
}
