use utils::vector_sample;

pub struct Key<const N: usize> {
    rows: [[u8; N]; N],
}

fn modular_dot<const N: usize>(a: [u8; N], b: [u8; N]) -> u8 {
    let conv = |v: [u8; N]| -> [u16; N] {
        v.iter()
            .map(|x| *x as u16)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    };
    let (a, b) = (conv(a), conv(b)); // because 25 * 25 > u8::MAX
    (a.into_iter().zip(b).fold(0, |acc, (x, y)| acc + x * y) % 26) as u8
}

fn modular_vector_multiply<const N: usize>(v: [u8; N], mat: [[u8; N]; N]) -> [u8; N] {
    mat.iter()
        .map(|u| modular_dot::<N>(*u, v))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

impl<const N: usize> Key<N> {
    pub fn new() -> Self {
        let sample = (0..26).collect::<Vec<_>>();
        let rows = (0..N).map(|_| vector_sample::<N, u8>(&sample));
        let rows: Vec<[u8; N]> = rows.map(|x| x.try_into().unwrap()).collect();
        Self {
            rows: rows.try_into().unwrap(),
        }
    }

    pub fn encipher(self, pt: &[u8]) -> Vec<u8> {
        let mut pt = pt.to_vec();
        while pt.len() % N != 0 {
            pt.push(b'X');
        }

        pt = pt.iter().map(|x| x - b'A').collect();

        let vectors: Vec<[u8; N]> = pt.chunks(N).map(|x| x.try_into().unwrap()).collect();
        vectors
            .iter()
            .map(|v| modular_vector_multiply(*v, self.rows))
            .map(|x| x.to_vec())
            .flatten()
            .map(|x| x + b'A')
            .collect()
    }
}
