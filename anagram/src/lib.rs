use std::collections::HashSet;


fn is_anagram<'a>(fst: &'a str, snd: &'a str) -> bool {
    let mut fst_letters = fst.chars().collect::<Vec<char>>();
    let mut snd_letters = snd.chars().collect::<Vec<char>>();
    fst_letters.sort();
    snd_letters.sort();
    fst_letters == snd_letters && fst != snd
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    possible_anagrams
        .iter()
        .filter(|w| is_anagram(&word.to_lowercase(), &w.to_lowercase()))
        .copied()
        .collect::<HashSet<&'a str>>()

}
