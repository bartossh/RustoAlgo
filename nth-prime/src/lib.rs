pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }
    let mut primes = vec![2];    

    let mut i = 3;
    while primes.len() <= n as usize {
        let mut prime = true;
        'check: for p in primes.iter() {
            if i % p == 0 {
                prime = false;
                break 'check;
            }
        }
        if prime {
            primes.push(i);
        }
        i+=1;
    }

    primes.last().unwrap().clone()
}
