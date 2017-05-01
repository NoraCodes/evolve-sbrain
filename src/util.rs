use super::Population;

pub fn sort_population_by_cost(mut population: Population) -> Population {
    population.sort_by_key(|k| k.0);
    population
}