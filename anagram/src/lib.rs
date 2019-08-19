use std::collections::HashSet;

fn sorted_char_vec<'a>(s: &'a str) -> Vec<char> {
    let mut v = s.chars().collect::<Vec<char>>();
    v.sort();
    v
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();
    let letters = sorted_char_vec(&word);
    possible_anagrams
        .iter()
        .filter(|w| {
            let lowercased = w.to_lowercase();
            letters == sorted_char_vec(&lowercased) && &word != &lowercased
        })
        .copied()
        .collect::<HashSet<&'a str>>()
}
