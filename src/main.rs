#![feature(slice_concat_ext)]

extern crate sbrain;
extern crate random;
extern crate rayon;

extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate toml;

use std::sync::Arc;

mod randomness;

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

type Program = Vec<char>;
type Population = Vec<(u64, Program)>;
type UncostedPopulation = Vec<Program>;

const SB_SYMBOLS: [char; 26] = ['<', '>', '-', '+', '.', ',', '[', ']',
                                '{', '}', '(', ')', 'z', '!', 's', 'S', 
                                '@', '|', '&', '*', '^', 'a', 'd', 'q', 
                                'm', 'p'];

fn main() {
    use std::env::args;
    let argv: Vec<_> = args().collect();

    if argv.len() < 2 {
        println!("Provide the name of a configuration file to run the evolver.");
        std::process::exit(1);
    }

    use std::path::Path;
    let path = Path::new(&argv[1]);
    let config = Arc::new(read_config(path));
    println!("{:?}", config);

    let mut pop: Population = generate_population(config.clone());
    let mut tries = 0;
    let mut last_cost = std::u64::MAX;
    loop {
        tries += 1;
        pop = cost_population(mutate_population(pop, config.clone()), config.clone());
        pop = sort_population_by_cost(pop);

        // Report only improvements
        if pop[0].0 < last_cost {
            last_cost = pop[0].0;
            let prog = pop[0].1.iter().collect::<String>();
            println!("Generation {:5} Cost {:5}: {}", tries, last_cost, prog);
        }

        // If the cost is zero, this generation has won!
        if pop[0].0 == 0 { break; }
    }

    let p = pop.into_iter().nth(0).unwrap();

    println!("Program found after {} tries.", tries);
    println!("{}", p.1.iter().collect::<String>());
    for i in 0..config.inputs.len() {
        let res = sbrain::fixed_evaluate(&(p.1.iter().collect::<String>()), Some(config.inputs[i].clone()), Some(config.max_runtime as u32));
        println!("Ran for {} cycles and {} halt\n{:?} -> {} -> {:?}",
        res.cycles, 
        if res.halted {"did"} else {"did not"}, 
        config.inputs[i],
        p.1.iter().collect::<String>(),
        res.output
        );
    }
}
