pub fn reply(message: &str) -> &str {
    if is_question(message) && is_yelling(message) {
        "Calm down, I know what I'm doing!"
    } else if is_yelling(message) {
        "Whoa, chill out!"
    } else if is_question(message) {
        "Sure."
    } else if is_silence(message) {
        "Fine. Be that way!"
    } else {
        "Whatever."
    }
}

fn is_silence(s: &str) -> bool {
    s.to_string().trim().is_empty()
}

fn is_question(s: &str) -> bool {
    s.to_string().trim().ends_with("?")
}

fn is_yelling(s: &str) -> bool {
    let as_string = s.to_string();
    let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz".to_string();
    let contains_letters = |x: &str| {
        letters
            .chars()
            .map(|l| x.contains(l))
            .fold(false, |acc, n| acc || n)
    };

    as_string == as_string.to_uppercase() && contains_letters(s)
}
