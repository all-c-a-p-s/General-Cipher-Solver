use crate::ciphers;
use crate::features::get_all_features;
use rand::Rng;

use utils::*;

macro_rules! combos {
    ($pt:expr; $($key_len:literal),*) => {
        {
            let mut v = Vec::new();
            $(
                let t = process_plaintext::<true, $key_len>($pt);
                let f = process_plaintext::<false, $key_len>($pt);
                v.push(t);
                v.push(f);
            )*
            v
        }
    };
}

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

fn process_plaintext<const REMOVE_J: bool, const KEY_LENGTH: usize>(
    pt: &[u8],
) -> [(String, [f32; 47]); 13] {
    let labels = [
        "adfgx",
        "autokey",
        "bifid",
        "columnar",
        "foursquare",
        "hill",
        "monosub",
        "nihilist",
        "playfair",
        "polybius",
        "railfence",
        "twosquare",
        "vigenere",
    ];

    let mut cts = vec![vec![]; 13];
    cts[0] = ciphers::adfgx::Key::<REMOVE_J>::new().encipher(pt);
    cts[1] = ciphers::autokey::Key::<KEY_LENGTH>::new().encipher(pt);
    cts[2] = ciphers::bifid::Key::<REMOVE_J>::new().encipher(pt);
    cts[3] = ciphers::columnar::Key::<KEY_LENGTH>::new().encipher(pt);
    cts[4] = ciphers::foursquare::Key::<REMOVE_J>::new().encipher(pt);
    cts[5] = ciphers::hill::Key::<KEY_LENGTH>::new().encipher(pt);
    cts[6] = ciphers::monosub::Key::new().encipher(pt);
    cts[7] = ciphers::nihilist::Key::<REMOVE_J, KEY_LENGTH>::new().encipher(pt);
    cts[8] = ciphers::playfair::Key::<REMOVE_J>::new().encipher(pt);
    cts[9] = ciphers::polybius::Key::<REMOVE_J>::new().encipher(pt);
    cts[10] = ciphers::railfence::Key::<KEY_LENGTH>::new().encipher(pt);
    cts[11] = ciphers::twosquare::Key::<REMOVE_J>::new().encipher(pt);
    cts[12] = ciphers::vigenere::Key::<KEY_LENGTH>::new().encipher(pt);

    let unlabelled = cts.iter().map(|x| get_all_features(&x)).collect::<Vec<_>>();

    let labelled = unlabelled
        .iter()
        .enumerate()
        .map(|(i, &x)| (labels[i].to_string(), x))
        .collect::<Vec<_>>();
    labelled.try_into().unwrap()
}

fn gen_from(pt: &[u8]) -> Vec<[(String, [f32; 47]); 13]> {
    combos!(pt; 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15)
}

pub fn generate_once() -> Vec<[(String, [f32; 47]); 13]> {
    let pt = get_plaintext();
    gen_from(&pt)
}
