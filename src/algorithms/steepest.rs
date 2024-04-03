use std::collections::HashMap;
use std::time::Instant;

use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::data::instance::Instance;
use crate::algorithms::utils::{Algorithm, LocalSearch, SearchAlgorithm, AlgorithmStepStatistics};

pub struct SteepestLocalSearch {
    algorithm_name: String,
    instance: Instance,
    initial_solution: Option<Vec<i32>>,
    rng: ThreadRng,
    logging_interval: i32,
    log_history: HashMap<i32, AlgorithmStepStatistics>
}

impl LocalSearch for SteepestLocalSearch {
    fn _generate_neighbourhood(&self) ->  Vec<(usize, usize)> {
        let mut neighbourhood = Vec::new();
        for i in 0..self.instance.city_coords.len() {
            for j in i+1..self.instance.city_coords.len() {
                neighbourhood.push((i, j));
            }
        }
        neighbourhood
    }
}

impl SearchAlgorithm for SteepestLocalSearch {
    fn _load_initial_solution(&mut self) -> Vec<i32> {
        let current_solution: Vec<i32> = match &self.initial_solution {
            Some(solution) => solution.clone(),
            None => {
                let mut sol: Vec<i32> = (1..(self.instance.city_coords.len() + 1) as i32).collect();
                sol.shuffle(&mut self.rng);
                sol
            },
        };
        current_solution
    }
}

impl Algorithm for SteepestLocalSearch {
    fn new(instance: Instance, _steps: i32, logging_interval: i32, initial_solution: Option<Vec<i32>>) -> SteepestLocalSearch {
        Self {
            algorithm_name: "S".to_string(),
            instance,
            logging_interval,
            initial_solution,
            rng: thread_rng(),
            log_history: HashMap::new()
        }
    }
    
    fn get_name(&self) -> &String {
        &self.algorithm_name
    }

    fn execute(&mut self) -> HashMap<i32, AlgorithmStepStatistics> {
        let start_time = Instant::now();
        let mut current_solution = self._load_initial_solution();
        let mut current_distance = self.instance.get_solution_distance(&current_solution);
        let mut improved = true;
        let mut step = 0;
        let mut evaluated_solutions = 0;
        let neighbourhood = self._generate_neighbourhood();

        while improved {
            if step % self.logging_interval == 0 {
                let stats = AlgorithmStepStatistics {
                    solution: current_solution.clone(),
                    solution_score: self.instance.get_solution_score(&current_solution),
                    solution_distance: self.instance.get_solution_distance(&current_solution),
                    evaluated_solutions: evaluated_solutions,
                    elapsed_time: start_time.elapsed().as_micros()
                };
                self.log_history.insert(step, stats);
            }

            let mut best_neighbor_solution = current_solution.clone();
            let mut best_neighbor_distance = current_distance;
            improved = false;
            
            for (index1, index2) in &neighbourhood {
                evaluated_solutions += 1;
                let mut neighbor_solution = current_solution.clone();
                neighbor_solution.swap(*index1, *index2);
                
                let neighbor_distance = self.instance.get_solution_distance(&neighbor_solution);
                
                // Move to the first improvement found
                if neighbor_distance < current_distance {
                    best_neighbor_solution = neighbor_solution;
                    best_neighbor_distance = neighbor_distance;
                    step += 1;
                }
            }

            // Move to the best neighbor if it improves the solution
            if best_neighbor_distance < current_distance {
                current_solution = best_neighbor_solution;
                current_distance = best_neighbor_distance;
                improved = true;
            } 
        }

        let stats = AlgorithmStepStatistics {
            solution: current_solution.clone(),
            solution_score: self.instance.get_solution_score(&current_solution),
            solution_distance: self.instance.get_solution_distance(&current_solution),
            evaluated_solutions: evaluated_solutions,
            elapsed_time: start_time.elapsed().as_micros()
        };
        self.log_history.insert(step, stats);

        self.log_history.clone()
    }
}