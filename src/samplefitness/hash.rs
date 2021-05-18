use super::SampleFitness;

use std::hash::Hash;
use std::collections::HashMap;

impl<T> Hash for SampleFitness<T>
where
T: Hash 
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.sample.hash(state);
    }
}

