extern crate sbrain;
extern crate random;
use random::Source; // Trait for randomness iteration

mod randomness;

const SB_SYMBOLS: [char; 26] = ['<', '>', '-', '+', '.', ',', '[', ']',
                                '{', '}', '(', ')', 'z', '!', 's', 'S', 
                                '@', '|', '&', '*', '^', 'a', 'd', 'q', 
                                'm', 'p'];

const MUTATIONS_PER_CYCLE: usize = 3;
const STARTING_LENGTH: usize = 32;
const MAX_RUNTIME: u32 = 128;
const POPULATION_SIZE: usize = 16;

type Program = Vec<char>;
type Population = Vec<(u64, Program)>;

fn generate_random_program(length: usize) -> Program {
    let mut s = randomness::get_randomness();
    let mut r = Vec::with_capacity(length);

    s.iter()
        .take(length)
        .map(|index: usize| { r.push(SB_SYMBOLS[index % SB_SYMBOLS.len()]) })
        .count(); // .count here simply consumes the iterator so it is actually evaluated
    r
}

fn generate_population(length: usize, individuals: usize) -> Population {
    use std::u64::MAX;
    (0..individuals).map(|_| { (MAX, generate_random_program(length)) }).collect()
}

fn mutate_program(mut program: Program) -> Program {
    let mut s = randomness::get_randomness();
    for _ in 0..MUTATIONS_PER_CYCLE {
        let program_len = program.len();
        let mutation_type = s.read::<usize>() % 5; // HACK! Make enum and match
        match mutation_type {
            0|3|4 => {
                let target_index: usize = s.read::<usize>() % program.len();
                program[target_index] = SB_SYMBOLS[s.read::<usize>() % SB_SYMBOLS.len()];
            }  
            1 => { program.push(SB_SYMBOLS[s.read::<usize>() % SB_SYMBOLS.len()]); } 
            2 => { program.remove(s.read::<usize>() % program_len); }
            _ => {}
        }
    }
    program
}

fn mutate_population(population: Population) -> Vec<Program> {
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
        new_population.push(
            mutate_program(
                best_program.clone()
            )
        )
    }

    // Now the best from the old population
    pop_iter.take(empty_slots / 2).map(|prog| new_population.push(mutate_program(prog.1.clone()))).count();

    // Now fresh blood
    new_population.push(generate_random_program(STARTING_LENGTH));
    new_population
}

fn cost_program(program: &Program) -> u64 {
    let actual_output = "Hello, world!".to_string();
    let res = sbrain::fixed_evaluate(&(program.iter().collect::<String>()), Some(vec![0]), Some(100));
    let mut score = i64::abs(actual_output.len() as i64 - res.output.len() as i64) as u64 * 1024;
    for (expected, actual) in res.output.into_iter().zip(actual_output.chars()) {
        if expected != actual as u32 {
            score += i64::abs(expected as i64 - actual as i64) as u64;
        }
    }
    score
}

fn cost_population(uncosted_population: Vec<Program>) -> Population {
    uncosted_population.into_iter()
        .map(move |p| (cost_program(&p), p))
        .collect()
}

fn sort_population_by_cost(mut population: Population) -> Population {
    population.sort_by_key(|k| k.0);
    population
}

fn main() {
    let mut pop: Population = generate_population(STARTING_LENGTH, POPULATION_SIZE);
    let mut tries = 0;
    let mut last_cost = std::u64::MAX;
    loop {
        tries += 1;
        pop = cost_population(mutate_population(pop));
        pop = sort_population_by_cost(pop);

        // Report only improvements
        if pop[0].0 < last_cost {
            last_cost = pop[0].0;
            let prog = pop[0].1.iter().collect::<String>();
            println!("Generation {:5} Cost {:5}: {} \t-> {:?}", tries, last_cost, 
                prog,
                sbrain::fixed_evaluate(&prog, Some(vec![0]), Some(MAX_RUNTIME)).output);
        }

        // If the cost is zero, this generation has won!
        if pop[0].0 == 0 { break; }
    }

    let p = pop.into_iter().nth(0).unwrap();
    let res = sbrain::fixed_evaluate(&(p.1.iter().collect::<String>()), Some(vec![0]), Some(MAX_RUNTIME));
    println!("Program found after {} tries.", tries);
    println!("{}", p.1.iter().collect::<String>());
    println!("Ran for {} cycles and {} halt.\nGave: {:?}", 
        res.cycles, 
        if res.halted {"did"} else {"did not"}, 
        res.output);
    
}
