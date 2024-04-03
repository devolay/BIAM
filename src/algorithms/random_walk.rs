use std::collections::HashMap;
use std::time::Instant;

use rand::Rng;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::data::instance::Instance;
use crate::algorithms::utils::{Algorithm, SearchAlgorithm, AlgorithmStepStatistics};


pub struct RandomWalk {
    algorithm_name: String,
    instance: Instance,
    steps: i32,
    initial_solution: Option<Vec<i32>>,
    rng: ThreadRng,
    logging_interval: i32,
    log_history: HashMap<i32, AlgorithmStepStatistics>
}

impl SearchAlgorithm for RandomWalk {
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

impl Algorithm for RandomWalk {
    fn new(instance: Instance, steps: i32, logging_interval: i32, initial_solution: Option<Vec<i32>>) -> RandomWalk {
        Self {
            algorithm_name: "RW".to_string(),
            instance,
            steps,
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
        let mut step = 0;
        let mut evaluated_solutions = 0;

        for _ in 0..self.steps {
            evaluated_solutions += 1;
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
            let mut neighbor_solution = current_solution.clone();
            let index1 = self.rng.gen_range(0..neighbor_solution.len());
            let index2 = self.rng.gen_range(0..neighbor_solution.len());
            neighbor_solution.swap(index1, index2);
            
            let neighbor_distance = self.instance.get_solution_distance(&neighbor_solution);
            
            if neighbor_distance < current_distance {
                current_solution = neighbor_solution;
                current_distance = neighbor_distance;
                step += 1;
            }
        }

        let stats = AlgorithmStepStatistics {
            solution: current_solution.clone(),
            solution_score: self.instance.get_solution_score(&current_solution),
            solution_distance: self.instance.get_solution_distance(&current_solution),
            evaluated_solutions: evaluated_solutions,
            elapsed_time: start_time.elapsed().as_micros()
        };
        self.log_history.insert(self.steps, stats);

        self.log_history.clone()
    }
}