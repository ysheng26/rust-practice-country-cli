use clap::Parser;

mod cli;
mod service;

fn main() {
    let _ = cli::Args::parse();
    println!("Hello, world!");
}
