
extern crate bit_vec;

mod genetic;
mod algorithm;
mod utils;
mod distributions;
mod abstractions;

mod samplefitness;

pub type Chromosome = bit_vec::BitVec;

pub use genetic::Genetic;

pub use algorithm::AlgorithmParams;
pub use algorithm::genetic_algorithm;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
