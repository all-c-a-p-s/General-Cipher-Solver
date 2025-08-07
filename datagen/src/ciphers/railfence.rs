use rand::Rng;

pub struct Key<const N: usize> {
    shift: usize,
}

impl<const N: usize> Key<N> {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        Self {
            shift: rng.random_range(0..2 * N - 1),
        }
    }

    pub fn encipher(self, pt: &[u8]) -> Vec<u8> {
        let mut rows = vec![vec![]; N];
        let mut down = true;
        let mut next = |n: usize| {
            if down && n == N - 1 {
                if N == 1 {
                    0
                } else {
                    down = false;
                    n - 1
                }
            } else if down {
                n + 1
            } else if n == 0 {
                down = true;
                1
            } else {
                n - 1
            }
        };

        let mut current_row = 0;
        for _ in 0..self.shift {
            current_row = next(current_row);
        }

        for x in pt {
            rows[current_row].push(x);
            current_row = next(current_row);
        }

        rows.into_iter().flatten().copied().collect()
    }
}
