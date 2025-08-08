use utils::{
    GRID_LETTERS_J, GRID_LETTERS_Z, grid_fmt_j, grid_fmt_z, vector_initialise, vector_sample,
};

pub struct Key<const REMOVE_J: bool, const N: usize> {
    grid: [u8; 25],
    shift: [u8; N],
}

impl<const REMOVE_J: bool, const N: usize> Key<REMOVE_J, N> {
    const ALPHABET: [u8; 25] = if REMOVE_J {
        GRID_LETTERS_J
    } else {
        GRID_LETTERS_Z
    };
    pub fn new() -> Self {
        let sample = Self::ALPHABET.to_vec();
        let shift_sample = (0..25).collect::<Vec<_>>();
        Self {
            grid: vector_initialise::<25, u8>(&sample),
            shift: vector_sample::<N, u8>(&shift_sample),
        }
    }

    pub fn encipher(self, pt: &[u8]) -> Vec<u8> {
        let formatter = if REMOVE_J { grid_fmt_j } else { grid_fmt_z };
        let pt = formatter(pt);

        let coords = |idx: usize| (idx / 5, idx % 5);
        let find = |grid: [u8; 25], c: u8| grid.iter().position(|&x| x == c).unwrap();
        let to_dec = |(row, col): (usize, usize)| row * 10 + col;

        let shifter = self
            .shift
            .iter()
            .map(|x| {
                let idx = find(self.grid, Self::ALPHABET[*x as usize]);
                let cs = coords(idx);
                to_dec(cs) as u8
            })
            .collect::<Vec<_>>();

        let letter_map = |l: u8, shift: u8| {
            let idx = find(self.grid, l);
            let cs = coords(idx);
            let dec = to_dec(cs);

            dec as u8 + shift
        };
        pt.iter()
            .enumerate()
            .map(|(i, x)| letter_map(*x, shifter[i % N]))
            .fold(String::new(), |acc, x| acc + format!("{x}").as_str())
            .bytes()
            .collect()
    }
}
