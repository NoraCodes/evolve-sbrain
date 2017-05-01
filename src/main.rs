extern crate sbrain;
extern crate random;
extern crate rayon;

mod randomness;

mod generate;
use generate::{generate_population, generate_random_program};

mod mutate;
use mutate::mutate_population;

mod cost;
use cost::cost_population;

mod util;
use util::sort_population_by_cost;

const MUTATIONS_PER_CYCLE: usize = 20;
const STARTING_LENGTH: usize = 8;
const MAX_RUNTIME: u32 = 64;
const POPULATION_SIZE: usize = 16;

type Program = Vec<char>;
type Population = Vec<(u64, Program)>;
type UncostedPopulation = Vec<Program>;

const SB_SYMBOLS: [char; 26] = ['<', '>', '-', '+', '.', ',', '[', ']',
                                '{', '}', '(', ')', 'z', '!', 's', 'S', 
                                '@', '|', '&', '*', '^', 'a', 'd', 'q', 
                                'm', 'p'];

fn main() {
    let mut pop: Population = generate_population(STARTING_LENGTH, POPULATION_SIZE);
    let mut tries = 0;
    let mut last_cost = std::u64::MAX;
    loop {
        tries += 1;
        pop = cost_population(mutate_population(pop, MUTATIONS_PER_CYCLE, STARTING_LENGTH));
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
