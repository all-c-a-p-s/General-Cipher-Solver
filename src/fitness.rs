use std::collections::HashMap;

const TETRAGRAMS: &str = include_str!("../tetragrams.txt");

pub fn load_tetragrams() -> HashMap<[u8; 4], usize> {
    TETRAGRAMS
        .trim()
        .lines()
        .map(|x| {
            let split = x.split(",").collect::<Vec<&str>>();
            (
                split[0]
                    .trim()
                    .chars()
                    .map(|x| x as u8)
                    .collect::<Vec<u8>>()
                    .try_into()
                    .expect("wrong number of character before comma"),
                split[1]
                    .trim()
                    .parse::<usize>()
                    .expect("failed to parse to integer"),
            )
        })
        .collect()
}

pub fn tg_fitness(text: &Vec<u8>, m: &HashMap<[u8; 4], usize>) -> i32 {
    text.windows(4)
        .fold(0, |acc, x| acc + *m.get(x).unwrap_or(&0) as i32)
}
