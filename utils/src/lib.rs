use rand::Rng;
use rand::seq::SliceRandom;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

/// u8s corresponding to 26 capital letters in the English Alphabet
pub const LETTERS: [u8; 26] = [
    65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88,
    89, 90,
];

/// In grid ciphers one letter must be removed from the alphabet (5x5 = 25).
/// This is usually J or Z - here J has been removed.
pub const GRID_LETTERS_J: [u8; 25] = [
    65, 66, 67, 68, 69, 70, 71, 72, 73, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89,
    90,
];

/// In grid ciphers one letter must be removed from the alphabet (5x5 = 25).
/// This is usually J or Z - here Z has been removed.
pub const GRID_LETTERS_Z: [u8; 25] = [
    65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88,
    89,
];

/// Formats the ciphertext, removing spaces, punctuation etc.
pub fn fmt(v: &[u8]) -> Vec<u8> {
    v.iter()
        .map(|x| x.to_ascii_uppercase())
        .filter(|&x| x.is_ascii_uppercase())
        .collect()
}

/// Formats ciphertext for grid cipher with J removed
pub fn grid_fmt_j(v: &[u8]) -> Vec<u8> {
    v.iter()
        .map(|x| x.to_ascii_uppercase())
        .filter(|&x| x.is_ascii_uppercase() && x != b'J')
        .collect()
}

/// Formats ciphertext for grid cipher with Z removed
pub fn grid_fmt_z(v: &[u8]) -> Vec<u8> {
    v.iter()
        .map(|x| x.to_ascii_uppercase())
        .filter(|&x| x.is_ascii_uppercase() && x != b'Z')
        .collect()
}

/// Apply genetic crossover to 2 genomes of type [T; N]
/// This generates a random mix of the 2 vectors genomes, with some correction to make sure that all
/// letters are present in the resulting genome, and that there are no duplicates.
pub fn vector_crossover<const N: usize, T>(k1: [T; N], k2: [T; N], sample: &[T]) -> [T; N]
where
    T: Clone + Hash + Eq + Default + Copy,
{
    let mut rng = rand::rng();

    let mut indices: Vec<usize> = (0..N).collect();
    indices.shuffle(&mut rng);

    let mut child = [T::default(); N];

    for i in 0..N / 2 {
        child[indices[i]] = k1[indices[i]];
    }

    for i in N / 2..N {
        child[indices[i]] = k2[indices[i]];
    }

    let mut unused: HashSet<T> = sample.iter().copied().collect();
    let mut duplicates = Vec::new();
    let mut elems_found = HashSet::new();

    for (i, &elem) in child.iter().enumerate() {
        if elems_found.contains(&elem) {
            duplicates.push(i);
        } else {
            unused.remove(&elem);
        }
        elems_found.insert(elem);
    }

    let mut unused_elems: Vec<T> = unused.into_iter().collect();

    let mut duplicate_idx = 0;
    while !unused_elems.is_empty() && duplicate_idx < duplicates.len() {
        let r = if unused_elems.len() > 1 {
            rng.random_range(0..unused_elems.len())
        } else {
            0
        };

        child[duplicates[duplicate_idx]] = unused_elems[r];
        unused_elems.remove(r);
        duplicate_idx += 1;
    }

    child
}

/// This applies one random mutation to a genome of type [T; N]
/// by swapping a pair or elements
pub fn vector_mutate<const N: usize, T>(mut k: [T; N]) -> [T; N] {
    let mut rng = rand::rng();
    let r1 = rng.random_range(0..N);
    let mut r2 = rng.random_range(0..N);
    while r2 == r1 {
        r2 = rng.random_range(0..N);
    }

    k.swap(r1, r2);
    k
}

/// Initialises a random genome of type [T; N] with random state
pub fn vector_initialise<const N: usize, T>(sample: &[T]) -> [T; N]
where
    T: Clone + Debug,
{
    let mut v = sample.to_vec();
    let mut rng = rand::rng();
    v.shuffle(&mut rng);
    v.try_into().unwrap()
}

pub fn vector_sample<const N: usize, T>(sample: &[T]) -> [T; N]
where
    T: Clone + Debug,
{
    let mut rng = rand::rng();
    (0..N)
        .map(|_| sample[rng.random_range(0..sample.len())].clone())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}
