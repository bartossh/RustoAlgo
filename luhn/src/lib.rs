/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {

    for c in code.chars() {
        if c.is_whitespace() {
            continue;
        }
        if c.is_digit(10) {
            continue;
        }
        return false;
    }

    if code.trim().len() <= 1 {
        return false;
    }

    
    let sum: u32 = code
        .chars()
        .filter(|c| !c.is_whitespace())
        .rev()
        .enumerate()
        .map(|(i, c)| {
            if !c.is_digit(10) {
                return 0;
            }
            let mut d = c.to_digit(10).unwrap();
            if i % 2 == 1 {
                d *= 2;
                if d > 9 {
                    d -= 9;
                }
            }
            d
        })
        .sum();
    sum % 10 == 0
}
