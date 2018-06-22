pub fn series(digits: &str, len: usize) -> Vec<String> {
    match len {
        0 => vec!["".to_string(); (digits.len() + 1) as usize],
        1 => digits.split("").map(|s| String::from(s)).collect::<Vec<String>>(),
        2 => {
            let mut results: Vec<String> = vec![];
            let chars = digits.chars();
            let count = digits.len() / len;
            for i in 0..count + 2 {
                let chars = chars.clone();
                let pair = chars.skip(i as usize).take(2).collect::<String>();
                results.push(pair);
            } 
            results
        },
        _ if digits.len() == len => vec![digits.to_string()],
        _ => vec![]
    }    
}



