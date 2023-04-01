use digits_iterator::*;

pub fn is_armstrong_number(num: u32) -> bool {
    let digits: Vec<_> = num.digits().collect();

    let mut sum_pow = 0;
    for digit in digits.iter() {
        let digit = *digit as u64;
        sum_pow += digit.pow(digits.len() as u32) as u64;
    }
    if sum_pow == num as u64 {
        return true;
    }

    false
}
