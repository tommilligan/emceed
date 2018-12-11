use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashMap;
use std::hash::Hash;

pub fn random_lookup<T: Hash + Clone + Eq, R: Rng>(
    collection: &Vec<T>,
    rng: &mut R,
) -> HashMap<T, T> {
    // Randomise HashMap lookup between items of vector
    let alphabet = collection.to_vec();
    let mut shuffled = collection.to_vec();
    shuffled.shuffle(rng);

    let mut lookup = HashMap::new();
    for (a, b) in alphabet.into_iter().zip(shuffled.into_iter()) {
        lookup.insert(a, b);
    }
    lookup
}
