use crate::abstractions::CustomDistribution;
use crate::algorithm::GenHash;
use crate::Genetic;

pub struct Boltzmann<T> {

    t_coefficient: f64,
    sample: Vec<f64>,
    cache: GenHash<T>

}

