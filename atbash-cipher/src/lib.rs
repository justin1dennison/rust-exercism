#[macro_use]
extern crate lazy_static;

pub const LETTERS: &str = "abcdefghijklmnopqrstuvwxyz";

lazy_static! {
    static ref REVERSED: Vec<char> = LETTERS.chars().rev().collect::<Vec<char>>();
    static ref ALPHABET: Vec<char> = LETTERS.chars().collect::<Vec<char>>();
}

fn transform(text: &str, domain: Vec<char>, range: Vec<char>) -> Vec<String> {
    let lookup = |character: char| {
        domain
            .iter()
            .position(|&x| x == character)
            .and_then(|n| Some(range[n].to_string()))
            .or_else(|| Some(character.to_string()))
            .unwrap()
    };
    text.to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(lookup)
        .collect::<Vec<String>>()
}

fn chunk_to_string(chunk: &[String]) -> String {
    chunk.iter().map(|s| s.to_string()).collect::<String>()
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    transform(plain, ALPHABET.to_vec(), REVERSED.to_vec())
        .chunks(5)
        .map(chunk_to_string)
        .collect::<Vec<String>>()
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    transform(cipher, REVERSED.to_vec(), ALPHABET.to_vec()).join("")
}
