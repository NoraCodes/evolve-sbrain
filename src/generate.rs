use super::randomness::get_randomness;
use super::{Program, Population, Configuration, Arc};
use super::SB_SYMBOLS;

use random::Source;
use rayon::prelude::*;

pub fn generate_random_program(length: usize) -> Program {
    let mut s = get_randomness();
    let mut r = Vec::with_capacity(length);

    s.iter()
        .take(length)
        .map(|index: usize| { r.push(SB_SYMBOLS[index % SB_SYMBOLS.len()]) })
        .count(); // .count here simply consumes the iterator so it is actually evaluated
    r
}

pub fn generate_population(cfg: Arc<Configuration>) -> Population {
    use std::u64::MAX;
    (0..cfg.population_size).into_par_iter()
        .map(|_| { (MAX, generate_random_program(cfg.initial_program_length)) })
        .collect()
}