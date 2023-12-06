use std::env;
use std::io::ErrorKind;
use std::process;

use chapter_12_minigrep::config::Config;
use chapter_12_minigrep::error::RunError;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|error| {
        eprintln!("{error}");
        process::exit(1);
    });
    match chapter_12_minigrep::run(&config) {
        Ok(_) => (),
        Err(error) => match error {
            RunError::InputOutputError(error) => {
                match error.kind() {
                    ErrorKind::NotFound => {
                        eprintln!("Error: File `{}` not found.", config.file_path)
                    }
                    _ => eprintln!("Error: {error}"),
                }
                process::exit(1);
            }
            RunError::Other(error) => {
                eprintln!("Error: {error}");
                process::exit(1);
            }
        },
    }
}
