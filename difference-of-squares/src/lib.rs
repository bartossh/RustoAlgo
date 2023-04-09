pub fn square_of_sum(n: u32) -> u32 {
    let range = 1..=n;
    u32::pow(range.into_iter().sum(), 2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    let range = 1..=n;
    range.into_iter().reduce(|acc, v| acc + u32::pow(v, 2)).unwrap()
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}

