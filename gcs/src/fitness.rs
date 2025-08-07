use std::collections::HashMap;

const TETRAGRAMS: &str = include_str!("../../tetragrams.txt");

/// Loads the tetragrams from a file of frequencies.
/// The tetragrams in this repo have been scraped from the Brown Corpus.
#[must_use]
pub fn load_tetragrams() -> (HashMap<[u8; 4], f64>, f64) {
    let raw_counts: HashMap<[u8; 4], usize> = TETRAGRAMS
        .trim()
        .lines()
        .filter_map(|line| {
            let (tetragram_str, count_str) = line.split_once(',')?;

            let tetragram = tetragram_str.trim().as_bytes().try_into().ok()?;
            let count = count_str.trim().parse().ok()?;

            Some((tetragram, count))
        })
        .collect();

    let total_count: usize = raw_counts.values().sum();

    let min_count = *raw_counts.values().min().unwrap_or(&1);
    let unseen_penalty = ((min_count as f64 / 10.0) / total_count as f64).ln();

    let log_probs = raw_counts
        .into_iter()
        .map(|(tetragram, count)| (tetragram, (count as f64 / total_count as f64).ln()))
        .collect();

    (log_probs, unseen_penalty)
}

/// Tetragram is (one of) the best fitness functions for quantifying how close a text is to
/// English. This function calculates the tetragram fitness of a text using log probabilities.
#[must_use]
pub fn tg_fitness(text: &Vec<u8>, log_probs: &HashMap<[u8; 4], f64>, unseen_penalty: f64) -> f64 {
    text.windows(4)
        .map(|window| {
            let tetragram: [u8; 4] = window.try_into().unwrap();
            *log_probs.get(&tetragram).unwrap_or(&unseen_penalty)
        })
        .sum()
}
