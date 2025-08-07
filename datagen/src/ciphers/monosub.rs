use utils::{LETTERS, vector_initialise};

#[derive(Clone)]
pub struct Key {
    letter_map: [u8; 26],
}

impl Default for Key {
    fn default() -> Self {
        Self::new()
    }
}

impl Key {
    #[must_use]
    pub fn new() -> Self {
        let sample = LETTERS.to_vec();
        Self {
            letter_map: vector_initialise::<26, u8>(&sample),
        }
    }

    #[must_use]
    pub fn encipher(self, pt: &[u8]) -> Vec<u8> {
        pt.iter()
            .map(|x| self.letter_map[*x as usize - 'A' as usize])
            .collect()
    }
}
