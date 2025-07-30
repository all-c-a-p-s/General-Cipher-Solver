pub mod fitness;
pub mod ga_solver;
pub mod mono_solver;

// aim: to solve an cipher using the genetic algorithm given:
// - ciphertext
// - random initial key state
// - mutation function
// - decipher function
// - GA parameters

const MAX_GENERATIONS: usize = 100;
const POPULATION_SIZE: usize = 100;
const NUM_CHILDREN: usize = 20;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = ga_solver::solve::<true, [u8; 26]>(
        Box::new(mono_solver::initialise),
        Some(Box::new(mono_solver::crossover)),
        Box::new(mono_solver::mutate),
        Box::new(mono_solver::decipher),
        MAX_GENERATIONS,
        POPULATION_SIZE,
        NUM_CHILDREN,
    )?;
    Ok(println!("{}", s))
}
