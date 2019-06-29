use std::convert::TryFrom;
use std::vec::Vec;

pub fn nth(n: u32) -> u32 {
    let mut primes = Vec::new();
    for i in 2..u32::max_value() {
        if primes.len() == usize::try_from(n).unwrap() + 1 {
            return *primes.last().unwrap();
        }
        if primes.iter().find(|&&x| i % x == 0) == None || i == 2 {
            primes.push(i);
        }
    }

    return 0;
}
