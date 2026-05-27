use clap::Parser;
use clap::ValueEnum;

// question: seems like there are too many "pub" here
// Claude was wrong, these are needed in service.rs
// For now let's leave it
#[derive(Parser)]
pub struct Args {
    #[arg(long)]
    pub region: Option<String>,

    #[arg(long, default_value = "population")]
    pub sort: SortBy,

    #[arg(long, default_value_t = 10)]
    pub top: usize,
}

#[derive(ValueEnum, Clone, PartialEq)]
pub enum SortBy {
    Population,
    Area,
    Name,
}
