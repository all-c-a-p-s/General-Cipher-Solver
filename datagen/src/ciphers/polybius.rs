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
        let pt = formatter(pt);

        let coords = |idx: usize| (idx / 5, idx % 5);
        let find = |grid: [u8; 25], c: u8| grid.iter().position(|&x| x == c).unwrap();
        let to_dec = |(row, col): (usize, usize)| row * 10 + col;

        let strung: Vec<String> = pt
            .iter()
            .map(|x| {
                let idx = find(self.grid, *x);
                let cs = coords(idx);
                format!("{}", to_dec(cs))
            })
            .collect();

        strung
            .iter()
            .fold(String::new(), |acc, x| acc + x)
            .bytes()
            .collect()
    }
}
