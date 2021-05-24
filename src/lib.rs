extern crate bit_vec;

mod genetic;
mod algorithm;
mod distributions;
mod abstractions;

mod samplefitness;

pub type Chromosome = bit_vec::BitVec;

pub use genetic::Genetic;

pub use crate::algorithm::algorithm::AlgorithmParams;
pub use crate::algorithm::algorithm::genetic_algorithm;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
