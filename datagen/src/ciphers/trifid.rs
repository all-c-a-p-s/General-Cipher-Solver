use utils::{LETTERS, vector_initialise};

pub struct Key<const N: usize> {
    cube: [u8; 27],
}

impl<const N: usize> Key<N> {
    pub fn new() -> Self {
        let mut sample = LETTERS.to_vec();
        sample.push(b'#');
        Self {
            cube: vector_initialise::<27, u8>(&sample),
        }
    }

    pub fn encipher(&self, pt: &[u8]) -> Vec<u8> {
        let mut pt = pt.to_vec();
        while pt.len() % N != 0 {
            pt.push(b'#');
        }

        let find = |c: u8, cube: [u8; 27]| {
            let pos = cube.iter().position(|x| *x == c).unwrap();
            let (a, b, c) = (pos / 9, (pos % 9) / 3, pos % 3);
            [a, b, c]
        };

        let grid_index = |pos: [usize; 3]| pos[0] * 9 + pos[1] * 3 + pos[2];

        let mut chunk_index = 0;
        let mut chunks = vec![vec![vec![]; 3]];
        for x in pt {
            let [a, b, c] = find(x, self.cube);
            chunks[chunk_index][0].push(a);
            chunks[chunk_index][1].push(b);
            chunks[chunk_index][2].push(c);
            if chunks[chunk_index][0].len() == N {
                chunks.push(vec![vec![]; 3]);
                chunk_index += 1;
            }
        }

        let map_chunk = |chunk: &Vec<Vec<usize>>| {
            let flat = chunk.iter().flatten().copied().collect::<Vec<_>>();
            flat.chunks(3)
                .map(|v| self.cube[grid_index(v.try_into().unwrap())])
                .collect::<Vec<_>>()
        };

        chunks.iter().map(map_chunk).flatten().collect()
    }
}
