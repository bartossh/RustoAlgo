use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut cf: HashSet<u32> = HashSet::new();
    for f in factors.iter() {
        if *f == 0 {
            continue;
        }
        for i in 1..limit {
            if i % f == 0 {
                cf.insert(i);
            }
        }
    }

    cf.iter().sum()
}
