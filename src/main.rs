extern crate sbrain;
extern crate random;
use random::Source; // Trait for randomness iteration

mod randomness;

const SB_SYMBOLS: [char; 26] = ['<', '>', '-', '+', '.', ',', '[', ']', 
                               '{', '}', '(', ')', 'z', '!', 's', 'S', 
                               '@', '|', '&', '*', '^', 'a', 'd', 'q', 
                               'm', 'p'];

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
    let target_index: usize = s.read::<usize>() % program.len();
    program[target_index] = SB_SYMBOLS[s.read::<usize>() % SB_SYMBOLS.len()];
    program
}

fn mutate_population(population: Population) -> Vec<Program> {
    // Preserve the best program, so no reverse progress happens
    let best = population[0].1.clone();
    // Length for the next-generated program.
    let len = best.len();

    // Mutate all the programs except the best and worst
    let mut mutated: Vec<Program> = population[1..population.len()-1]
            .into_iter()
            .map(move |p| { mutate_program(p.1.clone()) } )
            .collect();
    

    let mut new: Vec<Program> = vec![best];
    // Add in the mutated population
    new.append(&mut mutated);
    // Add one single totally new program
    new.push(generate_random_program(len));

    new
}

fn cost_program(program: &Program) -> u64 {
    let actual_output = "Hello, world!".to_string();
    let res = sbrain::fixed_evaluate(&(program.iter().collect::<String>()), Some(vec![1,2,3,4,5]), Some(100));
    let mut score = i64::abs(actual_output.len() as i64 - res.output.len() as i64) as u64 * 128;
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
    let mut pop: Population = generate_population(128, 16);
    let mut tries = 0;
    let mut last_cost = std::u64::MAX;
    loop {
        tries += 1;
        pop = cost_population(mutate_population(pop));
        pop = sort_population_by_cost(pop);
        if pop[0].0 < last_cost {
            last_cost = pop[0].0;
            let prog = pop[0].1.iter().collect::<String>();
            println!("Generation {:5} Cost {:5}: {} \t-> {:?}", tries, last_cost, 
                prog,
                sbrain::fixed_evaluate(&prog, Some(vec![1,2,3,4,5]), Some(100)).output);
        }
        if pop[0].0 == 0 { break; }
    }

    let p = pop.into_iter().nth(0).unwrap();
    let res = sbrain::fixed_evaluate(&(p.1.iter().collect::<String>()), Some(vec![1,2,3,4,5]), Some(100));
    println!("Program found after {} tries.", tries);
    println!("{}", p.1.iter().collect::<String>());
    println!("Ran for {} cycles and {} halt.\nGave: {:?}", 
        res.cycles, 
        if res.halted {"did"} else {"did not"}, 
        res.output);
    
}
