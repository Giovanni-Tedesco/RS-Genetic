
// Makes a template to create custom distributions
// that conform to some standard
pub trait custom_distribution<T> {

    // Samples from the distribution
    fn sample(&self) -> f64; 

    // Generate a new distribution
    fn new(params: T) -> Self;

    // The probability density function
    fn pdf() -> f64;

    // Generate a sample of n fitnesses.
    // fn sample_n(&self) -> Vec<f64>;
}