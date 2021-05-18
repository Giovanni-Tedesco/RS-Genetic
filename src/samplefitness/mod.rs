mod hash;
mod eq;
mod samplefitness;

use std::hash::Hash;

use std::rc::Rc;
use std::collections::BTreeMap;
use std::collections::HashMap;

struct SampleFitness<T> {
    sample: T,
    fitness: f64
}

