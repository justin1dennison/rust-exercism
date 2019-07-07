use std::str;

enum CharPosition {
    Found(usize),
    NotFound,
}
pub const ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyz";
/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let plain = plain
        .to_lowercase()
        .replace(" ", "")
        .replace(".", "")
        .replace("?", "")
        .replace(",", "");
    let mut result = String::new();
    let reversed: String = ALPHABET.chars().rev().collect();
    for character in plain.chars() {
        if character.is_ascii() {
            let position = match ALPHABET.find(character) {
                Some(n) => CharPosition::Found(n),
                None => CharPosition::NotFound,
            };
            if let CharPosition::Found(position) = position {
                let cipher = match reversed.get(position..position + 1) {
                    Some(s) => s,
                    None => "",
                };
                result.push_str(cipher);
            } else {
                result.push_str(&character.to_string());
            };
        }
    }
    result
        .as_bytes()
        .chunks(5)
        .map(str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join(" ")
        .to_string()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    let cipher = cipher.replace(" ", "");
    let mut result = String::new();
    let reversed = ALPHABET.chars().rev().collect::<String>();
    for character in cipher.chars() {
        let position = match reversed.find(character) {
            Some(n) => CharPosition::Found(n),
            None => CharPosition::NotFound,
        };
        if let CharPosition::Found(n) = position {
            let letter = match ALPHABET.get(n..n + 1) {
                Some(found) => found,
                None => "",
            };
            result.push_str(letter);
        } else {
            result.push_str(&character.to_string());
        }
    }
    result
}
