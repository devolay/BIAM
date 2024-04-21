use rand::prelude::*;
use std::cmp::Ordering;
use std::collections::VecDeque;

use std::collections::HashMap;
use std::f64::INFINITY;
use std::time::Instant;

use crate::data::instance::Instance;
use crate::algorithms::utils::{Algorithm, SearchAlgorithm, AlgorithmStepStatistics, NeighbourhoodGenerator};


pub struct TabuSearch {
    algorithm_name: String,
    instance: Instance,
    initial_solution: Option<Vec<i32>>,
    rng: ThreadRng,
    logging_interval: i32,
    log_history: HashMap<i32, AlgorithmStepStatistics>
}


impl SearchAlgorithm for TabuSearch {
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

impl NeighbourhoodGenerator for TabuSearch {
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

impl Algorithm for TabuSearch {
    fn new(instance: Instance, _steps: i32, logging_interval: i32, initial_solution: Option<Vec<i32>>) -> TabuSearch {
        Self {
            algorithm_name: "TS".to_string(),
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
        let neighbourhood = self._generate_neighbourhood();

        let mut current_solution = self._load_initial_solution();
        let mut current_distance = self.instance.get_solution_distance(&current_solution);

        let mut best_solution = current_solution.clone();
        let mut best_distance = current_distance.clone();

        let mut evaluated_solutions: i32 = 0;
        let mut no_improvement_counter: i32 = 0;
        let mut step = 0;

        // Initialize tabu specific parameters
        let tabu_tenure = self.instance.city_coords.len() / 4;
        let mut tabu_list: HashMap<(usize, usize), usize> = HashMap::new(); // Move as a key and current tenure as value

        let mut master_list: VecDeque<((usize, usize), f64)> = VecDeque::new(); // Move as first element and distance as second
        let elite_k = self.instance.city_coords.len() / 10;
        let mut master_list_threshold: f64 = INFINITY;

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

            // Update tabu tenures
            for current_tenure in tabu_list.values_mut() {
                *current_tenure -= 1;
            }
            // Remove decayed move from tabu list
            let decayed_moves: Vec<(usize, usize)> = tabu_list.iter()
                .filter(|&(_, &value)| value <= 0)
                .map(|(key, _)| *key)
                .collect();
            for _move in decayed_moves {
                tabu_list.remove(&_move);
            }

            // Construct master list 
            if master_list.is_empty(){
                // If its empty, evaluate whole neighbourhood and take the elite
                let mut temp_master_list: Vec<((usize, usize), f64)> = neighbourhood.iter()
                .map(|&move_| {
                    let mut move_solution: Vec<i32> = current_solution.clone();
                    move_solution.swap(move_.0, move_.1);
                    let distance = self.instance.get_solution_distance(&move_solution);
                    (move_, distance)
                }).collect();
                evaluated_solutions += neighbourhood.len() as i32;

                temp_master_list.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal));
                master_list = temp_master_list.into_iter().take(elite_k).collect();
                master_list_threshold = current_distance - master_list.back().expect("").1;
            } else {
                // If elite candidates exists, re-evaluate only them
                let mut temp_master_list: Vec<((usize, usize), f64)> = Vec::from(master_list).iter()
                .map(|&(move_, _)| {
                    let mut move_solution: Vec<i32> = current_solution.clone();
                    move_solution.swap(move_.0, move_.1);
                    let distance = self.instance.get_solution_distance(&move_solution);
                    (move_, distance)
                }).collect();
                evaluated_solutions += temp_master_list.len() as i32;

                temp_master_list.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal));
                master_list = temp_master_list.into_iter().collect();
                if master_list.front().expect("").1 < master_list_threshold {
                    // If best solution worse than master threshold, then create master list from scratch
                    master_list = VecDeque::new();
                    continue;
                }
            }

            let (master_move, candidate_distance)= master_list.pop_front().expect("");
            if !tabu_list.contains_key(&master_move) || candidate_distance < best_distance{
                // If move is not tabu or has best distance ever found then use it
                let mut candidate = current_solution.clone();
                candidate.swap(master_move.0, master_move.1);
                current_solution = candidate;
                current_distance = candidate_distance;
                tabu_list.insert(master_move, tabu_tenure);
                step += 1;

                if current_distance < best_distance {
                    best_solution = current_solution.clone();
                    best_distance = current_distance;
                    no_improvement_counter = 0;
                } else {
                    no_improvement_counter += 1;
                }
            }

            if no_improvement_counter > self.instance.city_coords.len()as i32 {
                let stats = AlgorithmStepStatistics {
                    solution: best_solution.clone(),
                    solution_score: self.instance.get_solution_score(&best_solution),
                    solution_distance: self.instance.get_solution_distance(&best_solution),
                    evaluated_solutions: evaluated_solutions,
                    elapsed_time: start_time.elapsed().as_micros()
                };
                self.log_history.insert(step, stats);
                break;
            }
        }
        self.log_history.clone()
    }
}