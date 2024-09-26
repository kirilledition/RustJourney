use std::collections::HashSet;

fn word_to_letter_vector(word: &str) -> Vec<char> {
    let mut word_letters = word.to_lowercase().chars().collect::<Vec<char>>();
    word_letters.sort();
    word_letters
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let word_letter_vector = word_to_letter_vector(word);

    let selected_anagrams = possible_anagrams
        .iter()
        .filter(|anagram| {
            !anagram.to_lowercase().eq(&word.to_lowercase())
                && word_to_letter_vector(*anagram) == word_letter_vector
        })
        .copied()
        .collect::<HashSet<&str>>();

    selected_anagrams
}
