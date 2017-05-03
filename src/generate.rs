use super::randomness::get_randomness;
use super::{Program, Population, Configuration, Arc};
use super::{SB_SYMBOLS, BF_SYMBOLS};

use random::Source;
use rayon::prelude::*;

pub fn generate_random_program(cfg: Arc<Configuration>) -> Program {
    let mut s = get_randomness();
    let mut r = Vec::with_capacity(cfg.initial_program_length);

    // Determine whether to use BrainFuck symbols or SBrain symbols.
    // These aliases are here to satisfy the borrow checker.
    let bf_symbols = &BF_SYMBOLS[..];
    let sb_symbols = &SB_SYMBOLS[..];
    let symbols = if cfg.is_legacy() { bf_symbols } else { sb_symbols };

    s.iter()
        .take(cfg.initial_program_length)
        .map(|index: usize| { r.push(symbols[index % symbols.len()]) })
        .count(); // .count here simply consumes the iterator so it is actually evaluated
    r
}

pub fn generate_population(cfg: Arc<Configuration>) -> Population {
    use std::u64::MAX;
    (0..cfg.population_size).into_par_iter()
        .map(|_| { (MAX, generate_random_program(cfg.clone())) })
        .collect()
}