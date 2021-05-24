mod custom_distribution;

pub use custom_distribution::CustomDistribution;

pub type FitFunc<T> = Box<dyn Fn(&T) -> f64>;