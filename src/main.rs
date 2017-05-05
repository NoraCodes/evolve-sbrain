extern crate sbrain;
extern crate rand;
extern crate rayon;

extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate toml;

mod generate;
use generate::{generate_population, generate_random_program};

mod mutate;
use mutate::mutate_population;

mod cost;
use cost::cost_population;

mod util;
use util::sort_population_by_cost;

mod parameters;
use parameters::{read_config, Configuration};

mod types;
use types::*;

const SB_SYMBOLS: &[char] = &['<', '>', '-', '+', '.', ',', '[', ']',
                                '{', '}', '(', ')', 'z', '!', 's', 'S', 
                                '@', '|', '&', '*', '^', 'a', 'd', 'q', 
                                'm', 'p'];

const BF_SYMBOLS: &[char] = &['<', '>', '-', '+', '.', ',', '[', ']',];

fn main() {
    use std::env::args;
    let argv: Vec<_> = args().collect();

    if argv.len() < 2 {
        println!("Provide the name of a configuration file to run the evolver.");
        std::process::exit(1);
    }

    use std::path::Path;
    let path = Path::new(&argv[1]);
    let config = read_config(path);
    println!("{:?}", config);

    let mut pop: Population = generate_population(&config);
    let mut tries = 0;
    let mut last_cost = std::u64::MAX;
    loop {
        tries += 1;
        pop = cost_population(mutate_population(pop, &config), &config);
        pop = sort_population_by_cost(pop);

        // Report only improvements
        if pop[0].cost < last_cost {
            last_cost = pop[0].cost;
            let prog = pop[0].program_as_string();
            println!("Generation {:5} Cost {:5}: {}", tries, last_cost, prog);
        }

        // If the cost is zero, this generation has won!
        if pop[0].cost == 0 { break; }
    }

    let p = pop.into_iter().nth(0).unwrap();

    // Report a successful program
    println!("Program found after {} generations.", tries);
    // Report every input
    for i in 0..config.inputs.len() {
        let res = sbrain::fixed_evaluate(&(p.program_as_string()), Some(config.inputs[i].clone()), Some(config.max_runtime as u32));
        println!("Ran for {} cycles and {} halt\n{:?} -> {} -> {:?}",
        res.cycles, 
        if res.halted {"did"} else {"did not"}, 
        config.inputs[i],
        p.program_as_string(),
        res.output
        );
    }
}
