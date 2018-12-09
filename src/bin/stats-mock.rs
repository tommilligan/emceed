extern crate mcmc_decrypt;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::collections::HashMap;
use std::io::{self, BufRead};

static CHAR_SPACE: char = ' ';

#[derive(Serialize, Deserialize, Debug)]
struct SymbolStats {
    frequency: u64,
    next_symbol: HashMap<char, u64>,
}

impl SymbolStats {
    fn new() -> SymbolStats {
        SymbolStats {
            frequency: 0,
            next_symbol: HashMap::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct CorpusStats {
    symbols: HashMap<char, SymbolStats>,
}

impl CorpusStats {
    fn new() -> CorpusStats {
        CorpusStats {
            symbols: HashMap::new(),
        }
    }
    fn increment(&mut self, symbol: char) -> () {
        self.symbols
            .entry(symbol)
            .or_insert(SymbolStats::new())
            .frequency += 1;
    }
}

fn main() {
    let mut stats = CorpusStats::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        for c in line.unwrap().chars() {
            stats.increment(c)
        }
        stats.increment(CHAR_SPACE)
    }
    println!("{}", serde_json::to_string(&stats).unwrap());
}
