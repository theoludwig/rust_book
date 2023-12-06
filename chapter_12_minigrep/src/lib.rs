use colored::*;
use config::Config;
use error::RunError;
use search::{search, search_case_insensitive};
use std::fs;

pub mod config;
pub mod error;
pub mod search;

pub fn run(config: &Config) -> Result<(), RunError> {
    let file_content = fs::read_to_string(&config.file_path)?;
    let mut lines: Vec<String> = file_content.lines().map(|line| line.to_string()).collect();
    let matching_items = if config.ignore_case {
        search_case_insensitive(&config.query, &lines)
    } else {
        search(&config.query, &lines)
    };
    let mut lines_indexes_differences_colored: Vec<usize> = vec![0; lines.len()];
    let mut matching_line_indexes: Vec<usize> = Vec::new();
    for item in matching_items {
        let has_already_matched = matching_line_indexes.contains(&item.line_index);
        if !has_already_matched {
            matching_line_indexes.push(item.line_index);
        }
        let line_index_difference_colored = lines_indexes_differences_colored[item.line_index];
        let start_index = item.start_index + line_index_difference_colored;
        let end_index = item.end_index + line_index_difference_colored;
        let matching_text = lines[item.line_index]
            .get(start_index..=end_index)
            .expect("Failed to get `matching_text`.");
        let colored_text = matching_text.red().bold().to_string();
        lines_indexes_differences_colored[item.line_index] +=
            colored_text.len() - matching_text.len();
        lines[item.line_index].replace_range(start_index..=end_index, &colored_text);
    }
    for (index, line) in lines.iter().enumerate() {
        if matching_line_indexes.contains(&index) {
            println!("{line}");
        }
    }
    Ok(())
}
