use utils::{GRID_LETTERS_J, GRID_LETTERS_Z, grid_fmt_j, grid_fmt_z, vector_initialise};

const ADFGX: [u8; 5] = [b'A', b'D', b'F', b'G', b'X'];

pub struct Key<const REMOVE_J: bool> {
    grid: [u8; 25],
    column_order: [usize; 5],
}

impl<const REMOVE_J: bool> Key<REMOVE_J> {
    pub fn new() -> Self {
        let alphabet = if REMOVE_J {
            GRID_LETTERS_J
        } else {
            GRID_LETTERS_Z
        };
        let letter_sample = alphabet.to_vec();
        let index_sample = (0..5).collect::<Vec<_>>();
        Self {
            grid: vector_initialise::<25, u8>(&letter_sample),
            column_order: vector_initialise::<5, usize>(&index_sample),
        }
    }
    pub fn encipher(&self, ct: &[u8]) -> Vec<u8> {
        let formatter = if REMOVE_J { grid_fmt_j } else { grid_fmt_z };
        let ct = formatter(ct);
        let letter_map = |x: &u8| {
            let idx = self.grid.iter().position(|c| *c == *x).unwrap();
            let (row, column) = (idx / 5, idx % 5);
            vec![ADFGX[row], ADFGX[column]]
        };

        let grid_encoding = ct.iter().map(letter_map).flatten();

        let mut columns = vec![vec![]; 5];
        for (i, c) in grid_encoding.enumerate() {
            columns[i % 5].push(c);
        }

        let mut res = vec![];
        for x in self.column_order {
            res.extend(columns[x].clone());
        }

        res
    }
}
