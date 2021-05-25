// use std::collections::BTreeMap;

use std::hash::Hash;
use std::collections::HashMap;
use std::rc::Rc;
// use std::cmp::Ordering;


// use rand::Rng;
use rand::distributions::{Distribution};

use crate::distributions::standard_weighted::StandardWeighted;
use crate::{abstractions::CustomDistribution};
// use crate::utils::BoltzmannParams;

use crate::distributions::boltzmann::Boltzmann;


use super::super::Genetic;


pub type GenHash<T> = HashMap<Rc<T>, f64>;

pub struct AlgorithmParams 
{
    pub rounds: usize,
    // descendents_number: u64,
    pub max_popuation: u64,
    pub mutation_rate: f64, 
    pub co_factor: f64, 
    pub elitism: usize
}



// TODO: Implement elitism
pub fn genetic_algorithm<T>(
    initial_population: &Vec<Rc<T>>,
    params: &AlgorithmParams,    
    fitness: &Box<dyn Fn(&T) -> f64>,
    cache: &mut GenHash<T>
) -> Vec<Rc<T>> 
where
T: Genetic + Copy + Eq + Hash
{
    /*
    Hash with respect to ds.
    Before calculating fitness check if it is on the hash, retrieve from hash
    otherwise, calculate the fitness then store the new one in the hash
    also store the btree.
    Once stored in the btree check what the size is and when you surpace the size just erase
    until you get to the actual size.
    if you erase from btree erase from hash

    NOTE: Try to do it with a reference count.

    Implement: f_1(sample) -> SampleFitness
    Implement: f_2() -> Borrow to btree
    Implement: f_3() -> Borrow to hash 

    Eventually change function return type to new SampleFitness.
    */


    let mut population = if initial_population.is_empty() {
        let mut x: Vec<Rc<T>> = Vec::new();
        x.push(Rc::new(Genetic::generate_random()));
        x
    } else {
        initial_population.clone()
    };

    for i in 0..params.rounds {

        let mut rng = rand::thread_rng();

        let boltzmann_params = Boltzmann {
            distribution: None,
            t_coefficient: 1f64,
            f_max: 1f64,
            generation: i as f64,
            max_generation: params.rounds as f64,
        };

        let std_weighted = StandardWeighted{
            distribution: None
        };

        // let sampler= std_weighted.new(&population, fitness, cache);
        // let dist = sampler.distribution.unwrap();


        // let dist = utils::boltzmann_selection(&population, boltzmann_params, fitness, cache);        
        let sampler = boltzmann_params.new(&population, fitness, cache);
        let dist = sampler.distribution.unwrap();


        let mut new_population: Vec<Rc<T>> = Vec::new();

        while new_population.len() < params.max_popuation as usize {
            let parent_1 = &population[dist.sample(&mut rng)];
            let parent_2 = &population[dist.sample(&mut rng)];

            // let (child_1, child_2 )= parent_1.cross_over(&parent_2, params.co_factor);

            // let mutated_child_1 = child_1.mutation(params.mutation_rate);
            // let mutated_child_2 = child_2.mutation(params.mutation_rate);
            let (mutated_child_1, mutated_child_2) = parent_1.mutate(parent_2, params);

            new_population.push(Rc::new(mutated_child_1));
            new_population.push(Rc::new(mutated_child_2));
        }

        population = new_population;

    }

    return population;

}
