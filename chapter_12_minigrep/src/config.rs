use std::env;

#[derive(Debug, PartialEq)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut arguments: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        arguments.next();
        let error_message = "Usage: minigrep <query> <file_path>";
        let query = match arguments.next() {
            Some(arg) => arg,
            None => return Err(error_message),
        };
        let file_path = match arguments.next() {
            Some(arg) => arg,
            None => return Err(error_message),
        };
        let ignore_case = match env::var("IGNORE_CASE") {
            Ok(value) => value == "true",
            Err(_) => false,
        };
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
