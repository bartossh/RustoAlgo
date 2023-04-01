// with help of 'iamhere2's' solution that I found the most straight forward

use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut factorsHash: HashMap<char, i64> = HashMap::new();
    let mut sign = -1;
    let mut pos = 0;

    input
        .chars()
        .filter(|c| !c.is_whitespace())
        .rev()
        .into_iter()
        .for_each(|c| match c {
            '=' => {
                sign = 1;
                pos = 0;
            }
            '+' => {
                pos = 0;
            }
            _ => {
                let factor = sign * 10_i64.pow(pos);
                pos += 1;
                *factorsHash.entry(c).or_insert(0) += factor;
            }
        });

    let (characters, factors): (Vec<char>, Vec<i64>) = factorsHash
        .iter()
        .sorted_by_key(|(_, value)| -(**value).abs())
        .unzip();

    let firstsPositions = input
        .split(&['+', '='])
        .filter_map(|s| s.trim().chars().next())
        .collect::<HashSet<_>>();

    println!("{:?}", firstsPositions);

    for perm in (0..=9).permutations(characters.len()) {
        let sum = perm
            .iter()
            .enumerate()
            .map(|(idx, value)| *value * factors.get(idx).unwrap())
            .sum::<i64>();

        if sum == 0
            && !perm.iter().enumerate().any(|(idx, value)| {
                *value == 0 && firstsPositions.contains(characters.get(idx).unwrap())
            })
        {
            return Some(HashMap::from_iter(
                perm.iter()
                    .enumerate()
                    .map(|(idx, value)| (*characters.get(idx).unwrap(), *value as u8)),
            ));
        }
    }
    None
}
