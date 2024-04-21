use std::fs::File;
use std::io::BufWriter;
use csv::Writer;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

use crate::algorithms::utils::Algorithm;
use crate::data::instance::Instance;
use crate::algorithms::random_search::RandomSearch;
use crate::algorithms::random_walk::RandomWalk;
use crate::algorithms::steepest::SteepestLocalSearch;
use crate::algorithms::greedy::GreedyLocalSearch;
use crate::algorithms::heuristic::HeuristicBaseline;
use crate::algorithms::sim_annealing::SimmulatedAnnealing;
use crate::algorithms::tabu_search::TabuSearch;


pub fn run_comparison_experiment(num_runs: usize, file_path: &str) {
    let algorithms: Vec<Box<dyn Fn (Instance, i32, i32, Option<Vec<i32>>) -> Box<dyn Algorithm> + Send + Sync>> = vec![
        // Box::new(|instance, steps, logging_interval, initial_solution| 
        //     Box::new(HeuristicBaseline::new(instance, steps, logging_interval, initial_solution)) as Box<dyn Algorithm>),
        // Box::new(|instance, steps, logging_interval, initial_solution| 
        //     Box::new(RandomSearch::new(instance, steps, logging_interval, initial_solution)) as Box<dyn Algorithm>),
        // Box::new(|instance, steps, logging_interval, initial_solution| 
        //     Box::new(RandomWalk::new(instance, steps, logging_interval, initial_solution)) as Box<dyn Algorithm>),
        // Box::new(|instance, steps, logging_interval, initial_solution| 
        //     Box::new(GreedyLocalSearch::new(instance, steps, logging_interval, initial_solution)) as Box<dyn Algorithm>),
        // Box::new(|instance, steps, logging_interval, initial_solution| 
        //     Box::new(SteepestLocalSearch::new(instance, steps, logging_interval, initial_solution)) as Box<dyn Algorithm>),
        // Box::new(|instance, steps, logging_interval, initial_solution| 
        //     Box::new(SimmulatedAnnealing::new(instance, steps, logging_interval, initial_solution)) as Box<dyn Algorithm>)
        Box::new(|instance, steps, logging_interval, initial_solution| 
            Box::new(SimmulatedAnnealing::new(instance, steps, logging_interval, initial_solution)) as Box<dyn Algorithm>),
        Box::new(|instance, steps, logging_interval, initial_solution| 
            Box::new(TabuSearch::new(instance, steps, logging_interval, initial_solution)) as Box<dyn Algorithm>)
    ];
    let instance_names: Vec<&str> = vec!["berlin52", "ch130", "eil76", "lin105", "tsp225", "kroA100", "kroC100", "kroD100"];
    let writer = Arc::new(Mutex::new(Writer::from_writer(BufWriter::new(File::create(file_path).expect("Cannot create file")))));

    writer.lock().unwrap().write_record(
        &["Instance", "Algorithm", "Run", "Step", "Evaluated Solutions", "Elapsed Time (Microseconds)", "Solution", "Solution Score", "Solution Distance", "Optimal Solution", "Optimal Solution Distance"]
    ).expect("Error writing header");
        
    for instance_name in instance_names{
        println!("Running experiments on {:?} instance", instance_name);
        let instance = Instance::new(instance_name);
        algorithms.iter().for_each(|algo_creator| {
            (1..num_runs).into_par_iter().for_each(|run| {
                let optimal_solution = &instance.optimal_solution;
                let optimal_solution_str =  format!("{:?}", optimal_solution);
                let optimal_solution_distance = &instance.get_solution_distance(&optimal_solution);
                let mut algorithm = algo_creator(instance.clone(), 1000, 5, None);
                let result = algorithm.execute();            
                for (step, stats) in result.iter() {
                    let record = vec![
                        instance_name.to_string(),
                        algorithm.get_name().to_string(),
                        run.to_string(),
                        step.to_string(),
                        stats.evaluated_solutions.to_string(),
                        stats.elapsed_time.to_string(),
                        format!("{:?}", stats.solution),
                        stats.solution_score.to_string(),
                        stats.solution_distance.to_string(),
                        optimal_solution_str.clone(),
                        optimal_solution_distance.to_string(),
                    ];
                    let mut guard = writer.lock().unwrap();
                    guard.write_record(&record).expect("Error writing record");
                }
            });
        });
    }
}


pub fn run_initial_solution_experiment(num_runs: usize, file_path: &str) {

    let algorithms: Vec<Box<dyn Fn (Instance, i32, i32, Option<Vec<i32>>) -> Box<dyn Algorithm>>> = vec![
        Box::new(|instance, steps, logging_interval, initial_solution| 
            Box::new(GreedyLocalSearch::new(instance, steps, logging_interval, initial_solution)) as Box<dyn Algorithm>),
        Box::new(|instance, steps, logging_interval, initial_solution| 
            Box::new(SteepestLocalSearch::new(instance, steps, logging_interval, initial_solution)) as Box<dyn Algorithm>)
    ];
    let instance_names: Vec<&str> = vec!["berlin52", "eil76"];
    let mut writer = Writer::from_writer(BufWriter::new(File::create(file_path).expect("Cannot create file")));

    writer.write_record(
        &["Instance", "Algorithm", "Run", "Step", "Evaluated Solutions", "Elapsed Time (Microseconds)", "Solution", "Solution Score", "Solution Distance", "Optimal Solution", "Optimal Solution Distance"]
    ).expect("Error writing header");
        
    for instance_name in instance_names{
        println!("Running experiments on {:?} instance", instance_name);
        let instance = Instance::new(instance_name);
        let optimal_solution = &instance.optimal_solution;
        let optimal_solution_str =  format!("{:?}", optimal_solution);
        let optimal_solution_distance = instance.get_solution_distance(&optimal_solution);

        for algo_creator in algorithms.iter() {
            for run in 1..num_runs {
            //  let mut heur = HeuristicBaseline::new(instance.clone(), 1000, 10000, None);
            //     let heur_result = heur.execute();   
                let mut algorithm = algo_creator(instance.clone(), 1000, 10000, None);
                let result = algorithm.execute();            
                for (step, stats) in result.iter() {
                    writer.write_record(&[
                        instance_name,
                        &algorithm.get_name(),
                        &run.to_string(),
                        &step.to_string(),
                        &stats.evaluated_solutions.to_string(),
                        &stats.elapsed_time.to_string(),
                        &format!("{:?}", stats.solution),
                        &stats.solution_score.to_string(),
                        &stats.solution_distance.to_string(),
                        &optimal_solution_str,
                        &optimal_solution_distance.to_string(),
                    ]).expect("Error writing record");
                }
            }
        }
    }
}