use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use rand::Rng;

pub mod fitness;
pub mod mono_solver;

// aim: to solve an cipher using the genetic algorithm given:
// - ciphertext
// - random initial key state
// - mutation function
// - decipher function
// - GA parameters

const CIPHERTEXT: &str = include_str!("ciphertext.txt");

fn fmt(v: &Vec<u8>) -> Vec<u8> {
    v.iter()
        .filter(|x| ('A' as u8..='Z' as u8).contains(*x))
        .map(|x| *x)
        .collect()
}

fn solve<const USE_CROSSOVER: bool, T: Clone>(
    initialise: Box<dyn Fn() -> T>,
    crossover: Option<Box<dyn Fn(&T, &T) -> T>>,
    mutate: Box<dyn Fn(T) -> T>,
    decipher: Box<dyn Fn(&Vec<u8>, T) -> Vec<u8>>,
    max_generations: usize,
    population_size: usize,
    num_children: usize,
) -> Result<String, Box<dyn std::error::Error>> {
    let ct = fmt(&CIPHERTEXT.trim().chars().map(|x| x as u8).collect());

    let tetragrams = fitness::load_tetragrams();

    let mut population = (0..population_size)
        .map(|_| initialise())
        .collect::<Vec<T>>();

    let pb = ProgressBar::new(max_generations as u64);
    pb.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
        )
        .unwrap()
        .progress_chars("##-"),
    );

    for _ in 0..max_generations {
        let mut new = population.clone();

        for (i, x) in population.iter().enumerate() {
            if USE_CROSSOVER {
                assert!(population_size > 1);
                let mut rng = rand::rng();
                for _ in 0..num_children {
                    let mut j = rng.random_range(0..population_size);
                    while j == i {
                        j = rng.random_range(0..population_size);
                    }

                    new.push(mutate(crossover
                        .as_ref()
                        .expect("attempt to use crossover with no crossover function")(
                        x,
                        &population[j],
                    )));
                }
            } else {
                for _ in 0..num_children {
                    new.push(mutate(x.clone()))
                }
            }
        }

        population = new;

        population.sort_by(|a, b| {
            let decipher_a = decipher(&ct, a.clone());
            let decipher_b = decipher(&ct, b.clone());

            fitness::tg_fitness(&decipher_b, &tetragrams)
                .cmp(&fitness::tg_fitness(&decipher_a, &tetragrams))
        });

        population = population[0..population_size].to_vec();

        pb.inc(1);
    }

    pb.finish();

    Ok(String::from_utf8(decipher(&ct, population[0].clone()))?)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = solve::<false, [u8; 26]>(
        Box::new(mono_solver::initialise),
        None,
        Box::new(mono_solver::mutate),
        Box::new(mono_solver::decipher),
        200,
        50,
        5,
    )?;
    Ok(println!("{}", s))
}
