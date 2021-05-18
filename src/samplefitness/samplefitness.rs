use super::SampleFitness;

use std::{collections::HashMap, hash::Hash};
use std::rc::Rc;
use std::collections::BTreeMap;

impl<T> SampleFitness<T>
where
    T: Hash + Eq + Clone
{
    // TODO: Do these functions
    fn func_1(sample: Vec<Rc<T>>) -> SampleFitness<T> {
        todo!();
    }

    fn func_2(&self) -> BTreeMap<Rc<T>, f64> {
        todo!();
    }

    fn func_3(&self) -> HashMap<Rc<T>, f64> {
        todo!();
    }

    fn insert_to_hash() {
        todo!();
    }

    fn delete_from_hash() {
        todo!();
    }
}