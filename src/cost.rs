use sbrain;
use rayon::prelude::*;
use super::{Program, ProgramWithCost, Population, UncostedPopulation, Configuration, Cost};
use std::cmp::{min, max};

const LENGTH_WEIGHT: Cost = 1024;
const UNIT_COST: Cost = 1;

fn cost_program(program: &Program, cfg: &Configuration) -> u64 {
    let mut cost = 0;
    for i in 0..cfg.inputs.len() {
        cost += cost_single_target(program, &cfg.inputs[i], &cfg.targets[i]);
    }
    cost
}

fn cost_single_target(program: &Program, input: &[u32], target: &[u32]) -> u64 {
    // Evaluate the program
    let program_source = program.iter().collect::<String>();
    let res = sbrain::fixed_evaluate(&program_source, Some(Vec::from(input)), Some(100));

    let mut cost;

    // Score for length
    let max_length = max(target.len(), res.output.len()) as Cost;
    let min_length = min(target.len(), res.output.len()) as Cost;
    cost = (max_length - min_length) * LENGTH_WEIGHT * UNIT_COST;

    // Score for value
    for (a, b) in res.output.into_iter().zip(target.into_iter()) {
        let max_value = max(a, *b) as Cost;
        let min_value = min(a, *b) as Cost;
        cost += max_value - min_value * UNIT_COST;
    }

    cost
}

pub fn cost_population(uncosted_population: UncostedPopulation, cfg: &Configuration) -> Population {
    uncosted_population.into_par_iter()
        .map(move |p| ProgramWithCost::new(cost_program(&p, cfg), p))
        .collect()
}