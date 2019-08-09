use std::collections::HashSet;
use std::iter::FromIterator;


fn is_anagram<'a>(fst: &'a str, snd: &'a str) -> bool {
    let mut fst_letters = fst.chars().collect::<Vec<char>>();
    let mut snd_letters = snd.chars().collect::<Vec<char>>();
    fst_letters.sort();
    snd_letters.sort();
    let same_letters = fst_letters
        .iter()
        .zip(snd_letters.iter())
        .fold(true, |acc, (l, r)| acc && l == r);
    (fst_letters == snd_letters &&
     fst.len() == snd.len() &&
     same_letters && 
     fst != snd)
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let anagrams = possible_anagrams
        .iter()
        .filter(|w| is_anagram(&word.to_lowercase(), &w.to_lowercase()));
    HashSet::from_iter(anagrams.cloned())
}
