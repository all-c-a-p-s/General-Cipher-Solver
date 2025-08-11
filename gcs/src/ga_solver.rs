use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use rand::Rng;
use rayon::prelude::*;
use std::collections::HashMap;

use crate::{CIPHERTEXT, fitness};
use utils::*;

/// Chooses a partner for the 'i'th individual
/// All numbers in the range 0..n-1 are mapped to a partner
fn pick_partner(i: usize, n: usize) -> usize {
    let mut rng = rand::rng();
    let j = rng.random_range(0..n - 1);
    if j >= i { j + 1 } else { j }
}

/// Takes the 'n' fittest individuals from a population, ready for the next generation
fn take_fittest<T>(
    population: &[T],
    n: usize,
    decipher: &Box<dyn Fn(&Vec<u8>, T) -> Vec<u8> + Send + Sync>,
    tetragrams: &HashMap<[u8; 4], f64>,
    unseen_penalty: f64,
    ct: &Vec<u8>,
) -> Vec<T>
where
    T: Send + Sync + Clone,
{
    // pair each key with fitness so we only have to compute once
    let mut fitness_pairs: Vec<(T, f64)> = population
        .into_par_iter()
        .map(|individual| {
            let deciphered = decipher(ct, individual.clone());
            let fitness = fitness::tg_fitness(&deciphered, &tetragrams, unseen_penalty);
            (individual.clone(), fitness)
        })
        .collect();

    fitness_pairs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    // take fittest individuals in population
    fitness_pairs
        .into_iter()
        .take(n)
        .map(|(individual, _)| individual)
        .collect()
}

/// Applies the Genetic Algorithm to solve a cipher, given the parameters
pub fn solve<const USE_CROSSOVER: bool, T>(
    initialise: Box<dyn Fn() -> T + Send + Sync>,
    crossover: Option<Box<dyn Fn(T, T) -> T + Send + Sync>>,
    mutate: Box<dyn Fn(T) -> T + Send + Sync>,
    decipher: Box<dyn Fn(&Vec<u8>, T) -> Vec<u8> + Send + Sync>,
    max_generations: usize,
    population_size: usize,
    num_children: usize,
) -> Result<String, Box<dyn std::error::Error>>
where
    T: Clone + Send + Sync,
{
    assert!(
        !(USE_CROSSOVER && population_size < 2),
        "attempt to use crossover with population size 1 or less"
    );

    let ct = fmt(CIPHERTEXT);
    let (tetragrams, unseen_penalty) = fitness::load_tetragrams();

    let mut population = (0..population_size)
        .into_par_iter()
        .map(|_| initialise())
        .collect::<Vec<T>>();

    let pb = ProgressBar::new(max_generations as u64);
    pb.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg} [{eta_precise}]",
        )
        .unwrap()
        .progress_chars("##-"),
    );

    for _ in 0..max_generations {
        let create_child = |parent: &T, parent_idx: usize| {
            if USE_CROSSOVER {
                let crossover_fn = crossover
                    .as_ref()
                    .expect("crossover enabled but no crossover function provided");
                let partner_idx = pick_partner(parent_idx, population_size);
                let partner = &population[partner_idx];
                mutate(crossover_fn(parent.clone(), partner.clone()))
            } else {
                mutate(parent.clone())
            }
        };

        let new_individuals: Vec<T> = population
            .par_iter()
            .enumerate()
            .flat_map(|(i, parent)| {
                (0..num_children)
                    .into_par_iter()
                    .map(|_| create_child(parent, i))
                    .collect::<Vec<T>>()
            })
            .collect();

        let mut combined_population = population.clone();
        combined_population.extend(new_individuals);

        population = take_fittest(
            &combined_population,
            population_size,
            &decipher,
            &tetragrams,
            unseen_penalty,
            &ct,
        );

        pb.inc(1);
    }

    pb.finish();

    let pt = decipher(&ct, population[0].clone());

    println!(
        "Final fitness score: {}",
        fitness::tg_fitness(&pt, &tetragrams, unseen_penalty) as f64 / ((ct.len() - 3) as f64)
    );

    Ok(String::from_utf8(pt)?)
}
