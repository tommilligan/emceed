use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashMap;

use super::utils::random_lookup;

#[derive(Debug, Clone)]
pub struct CipherKey<R> {
    alphabet: Vec<char>,
    key: HashMap<char, char>,
    rng: R,
}

impl<R: Rng> CipherKey<R> {
    pub fn new(chars: &str, mut rng: R) -> CipherKey<R> {
        let alphabet = chars.chars().collect();
        CipherKey {
            key: random_lookup(&alphabet, &mut rng),

            alphabet,
            rng: rng,
        }
    }

    pub fn choose(&mut self) -> char {
        self.alphabet
            .choose(&mut self.rng)
            .expect("No alphabet choice")
            .to_owned()
    }

    pub fn perturb(&mut self) -> &mut CipherKey<R> {
        let a_key = self.choose();
        let b_key = self.choose();
        let a_value = self.key.get(&a_key).expect("Get perturb a").to_owned();
        let b_value = self.key.get(&b_key).expect("Get perturb b").to_owned();
        self.key.insert(a_key, b_value);
        self.key.insert(b_key, a_value);
        self
    }

    pub fn decipher(&self, ciphered: &str) -> String {
        ciphered
            .chars()
            .map(|c| self.key.get(&c).unwrap_or(&c).to_owned())
            .collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    extern crate rand;

    use rand_pcg::Mcg128Xsl64;
    use std::collections::HashMap;

    use super::CipherKey;

    #[test]
    fn test_cipher_key() {
        let rng: Mcg128Xsl64 = rand::SeedableRng::seed_from_u64(0);
        let mut cipher_key = CipherKey::new("abc", rng);
        let ciphertext = "aabbcc";

        // test initialisation
        assert_eq!(cipher_key.alphabet, vec!['a', 'b', 'c']);
        let mut key = HashMap::new();
        key.insert('b', 'c');
        key.insert('a', 'a');
        key.insert('c', 'b');
        assert_eq!(cipher_key.key, key);
        assert_eq!(cipher_key.decipher(ciphertext), "aaccbb");

        // test pertubation - should swap two keys
        cipher_key.perturb();
        assert!(cipher_key.key != key);
        key.insert('a', 'c');
        key.insert('b', 'a');
        assert_eq!(cipher_key.key, key);
        assert_eq!(cipher_key.decipher(ciphertext), "ccaabb");
    }

    #[test]
    fn test_cipher_key_non_alphabet() {
        let rng: Mcg128Xsl64 = rand::SeedableRng::seed_from_u64(0);
        let cipher_key = CipherKey::new("abc", rng);
        let ciphertext = "abcxyz";

        // test deciphering non alphabet characters
        assert_eq!(cipher_key.decipher(ciphertext), "acbxyz");
    }
}
