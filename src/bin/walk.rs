extern crate clap;
extern crate mcmc_decrypt;
extern crate serde_json;

use clap::{App, Arg};
use mcmc_decrypt::message::{Alphabet, Message};
use mcmc_decrypt::stats::CorpusStats;
use std::fs;

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

    let mut alphabet = Alphabet::new("abcdef ");

    println!("Original message: {:?}", message);
    for _ in 0..10 {
        message.swap(alphabet.choose(), alphabet.choose());
        println!("{:?}", message);
    }
}
