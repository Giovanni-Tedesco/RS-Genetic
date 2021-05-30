use genetic::*;
use std::{convert::TryInto};
// use rand::Rng;

use std::hash::Hash;

use std::collections::HashMap;
use std::rc::Rc;


#[derive(Copy, Clone, Debug)]
pub struct GraphRepresentation {
    x: f64
}

impl Genetic for GraphRepresentation {
    const LENGTH: usize = 64;

    fn gene(&self) -> Chromosome {
        let big_i = if self.x < 0f64 {
            0u16
        } else if self.x < 1f64 {
            (self.x * (u16::MAX as f64)) as u16
        }  else {
            1u16
        };

        let bits = big_i.to_be_bytes();

        return Chromosome::from_bytes(&bits);
    }

    fn from_gene(chromosome: &Chromosome) -> Self {
        let temp = chromosome.to_bytes();
        let t: Result<[u8; 2], _> = temp.try_into();

        let new_x = u16::from_be_bytes(t.unwrap()) as f64 / (u16::MAX as f64);


        return GraphRepresentation{x: new_x};
    }

    fn generate_random() -> Self {
        // let mut rng = rand::thread_rng();
        GraphRepresentation{x: 0.2}
    }

    fn mutate_step(&self, other: &Self, params: &AlgorithmParams) -> (Self, Self) where Self: Sized {

        let (child_1, child_2 )= self.cross_over(other, params.co_factor);

        let mutated_child_1 = child_1.mutation(params.mutation_rate);
        let mutated_child_2 = child_2.mutation(params.mutation_rate);

        return (mutated_child_1, mutated_child_2);
    }
}

impl Hash for GraphRepresentation {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.gene().hash(state);
    }
}

impl PartialEq for GraphRepresentation {
    fn eq(&self, other: &Self) -> bool {
        self.gene() == other.gene()
    }
}

impl Eq for GraphRepresentation {}



pub fn fitness(gene: &GraphRepresentation) -> f64 {
    if gene.x > 1.0 || gene.x < 0.0 {
        return 0.001
    } else {
        return -(gene.x - 0.5).powf(2.0) + 0.36
        // return gene.x.powf(2.0)
    }
}


fn main() {

    let x = GraphRepresentation{
        x: 0.02
    };

    let gene = x.gene();
    let recovered = GraphRepresentation::from_gene(&gene);

    println!("{:?}", x.clone());
    println!("{:?}", gene.clone());
    println!("{:?}", recovered);

    let params: AlgorithmParams = AlgorithmParams{
        rounds: 20,
        max_popuation: 10,
        mutation_rate: 0.05,
        co_factor: 0.05,
        elitism: 2
    };


    let mut cache: HashMap<Rc<GraphRepresentation>, f64> = HashMap::new();
    let v: Vec<Rc<GraphRepresentation>> = Vec::new();
    let fitness_func: Box<dyn Fn(&GraphRepresentation) -> f64> = Box::new(fitness);

    let mut ret = genetic_algorithm(&v, &params, &fitness_func, &mut cache);

    ret.sort_by(|a, b| fitness(a).partial_cmp(&fitness(b)).unwrap());
    
    // println!("{:?}", ret);
    for item in ret {
        println!("value: {:?}, fitness {:?}", item.x, fitness(&item))
    }
    
}

