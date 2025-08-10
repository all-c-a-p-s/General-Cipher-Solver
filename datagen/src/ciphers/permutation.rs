use utils::vector_initialise;

pub struct Key<const N: usize> {
    order: [usize; N],
}

impl<const N: usize> Key<N> {
    pub fn new() -> Self {
        let sample = (0..N).collect::<Vec<_>>();
        Self {
            order: vector_initialise::<N, usize>(&sample),
        }
    }

    pub fn encipher(&self, pt: &[u8]) -> Vec<u8> {
        let mut pt = pt.to_vec();
        while pt.len() % N != 0 {
            pt.push(b'X');
        }
        let map_n_gram = |ng: [u8; N]| {
            let mut res = [0u8; N];
            for (i, &x) in self.order.iter().enumerate() {
                res[x] = ng[i];
            }
            res
        };

        pt.chunks(N)
            .map(|x| map_n_gram(x.try_into().unwrap()))
            .flatten()
            .collect()
    }
}
