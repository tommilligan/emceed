extern crate clap;
extern crate mcmc_decrypt;
extern crate rand;
extern crate rand_pcg;
extern crate serde_json;

use clap::{App, Arg};
use mcmc_decrypt::CorpusStats;
use rand::seq::SliceRandom;
use rand::Rng;
use std::fs;

#[derive(Debug)]
pub struct Message {
    text: String,
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
pub struct Alphabet<T> {
    chars: Vec<char>,
    prng: T,
}

impl<T: Rng> Alphabet<T> {
    pub fn choose(&mut self) -> char {
        self.chars.choose(&mut self.prng).unwrap().to_owned()
    }
}

fn main() {
    let matches = App::new("emceede")
        .version("0.1")
        .author("Tom Milligan")
        .about("Decrypt a caeser cipher using emcee")
        .arg(
            Arg::with_name("STATS")
                .help("Corpus stats used to guide the model")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("MESSAGE")
                .help("Encoded message")
                .required(true)
                .index(2),
        )
        .get_matches();

    let stats_file = matches.value_of("STATS").unwrap();
    let stats = fs::read_to_string(stats_file).expect("Could not read stats file");

    let message_file = matches.value_of("MESSAGE").unwrap();
    let mut message = Message {
        text: fs::read_to_string(message_file).expect("Could not read message file"),
    };

    let mut message_stats = CorpusStats::new();
    message_stats.read(&message.text).finish();

    let rng: rand_pcg::Mcg128Xsl64 = rand::SeedableRng::seed_from_u64(0);
    let mut alphabet = Alphabet {
        prng: rng,
        chars: "abcdef ".chars().collect(),
    };

    println!("Original message: {:?}", message);
    for _ in 0..10 {
        message.swap(alphabet.choose(), alphabet.choose());
        println!("{:?}", message);
    }
}
