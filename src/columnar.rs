use rand::Rng;
use rand::seq::SliceRandom;
use std::collections::HashSet;

// Note: for a columnar transposition cipher, work will still have to be done to find the key
// length.

#[derive(Clone)]
pub struct Key<const COLUMNS: usize> {
    pub column_order: [usize; COLUMNS],
}

impl<const COLUMNS: usize> Key<COLUMNS> {
    pub fn new() -> Key<{ COLUMNS }> {
        let mut ls: Vec<usize> = (0..COLUMNS).into_iter().collect();

        let mut rng = rand::rng();
        ls.shuffle(&mut rng);
        Self {
            column_order: ls.try_into().unwrap(),
        }
    }
}

/// To decipher a text read by rows, written by columns
pub fn decipher_rc<const COLUMNS: usize>(ciphertext: &Vec<u8>, k: Key<COLUMNS>) -> Vec<u8> {
    let row_count = ciphertext.len() / COLUMNS;
    let mut rows = vec![];
    for r in ciphertext.chunks(row_count) {
        rows.push(r);
    }

    let mut res = vec![];
    for i in 0..ciphertext.len() {
        res.push(rows[k.column_order[i % COLUMNS]][i / COLUMNS]);
    }

    res
}

/// To decipher a text written by columns, written by rows
pub fn decipher_cr<const COLUMNS: usize>(ciphertext: &Vec<u8>, k: Key<COLUMNS>) -> Vec<u8> {
    let mut columns = vec![vec![]; COLUMNS];
    for (i, &c) in ciphertext.iter().enumerate() {
        columns[i % COLUMNS].push(c);
    }

    let mut sorted_columns = vec![vec![]; COLUMNS];
    for (i, &c) in k.column_order.iter().enumerate() {
        sorted_columns[c] = columns[i].to_vec();
    }

    sorted_columns.into_iter().flatten().collect()
}

pub fn mutate<const COLUMNS: usize>(mut k: Key<COLUMNS>) -> Key<COLUMNS> {
    let mut rng = rand::rng();
    let r1 = rng.random_range(0..COLUMNS);
    let mut r2 = rng.random_range(0..COLUMNS);
    while r2 == r1 {
        r2 = rng.random_range(0..COLUMNS);
    }

    k.column_order.swap(r1, r2);

    k
}

pub fn crossover<const COLUMNS: usize>(k1: Key<COLUMNS>, k2: Key<COLUMNS>) -> Key<COLUMNS> {
    let mut nums: Vec<usize> = (0..COLUMNS).collect();
    let mut rng = rand::rng();
    nums.shuffle(&mut rng);

    let mut unused: HashSet<usize> = nums.iter().cloned().collect();
    let mut child = [0; COLUMNS];

    for &i in nums.iter().take(COLUMNS / 2) {
        child[i] = k1.column_order[i];
    }
    for &i in nums.iter().skip(COLUMNS - COLUMNS / 2) {
        child[i] = k2.column_order[i];
    }

    let mut duplicates = Vec::new();
    let mut letters_found = HashSet::new();

    for (i, &letter) in child.iter().enumerate() {
        if letters_found.contains(&letter) {
            duplicates.push(i);
        } else {
            unused.remove(&letter);
            letters_found.insert(letter);
        }
    }

    let mut unused_nums: Vec<usize> = unused.into_iter().collect();

    for &duplicate_idx in duplicates.iter() {
        if unused_nums.is_empty() {
            break;
        }

        let r = rng.random_range(0..unused_nums.len());
        child[duplicate_idx] = unused_nums.remove(r);
    }

    Key {
        column_order: child,
    }
}
