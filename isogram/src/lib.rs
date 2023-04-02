use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut set: HashSet<char> = HashSet::new();
    for c in candidate.chars() {
        if c.is_alphabetic() && c != ' ' && c != '-' {
            if set.contains(&c.to_ascii_lowercase()) {
                return false;
            }
            set.insert(c.to_ascii_lowercase().clone());
        }
    }
    true
}
