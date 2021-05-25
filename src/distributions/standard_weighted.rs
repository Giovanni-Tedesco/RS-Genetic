use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::rc::Rc;

use rand::distributions::WeightedIndex;
use rand::prelude::Distribution;
use crate::abstractions::CustomDistribution;

use std::hash::Hash;

use crate::abstractions::FitFunc;

use crate::algorithm::algorithm::GenHash;


pub struct StandardWeighted {
    pub distribution: Option<WeightedIndex<f64>>,
}

impl<T> CustomDistribution<T> for StandardWeighted 

where
    T: Eq + Hash
{
    fn sample(&self) -> usize {
        let mut rng = rand::thread_rng();
        self.distribution.as_ref().unwrap().sample(&mut rng)
    }

    fn new(&self, population: &Vec<std::rc::Rc<T>>, fitness: &FitFunc<T>
            , cache: &mut GenHash<T>) -> Self {
        let dist = WeightedIndex::new(
            population.into_iter()
            .map(|item| StandardWeighted::calc_fitness(item, fitness, cache))
        ).unwrap();

        return StandardWeighted {
            distribution: Some(dist)
        }
    }
}

impl StandardWeighted {

    pub fn calc_fitness<T>(
        item: &Rc<T>,
        fitness: &Box<dyn Fn(&T) -> f64>,
        cache: &mut HashMap<Rc<T>, f64>
    ) -> f64
    where
        T: Hash + Eq
    {
        match cache.entry(item.clone()) {
            Entry::Vacant(entry) => *entry.insert(fitness(item)),
            Entry::Occupied(entry) => *entry.get()
        }

    }

}
