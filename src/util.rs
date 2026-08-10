use std::collections::HashSet;

use rand_xoshiro::rand_core::RngCore;
use rand_xoshiro::Xoshiro256StarStar;
use std::hash::Hash;

pub fn uniform_random_vector<T: Clone>(
    alphabet: &Vec<T>,
    length: usize,
    shuffles: u8,
    rng: &mut Xoshiro256StarStar,
) -> Vec<T> {
    let mut vector: Vec<T> = Vec::new();
    let alphabet_length = alphabet.len();

    for _ in 0..((length - 1) / alphabet_length + 1) {
        let mut chunk = alphabet.clone();

        shuffle(&mut chunk, shuffles, rng);

        vector.extend(chunk);
    }

    vector.truncate(length);

    shuffle(&mut vector, shuffles, rng);

    vector
}

// Fisher-Yates shuffles
pub fn shuffle<T>(array: &mut Vec<T>, shuffles: u8, rng: &mut Xoshiro256StarStar) {
    let len = array.len();

    if len > 1 {
        for _ in 0..shuffles {
            for i in 0..(len - 1) {
                let uniform: usize = rng.next_u64() as usize;
                let j = i + uniform % (len - i);

                array.swap(i, j);
            }
        }
    }
}

pub fn unique_vec<V>(array: V) -> Vec<V::Item>
where
    V: IntoIterator,
    V::Item: Eq + Hash + Clone,
{
    let mut mapped: HashSet<V::Item> = HashSet::new();
    let mut rv = Vec::new();

    for item in array {
        if mapped.insert(item.clone()) {
            rv.push(item);
        }
    }

    rv
}
