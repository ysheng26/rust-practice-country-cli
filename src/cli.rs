use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(long)]
    region: Option<String>,

    #[arg(long, default_value = "population")]
    sort: String,

    #[arg(long, default_value_t = 10)]
    top: usize,
}

// #[derive(ValueEnum)]
pub enum SortBy {
    Population,
    Area,
    Name,
}
