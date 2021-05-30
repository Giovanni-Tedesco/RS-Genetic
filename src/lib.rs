extern crate bit_vec;

pub mod genetic;
pub mod algorithm;
pub mod distributions;
pub mod abstractions;

mod samplefitness;

pub type Chromosome = bit_vec::BitVec;

pub use genetic::Genetic;

pub use genetic::gen_custom::GeneticCustom;

pub use crate::algorithm::algorithm::AlgorithmParams;
pub use crate::algorithm::algorithm::genetic_algorithm;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
