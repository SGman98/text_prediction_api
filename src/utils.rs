pub fn get_regex(text: &str, keys: Vec<String>) -> String {
    text.chars()
        .map(|letter| {
            let col = keys.iter().find_map(|row| row.find(letter));

            match col {
                Some(3 | 4) => {
                    let chars = keys
                        .iter()
                        .map(|row| row.chars().skip(3).take(2).collect::<String>())
                        .collect::<String>();
                    format!("[{}]", chars)
                }
                Some(5 | 6) => {
                    let chars = keys
                        .iter()
                        .map(|row| row.chars().skip(5).take(2).collect::<String>())
                        .collect::<String>();
                    format!("[{}]", chars)
                }
                Some(c) => {
                    let chars = keys
                        .iter()
                        .map(|row| row.chars().nth(c).unwrap().to_string())
                        .collect::<String>();
                    format!("[{}]", chars)
                }
                None => letter.to_string(),
            }
        })
        .collect::<String>()
}
