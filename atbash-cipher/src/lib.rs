#[macro_use]
extern crate lazy_static;

pub const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

lazy_static! {
    static ref REVERSED: String = ALPHABET.chars().rev().collect();
}

fn transform(text: &str, domain: &str, range: &str) -> Vec<String> {
    text.to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|character| {
            domain
                .find(character)
                .and_then(|n| range.get(n..=n).map(|s| s.to_string()))
                .or_else(|| Some(character.to_string()))
        })
        .filter_map(|e| e)
        .collect::<Vec<String>>()
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    transform(plain, ALPHABET, &REVERSED)
        .chunks(5)
        .map(|chunk| {
            chunk
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join("")
        })
        .collect::<Vec<String>>()
        .join(" ")
        .to_string()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    transform(cipher, &REVERSED, ALPHABET).join("").to_string()
}
