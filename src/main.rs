// use cdt_rs::{bowyer_watson, Point};
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Number of vertices
    #[arg(value_parser = clap::value_parser!(u32).range(3..))]
    number_of_vertices: u32,
}

fn main() {
    let cli = Cli::parse();

    if let Err(e) = cdt_rs::run(cli.number_of_vertices) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
