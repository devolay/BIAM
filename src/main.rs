mod data;
mod algorithms;
mod experiments;
use experiments::run_initial_solution_experiment;

fn main() {
    run_initial_solution_experiment(
        300, 
        "/Users/dawid/Private/School/Sem 1/Biologically Inspired Algorithms/codebase/restarts.csv"
    )
}

