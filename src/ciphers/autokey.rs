use utils::{LETTERS, vector_sample};

pub struct Key<const N: usize> {
    initial_key: [u8; N],
}

impl<const N: usize> Key<N> {
    pub fn new() -> Self {
        let sample = (0..26).collect::<Vec<_>>();
        Self {
            initial_key: vector_sample::<N, u8>(&sample),
        }
    }

    pub fn encipher(&self, pt: &[u8]) -> Vec<u8> {
        let add_shift = |x: u8, s: u8| LETTERS[((x - b'A' + s) % 26) as usize];
        let mut shifts = self.initial_key.to_vec();
        shifts.extend(pt);

        pt.iter()
            .enumerate()
            .map(|(i, x)| add_shift(*x, shifts[i]))
            .collect()
    }
}
