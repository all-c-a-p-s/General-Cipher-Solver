use utils::{GRID_LETTERS_J, GRID_LETTERS_Z, grid_fmt_j, grid_fmt_z, vector_initialise};

pub struct Key<const REMOVE_J: bool> {
    grid: [u8; 25],
}

impl<const REMOVE_J: bool> Key<REMOVE_J> {
    const ALPHABET: [u8; 25] = if REMOVE_J {
        GRID_LETTERS_J
    } else {
        GRID_LETTERS_Z
    };
    pub fn new() -> Self {
        let sample = Self::ALPHABET.to_vec();
        Self {
            grid: vector_initialise::<25, u8>(&sample),
        }
    }

    pub fn encipher(&self, pt: &[u8]) -> Vec<u8> {
        let formatter = if REMOVE_J { grid_fmt_j } else { grid_fmt_z };
        let pt = formatter(pt);
        let letter_map = |x: &u8| {
            let idx = self.grid.iter().position(|c| *c == *x).unwrap();
            (idx / 5, idx % 5)
        };

        let coords = pt.iter().map(letter_map);

        let mut new_coords = vec![vec![]; 2];

        for (a, b) in coords {
            new_coords[0].push(a);
            new_coords[1].push(b);
        }

        let indices = new_coords.iter().flatten().collect::<Vec<_>>();
        let index_map = |(row, col): (usize, usize)| self.grid[row * 5 + col];

        let mut res = vec![];
        for pair in indices.chunks(2) {
            let cs = (*pair[0], *pair[1]);
            res.push(index_map(cs));
        }

        res
    }
}
