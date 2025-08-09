use utils::{GRID_LETTERS_J, GRID_LETTERS_Z, grid_fmt_j, grid_fmt_z, vector_initialise};

pub struct Key<const REMOVE_J: bool> {
    a: [u8; 25],
    b: [u8; 25],
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
            a: vector_initialise::<25, u8>(&sample),
            b: vector_initialise::<25, u8>(&sample),
        }
    }

    pub fn encipher(&self, pt: &[u8]) -> Vec<u8> {
        let formatter = if REMOVE_J { grid_fmt_j } else { grid_fmt_z };
        let mut pt = formatter(pt);

        if pt.len() % 2 != 0 {
            pt.push(b'X');
        }

        let coords = |idx: usize| (idx / 5, idx % 5);
        let grid_index = |row: usize, column: usize| row * 5 + column;
        let find = |grid: [u8; 25], c: u8| grid.iter().position(|&x| x == c).unwrap();

        let encipher_bigram = |bg: &[u8]| {
            let i1 = find(self.a, bg[0]);
            let i2 = find(self.b, bg[1]);

            let (r1, c1) = coords(i1);
            let (r2, c2) = coords(i2);

            let gi1 = grid_index(r1, c2);
            let gi2 = grid_index(r2, c1);

            if r1 == r2 {
                vec![bg[1], bg[0]]
            } else {
                vec![self.b[gi1], self.a[gi2]]
            }
        };

        pt.chunks(2).map(|x| encipher_bigram(x)).flatten().collect()
    }
}
