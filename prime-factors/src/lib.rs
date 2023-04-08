pub fn factors(n: u64) -> Vec<u64> {
    let mut prime_factors: Vec<u64> = Vec::new();
    if n == 0 || n == 1 {
        return prime_factors;
    }

    if n == 2 {
        prime_factors.push(2);
        return prime_factors;   
    }

    let mut nn = n.clone();
    let mut numbers = 2..;

    while nn > 1 {
        let candidate = numbers.next().unwrap();

        'inner: loop {

            if nn % candidate == 0 {
                nn /= candidate;
                prime_factors.push(candidate);
            } else {
                break 'inner;
            }
        }
    }

    prime_factors
}

