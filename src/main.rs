use clap::Parser;

mod cli;
mod service;

// what data structure should I use to hold the params?
// I can just use Args and pass to reqwest

fn main() {
    let cli = cli::Args::parse();

    // question: Some(ref region) or cli.region.as_deref()
    if let Some(ref region) = cli.region {
        println!("region = {}", region);
    }

    // sort by population, area or name
}
