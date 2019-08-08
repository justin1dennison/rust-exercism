#[macro_use]
extern crate lazy_static;

pub const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

lazy_static! {
    static ref REVERSED: Vec<char> = ALPHABET.chars().rev().collect::<Vec<char>>();
}

fn lookup(c: char) -> char {
    if c.is_ascii_alphabetic() {
        REVERSED[ALPHABET.find(c).unwrap()]
    } else {
        c
    }
}

fn chunk_to_string(chunk: &[char]) -> String {
    chunk.iter().map(|s| s.to_string()).collect::<String>()
}

fn transform(text: &str) -> Vec<char> {
    text.to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(lookup)
        .collect()
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    transform(plain)
        .chunks(5)
        .map(chunk_to_string)
        .collect::<Vec<String>>()
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    transform(cipher).iter().collect::<String>()
}
