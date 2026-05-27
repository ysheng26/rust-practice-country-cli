use clap::Parser;

mod cli;
mod service;

// what data structure should I use to hold the params?
// I can just use Args and pass to reqwest

fn main() {
    let args = cli::Args::parse();

    // question: Some(ref region) or cli.region.as_deref()
    // if let Some(ref region) = args.region {
    //     println!("region = {}", region);
    // }

    // sort by population, area or name

    // question: should I unwrap here?
    let xs = service::get_results(args).expect("failed to get results");
    // println!("{}", serde_json::to_string_pretty(&xs).unwrap());

    for (i, item) in xs.iter().enumerate() {
        println!("{} - {}", i, item);
    }

    // for item in xs.iter() {
    //     println!("{}", item);
    // }
}
