use std::collections::HashMap;
use std::time::Instant;

use rand::Rng;
use rand::rngs::ThreadRng;
use rand::thread_rng;

use crate::data::instance::Instance;
use crate::algorithms::utils::{Algorithm, AlgorithmStepStatistics};

pub struct HeuristicBaseline {
    algorithm_name: String,
    instance: Instance,
    rng: ThreadRng,
    log_history: HashMap<i32, AlgorithmStepStatistics>
}

impl Algorithm for HeuristicBaseline {
    fn new(instance: Instance, _steps: i32, _logging_interval: i32, _initial_solution: Option<Vec<i32>>) -> HeuristicBaseline {
        Self {
            algorithm_name: "H".to_string(),
            instance,
            rng: thread_rng(),
            log_history: HashMap::new()
        }
    }

    fn get_name(&self) -> &String {
        &self.algorithm_name
    }

    fn execute(&mut self) -> HashMap<i32, AlgorithmStepStatistics> {
        let start_time = Instant::now();
        let num_cities = self.instance.city_coords.len();
        let mut visited = vec![false; num_cities];
        let start_city = self.rng.gen_range(0..num_cities) as i32;
        visited[start_city as usize] = true;

        let mut tour = vec![start_city + 1];
        let mut current_city = start_city;

        while tour.len() < num_cities {
            let candidates: Vec<usize> = (0..num_cities)
                .filter(|&i| !visited[i])
                .collect();

            let mut min_distance = f64::INFINITY;
            let mut next_city = None;

            for &candidate_city in &candidates {
                let distance = self.instance.adjacency_matrix[current_city as usize][candidate_city];
                if distance < min_distance {
                    min_distance = distance;
                    next_city = Some(candidate_city);
                }
            }

            if let Some(next_city) = next_city {
                tour.push(next_city as i32 + 1);
                visited[next_city] = true;
                current_city = next_city as i32;
            }
        }
        let stats = AlgorithmStepStatistics {
            solution: tour.clone(),
            solution_score: self.instance.get_solution_score(&tour),
            solution_distance: self.instance.get_solution_distance(&tour),
            evaluated_solutions: 0,
            elapsed_time: start_time.elapsed().as_micros()
        };
        self.log_history.insert(0, stats);
        self.log_history.clone()
    }
}