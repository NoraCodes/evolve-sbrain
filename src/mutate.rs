use super::{Program, Population, UncostedPopulation, Configuration};
use super::rand;
use super::generate_random_program;
use super::{SB_SYMBOLS, BF_SYMBOLS};

use rand::Rng;

pub fn mutate_program(mut program: Program, cfg: &Configuration) -> Program {
    let mut s = rand::thread_rng();

    // Loop through exactly the given number of mutations.
    for _ in 0..cfg.mutations_per_generation {
        let mut program_len = program.len();

        // Mutating an empty program means just making a new program.
        if program_len == 0 {
            program = generate_random_program(cfg);
            program_len = cfg.initial_program_length;
        }

        // Select the right set of symbols.
        let symbols = if cfg.is_legacy() { BF_SYMBOLS } else { SB_SYMBOLS };


        // Select the type of mutation to perform.
        let mutation_type = s.gen_range(0, 3); 
        match mutation_type {
            0 => {
                let target_index: usize = s.gen_range(0, program.len());
                program[target_index] = *s.choose(symbols).unwrap();
            }  
            1 => { program.insert(s.gen_range(0, program_len), *s.choose(symbols).unwrap()); } 
            2 => { program.remove(s.gen_range(0, program_len)); }
            _ => {unreachable!()}
        }
    }
    program
}

pub fn mutate_population(population: Population, cfg: &Configuration) -> UncostedPopulation {
    // Reserve one for the best and four for fresh blood
    let empty_slots = population.len() - 5;
    // Create buffer and iterator
    let mut new_population = Vec::with_capacity(population.len());
    let best_program = population[0].clone().program;
    if !cfg.is_free_mut() {
        // Preserve the best program, so no reverse progress happens
        new_population.push(best_program.clone());
    }

    // Mutate the best to fill half the new population
    for individual in population.iter().take(empty_slots / 2) {
        new_population.push(
            // Mutate the best and one of the top 50%, make them have kids.
            cross_programs(
                mutate_program(best_program.clone(), cfg),
                mutate_program(individual.program.clone(), cfg)
            ).1
        )
    }

    let mut pop_iter = population.into_iter();
    // Skip the first program
    pop_iter.next();
    // Now the best from the old population
    pop_iter.take(empty_slots / 2).map(
        |prog| new_population.push(mutate_program(prog.program, cfg))
    ).count();

    // Now fresh blood
    for _ in 0..4 {
        new_population.push(generate_random_program(cfg));
    }
    // Free_mut additional population
    if cfg.is_free_mut() { new_population.push(generate_random_program(cfg)); }
    new_population
}

fn cross_programs(mut a: Program, mut b: Program) -> (Program, Program) {
    use std::cmp::min;
    use std::mem;
    let min_length = min(a.len(), b.len());

    // Can't cross programs that are too short.
    // Not having this protections causes div-by-zero when computing bounds!
    if min_length < 2 { return (a,b); }
    
    let mut rand = rand::thread_rng();
    // Generate a section to pull and replace
    let upper_bound: usize = rand.gen_range(0, min_length - 1) + 1;
    let lower_bound: usize = rand.gen_range(0, upper_bound);

    assert!(upper_bound < min_length, "Upper bound is greater than the minimum length");

    // Run through the section, swapping values
    for i in lower_bound..upper_bound {
        mem::swap(&mut a[i], &mut b[i])
    }

    (a, b)
}
