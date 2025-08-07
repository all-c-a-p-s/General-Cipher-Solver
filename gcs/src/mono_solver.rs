use utils::{vector_crossover, vector_initialise, vector_mutate, LETTERS};

#[derive(Clone)]
pub struct Key {
    letter_map: [u8; 26],
}

impl Default for Key {
    fn default() -> Self {
        Self::new()
    }
}

impl Key {
    #[must_use]
    pub fn new() -> Self {
        let sample = LETTERS.to_vec();
        Self {
            letter_map: vector_initialise::<26, u8>(&sample),
        }
    }
}

#[must_use]
pub fn decipher(ct: &Vec<u8>, k: Key) -> Vec<u8> {
    ct.iter()
        .map(|x| k.letter_map[*x as usize - 'A' as usize])
        .collect()
}

#[must_use]
pub fn mutate(k: Key) -> Key {
    Key {
        letter_map: vector_mutate::<26, u8>(k.letter_map),
    }
}

#[must_use]
pub fn crossover(k1: Key, k2: Key) -> Key {
    let sample = LETTERS.to_vec();
    Key {
        letter_map: vector_crossover::<26, u8>(k1.letter_map, k2.letter_map, &sample),
    }
}
