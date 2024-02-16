use std::collections::HashSet;

fn sort_string(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort();
    chars.into_iter().collect()
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&'a str]) -> HashSet<&'a str> {
    let mut anagrams = HashSet::new();
    let sorted_word = sort_string(word);

    for possible_word in possible_anagrams {
        let sorted_possible_word = sort_string(possible_word);
        if sorted_possible_word.to_lowercase() == sorted_word.to_lowercase()
            && *possible_word != word
        {
            anagrams.insert(*possible_word);
        }
    }
    anagrams
}
