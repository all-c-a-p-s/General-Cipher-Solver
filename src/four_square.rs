use crate::utils::{GRID_LETTERS_J, vector_crossover, vector_initialise, vector_mutate};
use rand::Rng;

#[derive(Copy, Clone)]
pub struct Key {
    //where b and c are the 'keyed' grids, a and d are filled A-Z
    a: [u8; 25],
    b: [u8; 25],
    c: [u8; 25],
    d: [u8; 25],
}

#[derive(Debug)]
struct Coords {
    row: usize,
    column: usize,
}

impl Default for Key {
    fn default() -> Self {
        Self::new()
    }
}

impl Key {
    #[must_use] pub fn new() -> Self {
        let sample = GRID_LETTERS_J.to_vec();
        Self {
            a: GRID_LETTERS_J,
            b: vector_initialise::<25, u8>(&sample),
            c: vector_initialise::<25, u8>(&sample),
            d: GRID_LETTERS_J,
        }
    }
}

fn coordinates(grid_index: usize) -> Coords {
    let row = grid_index / 5;
    let column = grid_index - row * 5;
    Coords { row, column }
}

#[must_use] pub fn decipher(text: &Vec<u8>, k: Key) -> Vec<u8> {
    let grid_index = |row: usize, column: usize| row * 5 + column;
    let find = |grid: [u8; 25], c: u8| grid.iter().position(|&x| x == c).unwrap();
    let decipher_bigram = |bg: &[u8]| {
        let i1 = find(k.b, bg[0]);
        let i2 = find(k.c, bg[1]);

        let coords1 = coordinates(i1);
        let coords2 = coordinates(i2);

        let d_i1 = grid_index(coords1.row, coords2.column);
        let d_i2 = grid_index(coords2.row, coords1.column);

        vec![k.a[d_i1], k.d[d_i2]]
    };

    let mut deciphered = vec![];
    for i in (0..text.len() - 1).step_by(2) {
        let bg = &[text[i], text[i + 1]];
        deciphered.extend(decipher_bigram(bg));
    }

    deciphered
}

#[must_use] pub fn mutate(mut k: Key) -> Key {
    //randomly swaps two elements in one of the two grids
    let mut rng = rand::rng();
    let x = rng.random_bool(0.5); //rng choosing which grid to mutate

    if x {
        k.b = vector_mutate::<25, u8>(k.b);
    } else {
        k.c = vector_mutate::<25, u8>(k.c);
    }
    k
}

#[must_use] pub fn crossover(k1: Key, k2: Key) -> Key {
    let sample = GRID_LETTERS_J.to_vec();
    let mut n = k1;
    n.b = vector_crossover::<25, u8>(k1.b, k2.b, &sample);
    n.c = vector_crossover::<25, u8>(k1.c, k2.c, &sample);
    n
}
