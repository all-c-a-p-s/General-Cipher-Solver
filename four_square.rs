use rand::Rng;
use rand::seq::SliceRandom;
use std::collections::HashSet;

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

const LETTERS: [u8; 25] = [
    65, 66, 67, 68, 69, 70, 71, 72, 73, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89,
    90,
]; //excludes J

impl Key {
    pub fn new() -> Self {
        Self {
            a: LETTERS,
            b: randomise(),
            c: randomise(),
            d: LETTERS,
        }
    }
}

fn randomise() -> [u8; 25] {
    let mut ls: Vec<u8> = LETTERS.into_iter().collect();

    let mut rng = rand::rng();
    ls.shuffle(&mut rng);
    ls.try_into().unwrap()
}

fn coordinates(grid_index: usize) -> Coords {
    let row = grid_index / 5;
    let column = grid_index - row * 5;
    Coords { row, column }
}

fn grid_index(row: usize, column: usize) -> usize {
    row * 5 + column
}

pub fn decipher(text: &Vec<u8>, k: Key) -> Vec<u8> {
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
        deciphered.extend(decipher_bigram(bg))
    }

    deciphered
}

pub fn mutate(mut k: Key) -> Key {
    //randomly swaps two elements in one of the two grids
    let mut rng = rand::rng();
    let x = rng.random_bool(0.5); //rng choosing which grid to mutate
    let mut swap = |g: &mut [u8; 25]| {
        let i1 = rng.random_range(0..25);
        let mut i2 = rng.random_range(0..25);
        while i1 == i2 {
            i2 = rng.random_range(0..25);
        }

        (g[i1], g[i2]) = (g[i2], g[i1]);
    };

    if x {
        swap(&mut k.b);
    } else {
        swap(&mut k.c);
    }
    k
}

fn crossover_grids(k1: [u8; 25], k2: [u8; 25]) -> [u8; 25] {
    let mut rng = rand::rng();

    let mut nums: Vec<usize> = (0..25).collect();
    nums.shuffle(&mut rng);

    let mut unused: HashSet<u8> = LETTERS.into_iter().collect();

    let mut child = [0u8; 25];

    for i in 0..13 {
        child[nums[i]] = k1[nums[i]];
    }

    for i in 13..25 {
        child[nums[i]] = k2[nums[i]];
    }

    let mut duplicates = Vec::new();
    let mut letters_found = HashSet::new();

    for i in 0..child.len() {
        if letters_found.contains(&child[i]) {
            duplicates.push(i);
        } else {
            unused.remove(&child[i]);
        }
        letters_found.insert(child[i]);
    }

    let mut unused_letters: Vec<u8> = unused.into_iter().collect();

    let mut duplicate_idx = 0;
    while !unused_letters.is_empty() && duplicate_idx < duplicates.len() {
        let r = if unused_letters.len() > 1 {
            rng.random_range(0..unused_letters.len())
        } else {
            0
        };

        child[duplicates[duplicate_idx]] = unused_letters[r];
        unused_letters.remove(r);
        duplicate_idx += 1;
    }

    child
}

pub fn crossover(k1: Key, k2: Key) -> Key {
    let mut n = k1;
    n.b = crossover_grids(k1.b, k2.b);
    n.c = crossover_grids(k1.c, k2.c);
    n
}
