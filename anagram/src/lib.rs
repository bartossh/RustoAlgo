use std::collections::{HashSet, HashMap};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams: HashSet<&'a str> = HashSet::new();
    let mut footprint: HashMap<char, u8> = HashMap::new();
    for c in word.to_lowercase().chars() {
        match footprint.get(&c) {
            Some(v) => footprint.insert(c, v+1),
            None => footprint.insert(c, 1),
        };
    }
    
    for candidate in possible_anagrams.into_iter() {
        let mut compare_footprint = footprint.clone();
        if *candidate.to_lowercase() == word.to_lowercase() {
            continue;
        }
        if word.len() != candidate.len() {
            continue;
        }
        let mut is_matching = true;
        
        for c in candidate.to_lowercase().chars() {
            match compare_footprint.get(&c) {
                Some(v) => {
                    if *v == 0 {
                        is_matching = false;
                        break;
                    }
                    let v =*v-1;
                   compare_footprint.insert(c, v);
                },
                None => {
                    is_matching = false;
                    break;
                },
            }
        }
        if is_matching {
            anagrams.insert(candidate.clone());
        }
        
    }

    anagrams
}
