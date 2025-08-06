pub mod columnar;
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
    let s = ga_solver::solve::<true, columnar::Key<7>>(
        Box::new(columnar::Key::new),
        Some(Box::new(columnar::crossover::<7>)),
        Box::new(columnar::mutate::<7>),
        Box::new(columnar::decipher_rc::<7>),
        MAX_GENERATIONS,
        POPULATION_SIZE,
        NUM_CHILDREN,
    )?;
    Ok(println!("{}", s))
}
