mod data;
mod algorithms;
mod experiments;
use experiments::run_comparison_experiment;

fn main() {
    run_comparison_experiment(
        100, 
        "/Users/dawid/Private/School/Sem 1/Biologically Inspired Algorithms/codebase/results_SA_TA.csv"
    )
}

