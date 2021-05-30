use crate::AlgorithmParams;


/// The idea for this is to have a general genetic structure that can
/// be used for GA problems that cannot easily be represented as
/// bits. (TSP comes to mind...)
/// The downside to this is that the genetic algorithm will have to be implemented
/// by hand. However, it can leverage the existing selection methods. Since those
/// are not reliant on the Genetic trait defined in mod.rs
pub trait GeneticCustom<Chromosome> {
    fn gene(&self) -> Chromosome;

    fn from_gene(chromosome: &Chromosome) -> Self;

    fn mutate_step(&self, other: &Self, params: &AlgorithmParams) -> (Self, Self) where Self: Sized;
}