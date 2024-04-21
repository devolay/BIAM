use std::collections::HashMap;
use crate::data::instance::Instance;


pub trait Algorithm {
    fn new(instance: Instance, steps: i32, logging_interval: i32, initial_solution: Option<Vec<i32>>) -> Self where Self: Sized;
    fn execute(&mut self) -> HashMap<i32, AlgorithmStepStatistics>;
    fn get_name(&self) -> &String;
}
pub trait SearchAlgorithm {
    fn _load_initial_solution(&mut self) -> Vec<i32>;
}

pub trait NeighbourhoodGenerator: SearchAlgorithm {
    fn _generate_neighbourhood(&self) ->  Vec<(usize, usize)>;
}

#[derive(Clone)]
pub struct AlgorithmStepStatistics {
    pub solution: Vec<i32>,
    pub solution_score: f64,
    pub solution_distance: f64,
    pub evaluated_solutions: i32,
    pub elapsed_time: u128,
}


pub fn get_move_distance(a: usize, b: usize, instance: &Instance, current_solution: &Vec<i32>) -> f64 {
    let mut a_candidate = current_solution.clone();
    a_candidate.swap(a, b);
    instance.get_solution_distance(&a_candidate)
}
