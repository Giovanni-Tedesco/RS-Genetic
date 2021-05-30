use std::{collections::{HashMap, hash_map::Entry}, rc::Rc};

use rand::{distributions::WeightedIndex, prelude::Distribution};

use crate::{abstractions::CustomDistribution};

use crate::algorithm::algorithm::GenHash;

use crate::abstractions::FitFunc;
use std::hash::Hash;

// use crate::Chromosome;
// #[derive(Clone, Copy, Debug)]
pub struct Boltzmann {
    pub distribution: Option<WeightedIndex<f64>>,
    pub t_coefficient: f64,
    pub f_max: f64,
    pub generation: f64,
    pub max_generation: f64
}

impl<T> CustomDistribution<T> for Boltzmann 
where
    T: Hash + Eq + Clone 

{

    fn sample(&self) -> usize {
        let mut rng = rand::thread_rng();

        self.distribution.as_ref().unwrap().sample(&mut rng)
    }

    fn new(&self, population: &Vec<std::rc::Rc<T>>, fitness: &FitFunc<T>
            , cache: &mut GenHash<T>) -> Self {

        
        let distribution = self.boltzmann_selection(population, fitness, cache);

        return Boltzmann {
            distribution: Option::Some(distribution),
            t_coefficient: self.t_coefficient,
            f_max: self.f_max,
            generation: self.generation,
            max_generation: self.max_generation,
        }
        
    }

}

impl Boltzmann {
    fn boltzmann_selection<T> (
        &self,
        population: &Vec<Rc<T>>,
        fitness: &Box<dyn Fn(&T) -> f64>,
        cache: &mut GenHash<T>,
    ) -> WeightedIndex<f64> 
    where
        T: Hash + Eq
    {
        WeightedIndex::new(
            population
            .iter()
            .map(|item| self.boltzmann_fitnesses(item, fitness, cache))
        ).unwrap()

    }

    pub fn boltzmann_fitnesses<T> (
        &self,
        item: &Rc<T>,
        fitness: &Box<dyn Fn(&T) -> f64>,
        cache: &mut HashMap<Rc<T>, f64>,
    ) -> f64 
    where
        T: Hash + Eq

    {
        match cache.entry(item.clone()) {
            Entry::Vacant(entry) => *entry.insert(self.boltzmann_probability(item, fitness)),
            Entry::Occupied(entry) => *entry.get()
        }
    }

    fn boltzmann_probability<T> (
        &self,
        x: &Rc<T>, 
        fitness: &Box<dyn Fn(&T) -> f64>,

    ) -> f64 {

        f64::exp(-((self.f_max - fitness(x))/self.get_t_boltzmann()))

    }

    fn get_t_boltzmann(&self) -> f64 {

        (self.t_coefficient).powf((1f64 + 100f64*self.generation)/self.max_generation)

    }

}

