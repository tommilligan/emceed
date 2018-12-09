extern crate mcmc_decrypt;

use std::collections::HashMap;
use std::io::{self, BufRead};

static SPACE: char = ' ';

fn main() {
    let mut frequency: HashMap<char, u64> = HashMap::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        for c in line.unwrap().chars() {
            *frequency.entry(c).or_insert(0) += 1;
        }
        *frequency.entry(SPACE).or_insert(0) += 1;
    }
    println!("{:?}", frequency);
}
