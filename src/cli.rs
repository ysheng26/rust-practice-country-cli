use clap::Parser;
use clap::ValueEnum;

// question: seems like there are too many "pub" here
#[derive(Parser)]
pub struct Args {
    #[arg(long)]
    pub region: Option<String>,

    #[arg(long, default_value = "population")]
    pub sort_by: SortBy,

    #[arg(long, default_value_t = 10)]
    pub top: usize,
}

#[derive(ValueEnum, Clone)]
pub enum SortBy {
    Population,
    Area,
    Name,
}
