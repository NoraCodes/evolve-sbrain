extern crate sbrain;
extern crate random;
use random::Source; // Trait for randomness iteration
use std::char::from_u32;

mod randomness;

const SB_SYMBOLS: [char; 26] = ['<', '>', '-', '+', '.', ',', '[', ']', 
                               '{', '}', '(', ')', 'z', '!', 's', 'S', 
                               '@', '|', '&', '*', '^', 'a', 'd', 'q', 
                               'm', 'p'];

fn generate_random_program(length: usize) -> String {
    let mut s = randomness::get_randomness();
    let mut r = String::with_capacity(length);

    s.iter()
        .take(length)
        .map(|index: usize| { r.push(SB_SYMBOLS[index % SB_SYMBOLS.len()]) })
        .count(); // .count here simply consumes the iterator so it is actually evaluated
    r
}

fn main() {
    
    let mut p;
    let mut res;
    let mut tries = 0;
    loop {
        tries += 1;
        p = generate_random_program(32);
        res = sbrain::fixed_evaluate(&p, Some(vec![1,2,3,4,5]), Some(1000));
        if res.output.len() > 0 { break; }
    }
    println!("Program with output found after {} tries.", tries);
    println!("{}", p);
    println!("Ran for {} cycles and {} halt.\nGave: {:?}", 
        res.cycles, 
        if res.halted {"did"} else {"did not"}, 
        res.output);
    
}
