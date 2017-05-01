use super::randomness::get_randomness;
use super::{Program, Population};
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

pub fn generate_population(length: usize, individuals: usize) -> Population {
    use std::u64::MAX;
    (0..individuals).into_par_iter()
        .map(|_| { (MAX, generate_random_program(length)) })
        .collect()
}