// use std::{collections::BTreeMap};

// use std::hash::Hash;
// use std::cmp::Ordering;
// use std::collections::HashMap;
// use std::collections::hash_map::Entry;


// // use rand::Rng;
// use rand::distributions::WeightedIndex;

// use std::rc::Rc;


// use super::super::Genetic;


// pub fn sample_to_distribution<T>(
//     population: &Vec<Rc<T>>,
//     fitness: &Box<dyn Fn(&T) -> f64>,
//     cache: &mut HashMap<Rc<T>, f64>

// ) -> WeightedIndex<f64> 
// where
//     T: Genetic + Hash + Eq
// {

//     WeightedIndex::new(
//         population
//             .iter()
//             .map(|item| calc_fitness(item, fitness, cache))
//     ).unwrap()

// }

// pub fn calc_fitness<T>(
//     item: &Rc<T>,
//     fitness: &Box<dyn Fn(&T) -> f64>,
//     cache: &mut HashMap<Rc<T>, f64>
// ) -> f64
// where
//     T: Genetic + Hash + Eq
// {
//     match cache.entry(item.clone()) {
//         Entry::Vacant(entry) => *entry.insert(fitness(item)),
//         Entry::Occupied(entry) => *entry.get()
//     }

// }

// #[derive(Clone, Copy, Debug)]
// pub struct BoltzmannParams {
//     pub t_coefficient: f64,
//     pub f_max: f64,
//     pub generation: f64,
//     pub max_generation: f64,
// }

