use std::{collections::BTreeMap};

use std::hash::Hash;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::hash_map::Entry;


use rand::Rng;
use rand::distributions::WeightedIndex;

use std::rc::Rc;


use super::Genetic;


pub fn sample_to_distribution<T>(
    population: &Vec<Rc<T>>,
    fitness: &Box<dyn Fn(&T) -> f64>,
    cache: &mut HashMap<Rc<T>, f64>

) -> WeightedIndex<f64> 
where
    T: Genetic + Hash + Eq
{

    WeightedIndex::new(
        population
            .iter()
            .map(|item| calc_fitness(item, fitness, cache))
    ).unwrap()

}

pub fn calc_fitness<T>(
    item: &Rc<T>,
    fitness: &Box<dyn Fn(&T) -> f64>,
    cache: &mut HashMap<Rc<T>, f64>
) -> f64
where
    T: Genetic + Hash + Eq
{
    match cache.entry(item.clone()) {
        Entry::Vacant(entry) => *entry.insert(fitness(item)),
        Entry::Occupied(entry) => *entry.get()
    }

}

#[derive(Clone, Copy, Debug)]
pub struct BoltzmannParams {
    pub t_coefficient: f64,
    pub f_max: f64,
    pub generation: f64,
    pub max_generation: f64,
}

pub fn boltzmann_selection<T> (
    population: &Vec<Rc<T>>,
    params: BoltzmannParams,
    fitness: &Box<dyn Fn(&T) -> f64>,
    cache: &mut HashMap<Rc<T>, f64>,

) -> WeightedIndex<f64> 
where
    T: Genetic + Hash + Eq
{
    WeightedIndex::new(
        population
        .iter()
        .map(|item| boltzmann_fitnesses(item, params, fitness, cache))
    ).unwrap()

}

// TODO:
pub fn boltzmann_fitnesses<T> (
    item: &Rc<T>,
    params: BoltzmannParams,
    fitness: &Box<dyn Fn(&T) -> f64>,
    cache: &mut HashMap<Rc<T>, f64>,
) -> f64 
where
    T: Genetic + Hash + Eq

{
    match cache.entry(item.clone()) {
        Entry::Vacant(entry) => *entry.insert(boltzmann_probability(item, params, fitness)),
        Entry::Occupied(entry) => *entry.get()
    }
}

// TODO:
pub fn boltzmann_probability<T>(x: &Rc<T>, 
    params: BoltzmannParams,
    fitness: &Box<dyn Fn(&T) -> f64>,

) -> f64 {

    f64::exp(-(fitness(x)/get_t_boltzmann(params)))

}


fn get_t_boltzmann(params: BoltzmannParams) -> f64 {

    (params.t_coefficient).powf((1f64 + 100f64*params.generation)/params.max_generation)

}

