use super::{Program, Population, UncostedPopulation};

use sbrain;
use rayon::prelude::*;

const LENGTH_WEIGHT: u64 = 1024;

fn cost_program(program: &Program) -> u64 {
    //let expected_output = "Hello".to_string();
    let expected_output = vec![1, 2, 3, 4, 5];
    let res = sbrain::fixed_evaluate(&(program.iter().collect::<String>()), Some(vec![0]), Some(100));

    // Score for length
    let mut score = 
        if expected_output.len() > res.output.len() {expected_output.len() - res.output.len()}
        else {res.output.len() - expected_output.len()} as u64 * LENGTH_WEIGHT;

    for (expected, actual) in res.output.into_iter().zip(expected_output.into_iter()) {
        if expected != actual as u32 {
            score += i64::abs(expected as i64 - actual as i64) as u64;
        }
    }
    score
}

pub fn cost_population(uncosted_population: UncostedPopulation) -> Population {
    uncosted_population.into_par_iter()
        .map(move |p| (cost_program(&p), p))
        .collect()
}