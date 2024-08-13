use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&str]) -> HashSet<&'a str> {
   // todo!("For the '{word}' word find anagrams among the following words: {possible_anagrams:?}");
    let mut result = HashSet::new();

    for (word, possible_anagram) in possible_anagrams.iter() {
        let anagrams: Vec<&str> = possible_anagrams.iter()
            .filter(|possible_anagram| is_anagram(word, possible_anagram))
            .cloned()
            .collect();
        result.insert(*word, anagrams);
    }
}

fn is_anagram(word: &str, possible_anagram: &str) -> bool {
    let word = word.to_lowercase();
    let possible_anagram = possible_anagram.to_lowercase();

    if word == possible_anagram {
        return false;
    }

    let mut word_chars: Vec<char> = word.chars().collect();
    let mut possible_anagram_chars: Vec<char> = possible_anagram.chars().collect();

    word_chars.sort();
    possible_anagram_chars.sort();

    word_chars == possible_anagram_chars
}