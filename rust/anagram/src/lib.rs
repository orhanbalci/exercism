use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut word_slice = word.to_lowercase().chars().collect::<Vec<_>>();
    word_slice.sort();
    let length = word.chars().count();
    possible_anagrams
        .iter()
        .filter(|&possible| {
            let mut pos = possible.to_lowercase().chars().collect::<Vec<_>>();
            pos.sort();
            pos.iter()
                .zip(word_slice.iter())
                .filter(|(&a, &b)| a == b)
                .count()
                == length
                && length > 0
                && *possible.to_lowercase() != word.to_lowercase()
                && length == possible.chars().count()
        })
        .map(|&t| t)
        .collect::<HashSet<&str>>()
}
