use super::{Program, Population, UncostedPopulation, Configuration, Arc};
use super::randomness::get_randomness;
use super::generate_random_program;
use super::SB_SYMBOLS;

use random::Source;

pub fn mutate_program(mut program: Program, cfg: Arc<Configuration>) -> Program {
    let mut s = get_randomness();
    for _ in 0..s.read::<usize>() % cfg.mutations_per_generation {
        let mut program_len = program.len();
        if program_len == 0 {
            program = generate_random_program(cfg.initial_program_length);
            program_len = cfg.initial_program_length;
        }
        let mutation_type = s.read::<usize>() % 3; // HACK! Make enum and match
        match mutation_type {
            0 => {
                let target_index: usize = s.read::<usize>() % program.len();
                program[target_index] = SB_SYMBOLS[s.read::<usize>() % SB_SYMBOLS.len()];
            }  
            1 => { program.insert(s.read::<usize>() % program_len, SB_SYMBOLS[s.read::<usize>() % SB_SYMBOLS.len()]); } 
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
    let mut pop_iter = population.into_iter();
    // Preserve the best program, so no reverse progress happens
    new_population.push(pop_iter.next().unwrap().1);
    // Mutate the best to fill half the new population
    let best_program = new_population[0].clone();
    for _ in 0..(empty_slots / 2) {
        new_population.push(mutate_program(best_program.clone(), cfg.clone()))
    }

    // Now the best from the old population
    pop_iter.take(empty_slots / 2).map(
        |prog| new_population.push(mutate_program(prog.1.clone(), cfg.clone()))
    ).count();

    // Now fresh blood
    new_population.push(generate_random_program(cfg.initial_program_length));
    new_population
}