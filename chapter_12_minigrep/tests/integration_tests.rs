use assert_cmd::Command;

#[test]
fn test_main_success_default() {
    let mut command = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    command.args(["the", "poem.txt"]);
    command
        .assert()
        .success()
        .stdout("Then there\'s a pair of us - don\'t tell!\nTo tell your name the livelong day\n");
}

#[test]
fn test_main_success_case_sensitive() {
    let mut command = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    command.args(["I", "poem.txt"]);
    command.env_clear().env("IGNORE_CASE", "false");
    command
        .assert()
        .success()
        .stdout("I\'m nobody! Who are you?\n");
}

#[test]
fn test_main_success_case_insensitive() {
    let mut command = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    command.args(["I", "poem.txt"]);
    command.env_clear().env("IGNORE_CASE", "true");
    command
        .assert()
        .success()
        .stdout("I\'m nobody! Who are you?\nThen there\'s a pair of us - don\'t tell!\nThey\'d banish us, you know.\nHow public, like a frog\nTo tell your name the livelong day\nTo an admiring bog!\n");
}

#[test]
fn test_main_failure_invalid_arguments() {
    let mut command = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    command.args(["the"]);
    command
        .assert()
        .failure()
        .stderr("Usage: minigrep <query> <file_path>\n");
}

#[test]
fn test_main_failure_invalid_file_path() {
    let mut command = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    command.args(["the", "invalid_file_path.txt"]);
    command
        .assert()
        .failure()
        .stderr("Error: File `invalid_file_path.txt` not found.\n");
}
