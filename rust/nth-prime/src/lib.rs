use std::convert::TryFrom;
use std::vec::Vec;

pub fn nth(n: u32) -> u32 {
    let mut primes = Vec::new();
    for i in 2.. {
        if primes.len() == usize::try_from(n).unwrap() + 1 {
            return *primes.last().unwrap();
        }
        if primes
            .iter()
            .filter(|&&s| s <= (i as f32).sqrt().ceil() as u32)
            .all(|&x| i % x != 0)
            || i == 2
        {
            primes.push(i);
        }
    }

    return 0;
}
