use rand::Rng;
use rand::seq::SliceRandom;
use std::collections::HashSet;

pub fn decipher(ct: &Vec<u8>, key: [u8; 26]) -> Vec<u8> {
    ct.iter().map(|x| key[*x as usize - 'A' as usize]).collect()
}

pub fn initialise() -> [u8; 26] {
    let mut letters: Vec<u8> = ('A'..='Z').map(|x| x as u8).collect();
    let mut rng = rand::rng();
    letters.shuffle(&mut rng);
    letters.try_into().unwrap()
}

pub fn mutate(key: [u8; 26]) -> [u8; 26] {
    let mut n = key;
    let mut rng = rand::rng();
    let r1 = rng.random_range(0..26);
    let mut r2 = rng.random_range(0..26);
    while r2 == r1 {
        r2 = rng.random_range(0..26);
    }

    n[r1] = key[r2];
    n[r2] = key[r1];
    n
}

pub fn crossover(k1: [u8; 26], k2: [u8; 26]) -> [u8; 26] {
    let letters: Vec<u8> = ('A'..='Z').map(|x| x as u8).collect();
    let mut nums: Vec<usize> = (0..26).collect();
    let mut rng = rand::rng();
    nums.shuffle(&mut rng);

    let mut unused: HashSet<u8> = letters.iter().cloned().collect();
    let mut child = [0u8; 26];

    for &i in nums.iter().take(13) {
        child[i] = k1[i];
    }
    for &i in nums.iter().skip(13) {
        child[i] = k2[i];
    }

    let mut duplicates = Vec::new();
    let mut letters_found = HashSet::new();

    for (i, &letter) in child.iter().enumerate() {
        if letters_found.contains(&letter) {
            duplicates.push(i);
        } else {
            unused.remove(&letter);
            letters_found.insert(letter);
        }
    }

    let mut unused_letters: Vec<u8> = unused.into_iter().collect();

    for &duplicate_idx in duplicates.iter() {
        if unused_letters.is_empty() {
            break;
        }

        let r = rng.random_range(0..unused_letters.len());
        child[duplicate_idx] = unused_letters.remove(r);
    }

    child
}
