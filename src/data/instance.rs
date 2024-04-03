use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::data::utils;

const DATA_PATH: &str = "/Users/dawid/Private/School/Sem 1/Biologically Inspired Algorithms/data/tsp/SEL_tsp/";

#[derive(Clone)]
pub struct Instance {
    pub name: String,
    pub optimal_solution: Vec<i32>,
    pub adjacency_matrix: Vec<Vec<f64>>,
    pub city_coords: Vec<(f64, f64)>,
}

impl Instance {
    pub fn new(name: &str) -> Instance {
        println!("Loading instance");
        let instance_path = vec![DATA_PATH, name, ".tsp"].join("");
        let solution_path = vec![DATA_PATH, name, ".opt.tour"].join("");
        let (adjacency_matrix, city_coords) = Instance::load_instance(&instance_path);
        let optimal_solution = Instance::load_optimal_solution(&solution_path);
        let name = name.to_string();
        println!("Instance {:?} loaded.", &name);
        println!("Dimensions = {:?}", city_coords.len());
        Instance {
            name,
            optimal_solution,
            adjacency_matrix,
            city_coords,
        }
    }

    fn _adjacency_matrix(coordinates: &Vec<(f64, f64)>) -> Vec<Vec<f64>> {
        let mut adjacency_matrix = vec![vec![0.0; coordinates.len()]; coordinates.len()];
        for i in 0..coordinates.len() {
            for j in i + 1..coordinates.len() {
                let distance = utils::euclidean_distance(coordinates[i], coordinates[j]);
                // Symetric problem
                adjacency_matrix[i][j] = distance;
                adjacency_matrix[j][i] = distance;
            }
        }
        adjacency_matrix
    }

    fn load_optimal_solution(path: &str) -> Vec<i32> {
        let file = File::open(path).expect("Failed to open file");
        let reader = BufReader::new(file);
        let mut optimal_solution = Vec::new();
        let mut is_reading_tour = false;

        for line in reader.lines() {
            let line = line.expect("Failed to read line");
            let parts: Vec<&str> = line.trim().split_whitespace().collect();

            if parts.len() == 1 {
                if parts[0] == "TOUR_SECTION" {
                    is_reading_tour = true;
                    continue;
                }
                if is_reading_tour {
                    let id = parts[0].parse::<i32>().expect("Invalid ID format");
                    if id == -1 {
                        is_reading_tour = false;
                        continue;
                    }
                    optimal_solution.push(id);
                }
            }
        }
        optimal_solution
    }

    fn load_instance(path: &str) -> (Vec<Vec<f64>>, Vec<(f64, f64)>) {
        let file = File::open(path).expect("Failed to open file");
        let reader = BufReader::new(file);
        let mut coords = HashMap::new();
        let mut coords_section: bool = false;
        let mut dimension = 0;

        for line in reader.lines() {
            let line = line.unwrap();
            let parts: Vec<&str> = line.split_whitespace().collect();

            if line.starts_with("EDGE_WEIGHT_TYPE") {
                assert!(parts.last().unwrap() == &"EUC_2D", 
                "Use only instances with EUC_2D weight type")
            }
            else if line.starts_with("DIMENSION") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                dimension = parts.last().unwrap().parse::<usize>().unwrap();
            }
            else if line.trim() == "NODE_COORD_SECTION" {
                coords_section = true;
                continue;
            }

            if coords_section && parts.len() == 3 {
                let id = parts[0].parse::<usize>().expect("Invalid ID format");
                let x = parts[1].parse::<f64>().expect("Invalid X coordinate format");
                let y = parts[2].parse::<f64>().expect("Invalid Y coordinate format");
                coords.insert(id, (x, y));
            }
        }

        let city_coords: Vec<(f64, f64)> = (1..=dimension)
            .map(|i| coords[&i])
            .collect();

        let adjacency_matrix: Vec<Vec<f64>> = Instance::_adjacency_matrix(&city_coords);

        (adjacency_matrix, city_coords)
    }

    pub fn get_solution_distance(&self, solution: &Vec<i32>) -> f64 {
        let mut dist = 0.0;
        assert_eq!(solution.len(), self.city_coords.len(), "Solution has different dimensionality than instance!");
        for i in 0..solution.len() - 1 {
            dist += self.adjacency_matrix[(solution[i] - 1) as usize][(solution[i+1] - 1) as usize];
        }
        dist += self.adjacency_matrix[(solution[solution.len() - 1] - 1) as usize][(solution[0] - 1) as usize];
        dist
    }

    pub fn get_solution_score(&self, solution: &Vec<i32>) -> f64 {
        // Scores given solution relatively to optimal solution distance
        let optimal_solution_distance = self.get_solution_distance(&self.optimal_solution);
        let solution_distance = self.get_solution_distance(&solution);
        return solution_distance / optimal_solution_distance;
    }

}
