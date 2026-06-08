use clap::Parser;
use clap::ValueEnum;

#[derive(Parser)]
pub struct Args {
    #[arg(long)]
    pub region: Option<String>,

    #[arg(long, default_value = "population")]
    pub sort: SortBy,

    #[arg(long, default_value_t = 10)]
    pub top: usize,

    #[arg(long)]
    pub language: Option<String>,

    #[arg(long)]
    pub asc: bool,

    #[arg(long, default_value_t = 0)]
    pub min_population: u32,
}

#[derive(ValueEnum, Clone, PartialEq)]
pub enum SortBy {
    Population,
    Area,
    Name,
}
