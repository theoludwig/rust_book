#[derive(Debug, PartialEq)]
pub struct MatchingItem {
    pub start_index: usize,
    pub end_index: usize,
    pub line_index: usize,
}

// With Iterator
pub fn search(query: &str, lines: &[String]) -> Vec<MatchingItem> {
    lines
        .iter()
        .enumerate()
        .flat_map(|(line_index, line)| {
            line.char_indices().filter_map(move |(index, character)| {
                query.chars().next().and_then(|first_character| {
                    if first_character == character {
                        let end_index = index + (query.len() - 1);
                        line.get(index..=end_index).and_then(|matching_text| {
                            if matching_text == query {
                                Some(MatchingItem {
                                    start_index: index,
                                    end_index,
                                    line_index,
                                })
                            } else {
                                None
                            }
                        })
                    } else {
                        None
                    }
                })
            })
        })
        .collect()
}

// Without Iterator
// pub fn search(query: &str, lines: &[String]) -> Vec<MatchingItem> {
//     let mut matching_items: Vec<MatchingItem> = Vec::new();
//     for (line_index, line) in lines.iter().enumerate() {
//         for (index, character) in line.char_indices() {
//             if query
//                 .chars()
//                 .next()
//                 .is_some_and(|first_character| first_character == character)
//             {
//                 let end_index = index + (query.len() - 1);
//                 let matching_text = line.get(index..=end_index);
//                 if matching_text.is_some_and(|text| text == query) {
//                     matching_items.push(MatchingItem {
//                         start_index: index,
//                         end_index,
//                         line_index,
//                     });
//                 }
//             }
//         }
//     }
//     matching_items
// }

pub fn search_case_insensitive(query: &str, lines: &[String]) -> Vec<MatchingItem> {
    let query_lowercase = query.to_lowercase();
    let mut lines_lowercase: Vec<String> = Vec::new();
    for line in lines {
        lines_lowercase.push(line.to_lowercase());
    }
    search(&query_lowercase, &lines_lowercase)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let lines = [
            "Rust:".to_string(),
            "safe, fast, productive.".to_string(),
            "Pick three.".to_string(),
            "Duct tape.".to_string(),
        ];
        let expected = vec![MatchingItem {
            start_index: 15,
            end_index: 18,
            line_index: 1,
        }];
        let actual = search(query, &lines);
        assert_eq!(actual, expected);
        assert_eq!(query, lines[1].get(15..=18).unwrap());
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let lines = [
            "Rust:".to_string(),
            "safe, fast, productive.".to_string(),
            "Pick three.".to_string(),
            "Trust me.".to_string(),
        ];
        let expected = vec![
            MatchingItem {
                start_index: 0,
                end_index: 3,
                line_index: 0,
            },
            MatchingItem {
                start_index: 1,
                end_index: 4,
                line_index: 3,
            },
        ];
        let actual = search_case_insensitive(query, &lines);
        assert_eq!(actual, expected);
        assert_eq!("Rust", lines[0].get(0..=3).unwrap());
        assert_eq!("rust", lines[3].get(1..=4).unwrap());
    }
}
