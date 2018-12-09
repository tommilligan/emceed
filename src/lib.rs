extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;

// use space as a default character as it's the most common
static CHAR_DEFAULT: char = ' ';

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
pub struct CorpusStats {
    symbols: HashMap<char, SymbolStats>,
}

impl CorpusStats {
    pub fn new() -> CorpusStats {
        CorpusStats {
            symbols: HashMap::new(),
        }
    }

    // Iterate over each character in the corpus, building stats to return.
    pub fn read(&mut self, corpus: &str) -> &mut CorpusStats {
        // save the previous iterated character to calculate transitions
        let mut previous: Option<char> = None;
        for c in corpus.chars() {
            match previous {
                // if this is the first character, treat previous char as space
                None => self.increment_transition(CHAR_DEFAULT, c),
                Some(p) => self.increment_transition(p, c),
            };
            previous = Some(c);
        }

        // handle the final transition probability
        self.increment_transition(previous.unwrap_or(CHAR_DEFAULT), CHAR_DEFAULT)
    }

    // perform expensive tasks after all corpus read
    pub fn finish(&mut self) -> &mut CorpusStats {
        self.sum_frequencies()
    }

    fn increment_transition(&mut self, from: char, to: char) -> &mut CorpusStats {
        *self
            .symbols
            .entry(from)
            .or_insert(SymbolStats::new())
            .next_symbol
            .entry(to)
            .or_insert(0) += 1;
        self
    }

    fn sum_frequencies(&mut self) -> &mut CorpusStats {
        let keys: Vec<char> = self.symbols.keys().map(|k| k.to_owned()).collect();
        for key in keys {
            let sum = self.symbols.get(&key).unwrap().next_symbol.values().sum();
            self.symbols.get_mut(&key).unwrap().frequency = sum;
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::CorpusStats;
    #[test]
    fn test_corpus_stats() {
        let mut stats = CorpusStats::new();
        stats.read("stats").finish();
        assert_eq!(stats.symbols.get(&'t').unwrap().frequency, 2);
        assert!(stats.symbols.get(&'z').is_none());
        assert_eq!(
            stats
                .symbols
                .get(&'t')
                .unwrap()
                .next_symbol
                .get(&'a')
                .unwrap(),
            &1
        );
        assert!(
            stats
                .symbols
                .get(&'t')
                .unwrap()
                .next_symbol
                .get(&'t')
                .is_none()
        );
    }
}
