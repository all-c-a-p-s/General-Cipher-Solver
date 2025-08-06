use crate::utils::*;

// Note: for a columnar transposition cipher, work will still have to be done to find the key
// length.

#[derive(Clone)]
pub struct Key<const COLUMNS: usize> {
    pub column_order: [usize; COLUMNS],
}

impl<const COLUMNS: usize> Key<COLUMNS> {
    pub fn new() -> Key<{ COLUMNS }> {
        let sample: Vec<usize> = (0..COLUMNS).into_iter().collect();

        Self {
            column_order: vector_initialise::<COLUMNS, usize>(&sample),
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

pub fn mutate<const COLUMNS: usize>(k: Key<COLUMNS>) -> Key<COLUMNS> {
    Key {
        column_order: vector_mutate::<COLUMNS, usize>(k.column_order),
    }
}

pub fn crossover<const COLUMNS: usize>(k1: Key<COLUMNS>, k2: Key<COLUMNS>) -> Key<COLUMNS> {
    let sample: Vec<usize> = (0..COLUMNS).into_iter().collect();
    Key {
        column_order: vector_crossover::<COLUMNS, usize>(k1.column_order, k2.column_order, &sample),
    }
}
