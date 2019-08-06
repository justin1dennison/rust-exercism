
pub const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";
/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let reversed: String = ALPHABET.chars().rev().collect();
    plain
        .to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|character| {
            ALPHABET
                .find(character)
                .and_then(|n| reversed.get(n..=n).map(|s| s.to_string()))
                .or_else(|| Some(character.to_string()))

        })
        .filter_map(|e| e)
        .collect::<Vec<String>>()
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
    let reversed = ALPHABET.chars().rev().collect::<String>();
    cipher
        .to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|character| {
            reversed
                .find(character)
                .and_then(|n| ALPHABET.get(n..=n).map(|s| s.to_string()))
                .or_else(|| Some(character.to_string()))

        })
        .filter_map(|e| e)
        .collect::<Vec<String>>()
        .join("")
        .to_string()
}
