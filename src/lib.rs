use clap::Parser;

pub mod delaunay;

pub mod utilities;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// Dimensionality of the triangulation
    #[arg(short, long, value_parser = clap::value_parser!(u32).range(2..4))]
    dimension: Option<u32>,

    /// Number of vertices
    #[arg(short, long, value_parser = clap::value_parser!(u32).range(3..))]
    vertices: u32,

    /// Number of timeslices
    #[arg(short, long, value_parser = clap::value_parser!(u32).range(1..))]
    timeslices: u32,
}

impl Config {
    pub fn build() -> Self {
        Self::parse()
    }
}

pub fn run(config: Config) -> Result<Vec<delaunay::Triangle>, Box<dyn std::error::Error>> {
    let vertices = config.vertices;
    let timeslices = config.timeslices;

    if config.dimension.is_some_and(|d| d != 2) {
        eprintln!("Only 2D triangulations are supported right now.");
        std::process::exit(1);
    }

    println!("Dimensionality: {}", config.dimension.unwrap_or(2));
    println!("Number of vertices: {}", vertices);
    println!("Number of timeslices: {}", timeslices);

    let scale = 10.0; // The size of the grid
    let mut points = Vec::new();

    for _n in 1..vertices {
        let x = utilities::generate_random_float() * scale;
        let y = utilities::generate_random_float() * scale;
        points.push(delaunay::Point { x, y });
    }

    let triangulation = delaunay::bowyer_watson(points);
    for triangle in &triangulation {
        println!(
            "Triangle: {:?} Center: {:?}",
            triangle.vertices,
            triangle.center()
        );
    }

    Ok(triangulation)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_run() {
        let config = Config {
            dimension: Some(2),
            vertices: 32,
            timeslices: 3,
        };
        assert!(config.dimension.is_some());
        assert!(run(config).is_ok());
    }

    #[test]
    fn points_is_number_of_vertices() {
        let config = Config {
            dimension: Some(2),
            vertices: 32,
            timeslices: 3,
        };
        let triangulation = run(config).unwrap();
        assert!(triangulation.len() > 42);
    }
}
