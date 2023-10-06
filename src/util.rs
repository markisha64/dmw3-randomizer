use rand_xoshiro::rand_core::RngCore;
use rand_xoshiro::Xoshiro256StarStar;

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

        // skip advancing rng if unnecessary
        if alphabet_length > 1 {
            for _ in 0..shuffles {
                for i in 0..(alphabet_length - 2) {
                    let uniform = rng.next_u64() as usize;
                    let j = i + uniform % (alphabet_length - i - 1);

                    chunk.swap(i, j);
                }
            }
        }

        vector.extend(chunk);
    }

    vector.truncate(length);

    if length > 1 {
        for i in 0..(length - 2) {
            let uniform: usize = rng.next_u64() as usize;
            let j = i + uniform % (length - i - 1);

            vector.swap(i, j);
        }
    }

    vector
}
