use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use rand::Rng;
use rayon::prelude::*;
use std::sync::Arc;

use crate::{CIPHERTEXT, fitness};
use utils::*;

/// Applies the Genetic Algorithm to solve a cipher, given the parameters
pub fn solve<const USE_CROSSOVER: bool, T: Clone + Send + Sync>(
    initialise: Box<dyn Fn() -> T + Send + Sync>,
    crossover: Option<Box<dyn Fn(T, T) -> T + Send + Sync>>,
    mutate: Box<dyn Fn(T) -> T + Send + Sync>,
    decipher: Box<dyn Fn(&Vec<u8>, T) -> Vec<u8> + Send + Sync>,
    max_generations: usize,
    population_size: usize,
    num_children: usize,
) -> Result<String, Box<dyn std::error::Error>> {
    let ct = fmt(CIPHERTEXT);
    let (tgs, unseen_penalty) = fitness::load_tetragrams();
    let tetragrams = Arc::new(tgs);

    let initialise = Arc::new(initialise);
    let crossover = crossover.map(Arc::new);
    let mutate = Arc::new(mutate);
    let decipher = Arc::new(decipher);

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
        let new_individuals: Vec<T> = if USE_CROSSOVER {
            assert!(population_size > 1, "crossover with population size 1");
            population
                .par_iter()
                .enumerate()
                .flat_map(|(i, x)| {
                    let crossover = crossover
                        .as_ref()
                        .expect("attempt to use crossover with no crossover function");
                    let mutate = Arc::clone(&mutate);
                    let population = &population;

                    (0..num_children)
                        .into_par_iter()
                        .map(move |_| {
                            let mut rng = rand::rng();
                            let mut j = rng.random_range(0..population_size);
                            while j == i {
                                j = rng.random_range(0..population_size);
                            }
                            mutate(crossover(x.clone(), population[j].clone()))
                        })
                        .collect::<Vec<T>>()
                })
                .collect()
        } else {
            population
                .par_iter()
                .flat_map(|x| {
                    let mutate = Arc::clone(&mutate);
                    let x = x.clone();
                    (0..num_children)
                        .into_par_iter()
                        .map(move |_| mutate(x.clone()))
                        .collect::<Vec<T>>()
                })
                .collect()
        };

        let mut combined_population = population;
        combined_population.extend(new_individuals);

        let mut fitness_pairs: Vec<(T, f64)> = combined_population
            .into_par_iter()
            .map(|individual| {
                let deciphered = decipher(&ct, individual.clone());
                let fitness = fitness::tg_fitness(&deciphered, &tetragrams, unseen_penalty);
                (individual, fitness)
            })
            .collect();

        fitness_pairs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        population = fitness_pairs
            .into_iter()
            .take(population_size)
            .map(|(individual, _)| individual)
            .collect();

        pb.inc(1);
    }

    pb.finish();

    println!(
        "Final fitness score: {}",
        fitness::tg_fitness(
            &decipher(&ct, population[0].clone()),
            &tetragrams,
            unseen_penalty
        ) as f64
            / ((ct.len() - 3) as f64)
    );

    Ok(String::from_utf8(decipher(&ct, population[0].clone()))?)
}
