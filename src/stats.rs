extern crate serde;

use std::collections::HashMap;

// use space as a default character as it's the most common
static CHAR_DEFAULT: char = ' ';

#[derive(Serialize, Deserialize, Debug)]
pub struct CorpusStats {
    frequency: u64,
    symbols: HashMap<char, CorpusStats>,
    ratio: f64,
}

impl CorpusStats {
    pub fn new() -> CorpusStats {
        CorpusStats {
            frequency: 0,
            symbols: HashMap::new(),
            ratio: 0.0,
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
        self.sum_frequencies().calculate_ratios()
    }

    // calculate difference between two sets of stats
    pub fn diff(&self, other: &CorpusStats) -> f64 {
        let mut diff = 0.0;
        for key in self.symbols.keys() {
            diff +=
                (self.ratio(&key) - other.ratio(&key)).abs().powi(2) * self.frequency(&key) as f64
        }
        diff
    }

    pub fn ratio(&self, c: &char) -> f64 {
        match self.symbols.get(c) {
            Some(x) => x.ratio,
            None => 0.0,
        }
    }

    pub fn frequency(&self, c: &char) -> u64 {
        match self.symbols.get(c) {
            Some(x) => x.frequency,
            None => 0,
        }
    }

    fn increment_transition(&mut self, from: char, to: char) -> &mut CorpusStats {
        self.symbols
            .entry(from)
            .or_insert(CorpusStats::new())
            .symbols
            .entry(to)
            .or_insert(CorpusStats::new())
            .frequency += 1;
        self
    }

    fn copy_symbol_keys(&self) -> Vec<char> {
        self.symbols.keys().map(|k| k.to_owned()).collect()
    }

    fn sum_frequencies(&mut self) -> &mut CorpusStats {
        // if we have sub frequencies, add them
        if self.symbols.len() > 0 {
            // recurse down and get child frequencies first
            for key in &self.copy_symbol_keys() {
                self.symbols.get_mut(&key).unwrap().sum_frequencies();
            }
            self.frequency = self.symbols.values().map(|stat| stat.frequency).sum();
        }
        self
    }

    fn calculate_ratios(&mut self) -> &mut CorpusStats {
        for key in &self.copy_symbol_keys() {
            let mut symbol = self.symbols.get_mut(&key).unwrap();
            symbol.ratio = symbol.frequency as f64 / self.frequency as f64;

            // recurse down and process children
            symbol.calculate_ratios();
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

        // test frequencies
        assert_eq!(stats.frequency, 6);
        assert_eq!(stats.symbols.get(&'t').unwrap().frequency, 2);
        assert_eq!(
            stats
                .symbols
                .get(&'t')
                .unwrap()
                .symbols
                .get(&'a')
                .unwrap()
                .frequency,
            1
        );
        assert!(stats.symbols.get(&'z').is_none());

        // test ratios
        assert_eq!(stats.ratio, 0.0);
        assert_eq!(stats.symbols.get(&'t').unwrap().ratio, 1.0 / 3.0);
        assert_eq!(
            stats
                .symbols
                .get(&'t')
                .unwrap()
                .symbols
                .get(&'a')
                .unwrap()
                .ratio,
            0.5
        );
        assert!(stats.symbols.get(&'z').is_none());
    }
}
