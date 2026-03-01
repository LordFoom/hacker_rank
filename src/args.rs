use clap::{Parser, ValueEnum};
#[derive(Debug, Clone, ValueEnum)]
pub enum ProblemId {
    CountElemGtPriorAvg,
    Problem1,
}
#[derive(Parser, Debug)]
pub struct AppArgs {
    pub problem_ids: Vec<ProblemId>,
}
