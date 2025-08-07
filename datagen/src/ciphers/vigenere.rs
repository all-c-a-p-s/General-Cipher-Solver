use utils::{LETTERS, vector_sample};

pub struct Key<const N: usize> {
    shifts: [u8; N],
}

impl<const N: usize> Key<N> {
    pub fn new() -> Self {
        let sample = (0..26).collect::<Vec<_>>();
        Self {
            shifts: vector_sample::<N, u8>(&sample),
        }
    }

    pub fn encipher(self, pt: &[u8]) -> Vec<u8> {
        pt.iter()
            .enumerate()
            .map(|(i, &x)| LETTERS[((x - b'A' + self.shifts[i % N]) % 26) as usize])
            .collect()
    }
}
