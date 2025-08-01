pub mod fitness;
pub mod four_square;
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
    let s = ga_solver::solve::<true, four_square::Key>(
        Box::new(four_square::Key::new),
        Some(Box::new(four_square::crossover)),
        Box::new(four_square::mutate),
        Box::new(four_square::decipher),
        MAX_GENERATIONS,
        POPULATION_SIZE,
        NUM_CHILDREN,
    )?;
    Ok(println!("{}", s))
}
