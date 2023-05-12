pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut results: Vec<String> = Vec::new();
    if len > digits.len() {
        return results;
    }
    for i in 0..=digits.len() - len {
        if i + len > digits.len() {
            break;
        }
        results.push(digits[i..i + len].to_string())
    }
    results
}
