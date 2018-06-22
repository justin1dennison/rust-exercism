pub fn series(digits: &str, len: usize) -> Vec<String> {
    match len {
        0 => vec!["".to_string(); (digits.len() + 1) as usize],
        _ if digits.len() == len => vec![digits.to_string()],
        _ => digits.chars()
                   .collect::<Vec<char>>()
                   .windows(len)
                   .map(|a| a.iter().collect::<String>())
                   .collect::<Vec<String>>()
    }    
}



