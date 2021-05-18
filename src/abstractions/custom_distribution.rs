use std::rc::Rc;
use crate::algorithm::GenHash;

pub struct Params<T> {
    t_coefficient: f64,
    sample: Vec<Rc<T>>,
    cache: GenHash<T>
}
// Makes a template to create custom distributions
// that conform to some standard
pub trait CustomDistribution<T> {

    // Samples from the distribution
    fn sample(&self) -> f64; 

    // Generate a new distribution
    fn new(params: Params<T>) -> Self;

    // The probability density function
    fn pdf() -> f64;
    // Generate a sample of n fitnesses.
    // fn sample_n(&self) -> Vec<f64>;
}