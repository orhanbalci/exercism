use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut word_slice = word.to_lowercase().chars().collect::<Vec<_>>();
    let lowercase_word = word.to_lowercase();
    word_slice.sort();
    let length = word.chars().count();
    possible_anagrams
        .iter()
        .filter(|&possible| {
            let lowercase_pos = possible.to_lowercase();
            let mut pos = lowercase_pos.chars().collect::<Vec<_>>();
            pos.sort();
            pos == word_slice
                && length > 0
                && lowercase_pos != lowercase_word 
                && length == possible.chars().count()
        })
        .copied()
        .collect::<HashSet<&str>>()
}
