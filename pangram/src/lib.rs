use std::collections::HashSet;

const ALPHABET_LENGTH: usize = 26;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut set: HashSet<char> = HashSet::new();

    for c in sentence
        .to_ascii_lowercase()
        .chars()
        .filter(|c| !c.is_whitespace() && c.is_alphabetic() && c.is_ascii())
    {
        set.insert(c.clone());
    }

    set.len() == ALPHABET_LENGTH
}
