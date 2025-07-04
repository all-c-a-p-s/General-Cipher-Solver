use rand::Rng;
use rand::seq::SliceRandom;

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
