use super::{Program, Population, UncostedPopulation, Configuration, Arc};
use super::randomness::get_randomness;
use super::generate_random_program;
use super::{SB_SYMBOLS, BF_SYMBOLS};

use random::Source;

pub fn mutate_program(mut program: Program, cfg: Arc<Configuration>) -> Program {
    let mut s = get_randomness();

    // Loop through exactly the given number of mutations.
    for _ in 0..cfg.mutations_per_generation {
        let mut program_len = program.len();

        // Mutating an empty program means just making a new program.
        if program_len == 0 {
            program = generate_random_program(cfg.clone());
            program_len = cfg.initial_program_length;
        }

        // Select the right set of symbols.
        // These aliases are here to satisfy the borrow checker.
        let bf_symbols = &BF_SYMBOLS[..];
        let sb_symbols = &SB_SYMBOLS[..];
        let symbols = if cfg.is_legacy() { bf_symbols } else { sb_symbols };

        let mutation_type = s.read::<usize>() % 3; // HACK! Make enum and match
        match mutation_type {
            0 => {
                let target_index: usize = s.read::<usize>() % program.len();
                program[target_index] = symbols[s.read::<usize>() % symbols.len()];
            }  
            1 => { program.insert(s.read::<usize>() % program_len, symbols[s.read::<usize>() % symbols.len()]); } 
            2 => { program.remove(s.read::<usize>() % program_len); }
            _ => {}
        }
    }
    program
}

pub fn mutate_population(population: Population, cfg: Arc<Configuration>) -> UncostedPopulation {
    // Reserve one for the best and one for fresh blood
    let empty_slots = population.len() - 2;
    // Create buffer and iterator
    let mut new_population = Vec::with_capacity(population.len());
    // Preserve the best program, so no reverse progress happens
    new_population.push(population[0].clone().1);
    let best_program = new_population[0].clone();

    // Mutate the best to fill half the new population
    for old_program_to_cross_with in 0..(empty_slots / 2) {
        new_population.push(
            // Mutate the best and one of the top 50%, make them have kids.
            cross_programs(
                mutate_program(best_program.clone(), cfg.clone()),
                mutate_program(population[old_program_to_cross_with].1.clone(), cfg.clone())
            ).1
        )
    }

    let mut pop_iter = population.into_iter();
    // Skip the first program
    pop_iter.next();
    // Now the best from the old population
    pop_iter.take(empty_slots / 2).map(
        |prog| new_population.push(mutate_program(prog.1.clone(), cfg.clone()))
    ).count();

    // Now fresh blood
    new_population.push(generate_random_program(cfg.clone()));
    new_population
}

fn cross_programs(a: Program, b: Program) -> (Program, Program) {
    use std::cmp::min;
    use std::slice::SliceConcatExt;
    let min_length = min(a.len(), b.len());

    // Can't cross programs that are too short.
    // Not having this protections causes div-by-zero when computing bounds!
    if min_length < 2 { return (a,b); }
    
    let mut rand = get_randomness();
    // Generate a section to pull and replace
    let upper_bound: usize = (rand.read::<usize>() % (min_length - 1)) + 1;
    let lower_bound: usize = rand.read::<usize>() % (upper_bound);

    assert!(upper_bound < min_length, "Upper bound is greater than the minimum length");

    let a_section = &a[lower_bound..upper_bound];
    let b_section = &b[lower_bound..upper_bound];

    let new_a = [&a[0..lower_bound], b_section, &a[upper_bound..a.len()]].concat();
    let new_b = [&b[0..lower_bound], a_section, &b[upper_bound..b.len()]].concat();

    (new_a, new_b)
}