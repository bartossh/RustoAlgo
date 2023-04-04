use std::collections::HashMap;

const NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {

    match nucleotide_counts(dna) {
        Ok(h) => {
            if let Some(count) = h.get(&nucleotide.to_ascii_uppercase()) {
                return Ok(*count);
            }
            return Err(nucleotide);
        }
        Err(h) => Err(h)
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts: HashMap<char, usize> = NUCLEOTIDES.iter().map(|&c| (c, 0)).collect();
    for c in dna.chars() {
        if !NUCLEOTIDES.contains(&c.to_ascii_uppercase()) {
            return Err(c);
        }
        let count = counts.entry(c.to_ascii_uppercase()).or_insert(0);
        *count += 1;
    }
    Ok(counts)
}
