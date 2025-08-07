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

    pub fn encipher(self, pt: &[u8]) -> Vec<u8> {
        let formatter = if REMOVE_J { grid_fmt_j } else { grid_fmt_z };
        let mut pt = formatter(pt);

        let mut fixed = vec![];
        for w in pt.windows(2) {
            let x = if w[0] == w[1] {
                let null = if w[0] == b'X' {
                    //double x should be unlikely but you never know
                    b'Z'
                } else {
                    b'X'
                };
                vec![w[0], null, w[1]]
            } else {
                w.to_vec()
            };
            fixed.extend(x);
        }

        if pt.len() % 2 != 0 {
            pt.push(b'X');
        }

        let coords = |idx: usize| (idx / 5, idx % 5);
        let grid_index = |row: usize, column: usize| row * 5 + column;
        let find = |grid: [u8; 25], c: u8| grid.iter().position(|&x| x == c).unwrap();

        let encipher_bigram = |bg: &[u8]| {
            let i1 = find(self.grid, bg[0]);
            let i2 = find(self.grid, bg[1]);

            let (r1, c1) = coords(i1);
            let (r2, c2) = coords(i2);

            let mut gi1 = grid_index(r1, c2);
            let mut gi2 = grid_index(r2, c1);

            if c1 == c2 {
                gi1 = (i1 + 5) % 25;
                gi2 = (i2 + 5) % 25;
            } else if r1 == r2 {
                gi1 = if gi1 % 5 == 4 { gi1 - 4 } else { gi1 + 1 };
                gi2 = if gi2 % 5 == 4 { gi2 - 4 } else { gi2 + 1 };
            }

            vec![self.grid[gi1], self.grid[gi2]]
        };

        pt.chunks(2).map(|x| encipher_bigram(x)).flatten().collect()
    }
}
