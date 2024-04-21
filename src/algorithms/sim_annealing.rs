use rand::prelude::*;
use std::f64::consts::E;
use std::collections::HashMap;
use std::time::Instant;

use crate::data::instance::Instance;
use crate::algorithms::utils::{Algorithm, SearchAlgorithm, AlgorithmStepStatistics, NeighbourhoodGenerator};

pub struct SimmulatedAnnealing {
    algorithm_name: String,
    instance: Instance,
    initial_solution: Option<Vec<i32>>,
    rng: ThreadRng,
    logging_interval: i32,
    log_history: HashMap<i32, AlgorithmStepStatistics>
}

impl SimmulatedAnnealing {
    fn acceptance_probability(&mut self, current_energy: f64, new_energy: f64, temperature: f64) -> f64 {
        if new_energy < current_energy {
            1.0
        } else {
            E.powf(-(new_energy - current_energy) / temperature)
        }
    }
}

impl SearchAlgorithm for SimmulatedAnnealing {
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

impl NeighbourhoodGenerator for SimmulatedAnnealing {
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

impl Algorithm for SimmulatedAnnealing {
    fn new(instance: Instance, _steps: i32, logging_interval: i32, initial_solution: Option<Vec<i32>>) -> SimmulatedAnnealing {
        Self {
            algorithm_name: "SA".to_string(),
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
        let neighbourhood: Vec<(usize, usize)> = self._generate_neighbourhood();
    
        let mut current_solution = self._load_initial_solution();
        let mut current_distance = self.instance.get_solution_distance(&current_solution);

        let mut best_solution = current_solution.clone();
        let mut best_distance = current_distance.clone();

        let mut step = 0;
        let mut evaluated_solutions = 0;

        // Annealing specific params
        let mut temp: f64 = 1.0;
        let mut no_improvement_counter: i32 = 0;
        let heating_rate = 1.1;
        let cooling_rate = 0.99;
        let max_iterations = self.instance.city_coords.len() * 2;

        // Heating
        loop {
            let mut accepted_solutions: usize = 0;
            for _ in 0..max_iterations {
                let mut new_solution = current_solution.clone();
                let (swap_index1, swap_index2) = neighbourhood.choose(&mut self.rng).expect("Neighbourhood error!");
                new_solution.swap(*swap_index1, *swap_index2);
                let new_distance = self.instance.get_solution_distance(&new_solution);
                
                if self.acceptance_probability(current_distance, new_distance, temp) > self.rng.gen() {
                    accepted_solutions += 1;
                    current_solution = new_solution;
                    current_distance = new_distance;
                }
            }

            let acceptance_rate = accepted_solutions as f64 / max_iterations as f64;
            if acceptance_rate >= 0.95 {
                break;
            }
                
            temp *= heating_rate   
        };

        // Cooling
        loop {
            if step % self.logging_interval == 0 {
                let stats = AlgorithmStepStatistics {
                    solution: best_solution.clone(),
                    solution_score: self.instance.get_solution_score(&best_solution),
                    solution_distance: self.instance.get_solution_distance(&best_solution),
                    evaluated_solutions: evaluated_solutions,
                    elapsed_time: start_time.elapsed().as_micros()
                };
                self.log_history.insert(step, stats);
            }

            for _ in 0..max_iterations{
                let mut new_solution = current_solution.clone();
                let (swap_index1, swap_index2) = neighbourhood.choose(&mut self.rng).expect("Neighbourhood error!");
                new_solution.swap(*swap_index1, *swap_index2);

                let new_distance = self.instance.get_solution_distance(&new_solution);
                evaluated_solutions += 1;

                if self.acceptance_probability(current_distance, new_distance, temp) > self.rng.gen() {
                    current_solution = new_solution;
                    current_distance = new_distance;
                    step += 1;
                }

                if current_distance < best_distance {
                    best_solution = current_solution.clone();
                    best_distance = current_distance.clone();
                    no_improvement_counter = 0;
                }
                else {
                    no_improvement_counter += 1;
                }
            }
            temp *= cooling_rate;

            if temp < 0.01 && no_improvement_counter > self.instance.city_coords.len() as i32 {
                break;
            }
        }

        let stats = AlgorithmStepStatistics {
            solution: best_solution.clone(),
            solution_score: self.instance.get_solution_score(&best_solution),
            solution_distance: self.instance.get_solution_distance(&best_solution),
            evaluated_solutions: evaluated_solutions,
            elapsed_time: start_time.elapsed().as_micros()
        };
        self.log_history.insert(step, stats);
        self.log_history.clone()
    }
}