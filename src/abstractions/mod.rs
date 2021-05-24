mod custom_distribution;

pub use custom_distribution::CustomDistribution;

pub type FitFunc<T, R> = Box<dyn Fn(&T) -> R>;