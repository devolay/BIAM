extern crate rand;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn euclidean_distance(point1: (f64, f64), point2: (f64, f64)) -> f64 {
    let dx = point1.0 - point2.0;
    let dy = point1.1 - point2.1;
    (dx * dx + dy * dy).sqrt()
}

fn generate_random_unique_pairs(n: i32) -> Vec<Vec<(i32, i32)>> {
    let mut rng = rand::thread_rng();
    let mut numbers: Vec<i32> = (0..n).collect();
    let mut pairs: Vec<Vec<(i32, i32)>> = Vec::new();
    numbers.shuffle(&mut rng);
    for i in 0..(n as usize) {
        for j in (i + 1)..(n as usize) {
            let mut inner_pairs: Vec<(i32, i32)> = Vec::new();
            inner_pairs.push((numbers[i], numbers[j]));        }
    }
    pairs
}

fn permute_pairs(pairs: Vec<Vec<(i32, i32)>>) -> Vec<Vec<(i32, i32)>> {
    let mut rng = thread_rng();
    let mut shuffled_pairs = pairs.clone();
    shuffled_pairs.shuffle(&mut rng);
    shuffled_pairs
}