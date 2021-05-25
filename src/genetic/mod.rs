mod gen_custom;


use rand::Rng;
// use rand::distributions::{Distribution, Uniform};

use crate::{AlgorithmParams, Chromosome};


/// Produces genetic material for genetic algorithm
pub trait Genetic {

    const LENGTH: usize;

    /// Returns the gene of an object
    fn gene(&self) -> Chromosome;

    /// Returns object from gene
    fn from_gene(chromosome: &Chromosome) -> Self;

    /// Mutation
    /// Pick mutation_rate bits at random and flip them. 
    fn mutation(&self, mutation_rate: f64) -> Self where Self: Sized {

        let mut gene = self.gene();

        let mut rng = rand::thread_rng();

        // Check for iterator
        for i in 0..gene.len() {
            let number: f64 = rng.gen();

            if number < mutation_rate {
                gene.set(i, !gene.get(i).unwrap());
            }

        }

        return Genetic::from_gene(&gene);

    }


    fn mutate(&self, other: &Self, params: &AlgorithmParams) -> (Self, Self) where Self: Sized; 

    /// Crossover
    /// Gene 1 and Gene 2 assumed to be the same length
    // TODO: Check for duplicates
    fn cross_over(&self, other: &Self, co_factor: f64) -> (Self, Self) where Self: Sized {
        let mut rng = rand::thread_rng();

        let mut gene_1 = self.gene();
        let mut gene_2 = other.gene();

        for i in 0..gene_1.len() {
            let number: f64 = rng.gen();

            if number < co_factor {
                // Swap
                let temp = gene_1.get(i).unwrap();
                gene_1.set(i, gene_2.get(i).unwrap());
                gene_2.set(i, temp)
            }

        }

        //Generate return genes
        let ret1 = Genetic::from_gene(&gene_1);
        let ret2 = Genetic::from_gene(&gene_2);

        return (ret1, ret2);
    }



    // TODO: Implement a reverse function
    fn reverse(&self) -> Self where Self: Sized {
        todo!();
    }

    fn negation(&self) -> Self where Self: Sized {
        let mut gene = self.gene();

        // 1 1 0
        // 0 1 1 
        gene.negate();

        return Genetic::from_gene(&gene);
    }

    // TODO: Produce random.
    fn generate_random() -> Self;
}