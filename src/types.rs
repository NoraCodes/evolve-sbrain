pub type Program = Vec<char>;
pub type Cost = u64;
pub type Population = Vec<ProgramWithCost>;
pub type UncostedPopulation = Vec<Program>;

#[derive(Debug, Clone)]
pub struct ProgramWithCost{
    pub cost: Cost,
    pub program: Program
}

impl ProgramWithCost {
    pub fn new(cost: Cost, program: Program) -> Self {
        ProgramWithCost {cost: cost, program: program}
    }
    pub fn program_as_string(&self) -> String {
        self.program.iter().collect()
    }
}