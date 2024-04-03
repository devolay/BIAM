use std::collections::HashMap;
use std::time::Instant;

use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::data::instance::Instance;
use crate::algorithms::utils::{Algorithm, SearchAlgorithm, AlgorithmStepStatistics};


pub struct RandomSearch {
    algorithm_name: String,
    instance: Instance,
    steps: i32,
    initial_solution: Option<Vec<i32>>,
    rng: ThreadRng,
    logging_interval: i32,
    log_history: HashMap<i32, AlgorithmStepStatistics>
}

impl SearchAlgorithm for RandomSearch {
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

impl Algorithm for RandomSearch {
    fn new(instance: Instance, steps: i32, logging_interval: i32, initial_solution: Option<Vec<i32>>) -> RandomSearch {
        Self {
            algorithm_name: "RS".to_string(),
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
            let mut solution: Vec<i32> = (1..(self.instance.city_coords.len() + 1) as i32).collect();
            solution.shuffle(&mut self.rng);
            let distance = self.instance.get_solution_distance(&solution);
            if distance < current_distance {
                current_solution = solution;
                current_distance = distance;
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