extern crate clap;
extern crate mcmc_decrypt;
extern crate rand;
extern crate rand_pcg;
extern crate serde_json;

use clap::{App, Arg};
use mcmc_decrypt::message::CipherKey;
use mcmc_decrypt::stats::CorpusStats;
use rand_pcg::Mcg128Xsl64;
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
    let message = fs::read_to_string(message_file).expect("Could not read message file");

    let mut message_stats = CorpusStats::new();
    message_stats.read(&message).finish();
    println!("Starting with message: {}", &message);

    let rng: Mcg128Xsl64 = rand::SeedableRng::seed_from_u64(0);
    //let alphabet = "abcdef".to_string();
    let alphabet = r###"= -,;:!?/.'"()[]*0123456789abcdefghijklmnopqrstuvwxyz"###.to_string();

    let cipher_key = CipherKey::new(&alphabet, rng);
    let plain = cipher_key.decipher(&message);
    let mut plain_stats = CorpusStats::new();
    plain_stats.read(&plain).finish();

    for i in 0..10000 {
        let mut new_key = cipher_key.clone();
        new_key.perturb();
        let new_plain = cipher_key.decipher(&message);
        let mut new_plain_stats = CorpusStats::new();
        new_plain_stats.read(&new_plain).finish();

        let new_score = message_stats.diff(&new_plain_stats);
        let old_score = message_stats.diff(&plain_stats);

        if new_score < old_score {
            plain_stats = new_plain_stats;
            println!("New message (i={}): {}", i, new_plain);
        }
    }
}
