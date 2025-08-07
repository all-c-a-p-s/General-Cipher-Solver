use rand::Rng;
use utils::*;

pub mod ciphers;

const CIPHERTEXT_LENGTH: usize = 1500;

fn get_plaintext() -> Vec<u8> {
    let corpus = include_bytes!("../brown_corpus.txt");
    let mut rng = rand::rng();

    let start_idx = rng.random_range(0..corpus.len() - CIPHERTEXT_LENGTH);
    let window = &corpus[start_idx..start_idx + CIPHERTEXT_LENGTH];
    fmt(window)
    //this way, depending on the exact number of spaces / digits in the ciphertext, we
    //will get slight variation in ciphertext length
}

fn main() {
    let pt = get_plaintext();
    let s = pt
        .clone()
        .into_iter()
        .map(|x| x as char)
        .collect::<String>();
    let ct = ciphers::railfence::Key::<1>::new().encipher(&pt);
    let s2 = ct.into_iter().map(|x| x as char).collect::<String>();
    println!("{}", s);
    println!("Length was: {}", s.len());

    println!("{}", s2);
}
