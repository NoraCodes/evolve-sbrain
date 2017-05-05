use sbrain;
use rayon::prelude::*;
use super::{Program, Population, UncostedPopulation, Configuration};

const LENGTH_WEIGHT: u64 = 1024;

fn cost_program(program: &Program, cfg: &Configuration) -> u64 {
    let mut cost = 0;
    for i in 0..cfg.inputs.len() {
        cost += cost_single_target(program, &cfg.inputs[i], &cfg.targets[i]);
    }
    cost
}

fn cost_single_target(program: &Program, input: &[u32], target: &[u32]) -> u64 {
    let res = sbrain::fixed_evaluate(&(program.iter().collect::<String>()), Some(Vec::from(input)), Some(100));

    // Score for length
    let mut score = 
        if target.len() > res.output.len() {target.len() - res.output.len()}
        else {res.output.len() - target.len()} as u64 * LENGTH_WEIGHT;

    for (expected, actual) in res.output.into_iter().zip(target.into_iter()) {
        if expected != *actual as u32 {
            score += i64::abs(expected as i64 - *actual as i64) as u64;
        }
    }
    score
}

pub fn cost_population(uncosted_population: UncostedPopulation, cfg: &Configuration) -> Population {
    uncosted_population.into_par_iter()
        .map(move |p| (cost_program(&p, cfg), p))
        .collect()
}