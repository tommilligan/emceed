extern crate rand;
extern crate rand_pcg;

use self::rand::seq::SliceRandom;
use self::rand::Rng;
use self::rand_pcg::Mcg128Xsl64;

#[derive(Debug)]
pub struct Message {
    pub text: String,
}

impl Message {
    pub fn swap(&mut self, a: char, b: char) -> () {
        self.text = self
            .text
            .chars()
            .map(|c| match c {
                _ if c == a => b,
                _ if c == b => a,
                _ => c,
            })
            .collect();
    }
}

#[derive(Debug)]
pub struct Alphabet {
    chars: Vec<char>,
    prng: Mcg128Xsl64,
}

impl Alphabet {
    pub fn new(chars: &str) -> Alphabet {
        let rng: Mcg128Xsl64 = rand::SeedableRng::seed_from_u64(0);
        Alphabet {
            chars: chars.chars().collect(),
            prng: rng,
        }
    }
    pub fn choose(&mut self) -> char {
        self.chars.choose(&mut self.prng).unwrap().to_owned()
    }
}
