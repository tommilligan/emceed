use std::io::{self, Read};

extern crate serde_json;

extern crate mcmc_decrypt;

fn main() {
    // read everything from stdin
    let stdin = io::stdin();
    let mut corpus = String::new();
    stdin
        .lock()
        .read_to_string(&mut corpus)
        .expect("Couldn't read from stdin");

    // calculate stats for the text
    let mut stats = mcmc_decrypt::CorpusStats::new();
    stats.read(&corpus).finish();

    // output stats to stdout as json
    println!("{}", serde_json::to_string(&stats).unwrap());
}
