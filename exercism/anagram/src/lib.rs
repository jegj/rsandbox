use std::collections::HashSet;

fn sort_string(s: String) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort();
    chars.into_iter().collect()
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&'a str]) -> HashSet<&'a str> {
    let mut anagrams = HashSet::new();
    let sorted_word = sort_string(word.to_lowercase());

    for possible_word in possible_anagrams {
        let sorted_possible_word = sort_string(possible_word.to_lowercase());
        if sorted_possible_word.to_lowercase() == sorted_word.to_lowercase()
            && *possible_word.to_lowercase() != word.to_lowercase()
        {
            anagrams.insert(*possible_word);
        }
    }
    anagrams
}
