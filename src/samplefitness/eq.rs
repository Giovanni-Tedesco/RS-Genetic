use super::SampleFitness;

use std::cmp::Ordering;

impl<T> PartialOrd for SampleFitness<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.fitness.partial_cmp(&other.fitness)
    }
}

impl<T> PartialEq for SampleFitness<T> {
    
    fn eq(&self, other: &Self) -> bool {
        self.fitness.eq(&other.fitness)
    }

}