use clap::{Parser, ValueEnum};
#[derive(Debug, Clone, ValueEnum)]
pub enum ProblemId {
    CountElemGtPriorAvg,
}
#[derive(Parser, Debug)]
pub struct AppArgs {}
