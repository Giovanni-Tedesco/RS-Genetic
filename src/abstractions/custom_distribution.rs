use std::rc::Rc;
use crate::algorithm::algorithm::GenHash;

use crate::abstractions::FitFunc;

// Makes a template to create custom distributions
// that conform to some standard
pub trait CustomDistribution<T> {
    // Samples from the distribution
    fn sample(&self) -> usize; 

    // Generate a new distribution
    fn new(&self, population: &Vec<Rc<T>>, fitness: &FitFunc<T>
            , cache: &mut GenHash<T>) -> Self;

    // The probability density function
    // Generate a sample of n fitnesses.
    // fn sample_n(&self) -> Vec<f64>;
}