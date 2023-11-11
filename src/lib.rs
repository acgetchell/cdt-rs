// #![warn(missing_docs)]

//! Using <https://crates.io/crates/clap> for command line arguments
//! Using <https://crates.io/crates/spade> for 2D Delaunay triangulation
use clap::Parser;
use spade::InsertionError;
use spade::Triangulation;

mod triangulation {
    pub mod spade_triangulations;
}

use triangulation::spade_triangulations::generate_random_delaunay2;

/// Contains utility functions for the `cdt-rs` crate.
pub mod utilities;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
/// Configuration options for the `cdt-rs` crate.
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
    /// Builds a new instance of `Config`.
    pub fn build() -> Self {
        Self::parse()
    }
}

// pub fn run(config: Config) -> Result<Vec<triangulation::Triangle>, Box<dyn std::error::Error>> {
pub fn run(
    config: &Config,
) -> Result<spade::DelaunayTriangulation<spade::Point2<f64>>, InsertionError> {
    let vertices = config.vertices;
    let timeslices = config.timeslices;

    if config.dimension.is_some_and(|d| d != 2) {
        eprintln!("Only 2D triangulations are supported right now.");
        std::process::exit(1);
    }

    println!("Dimensionality: {}", config.dimension.unwrap_or(2));
    println!("Number of vertices: {}", vertices);
    println!("Number of timeslices: {}", timeslices);

    let triangulation = generate_random_delaunay2(vertices)?;

    println!("Number of triangles: {}", triangulation.num_inner_faces());

    // let scale = 10.0; // The size of the grid
    // let mut points = Vec::new();

    // for _n in 1..vertices {
    //     let x = utilities::generate_random_float() * scale;
    //     let y = utilities::generate_random_float() * scale;
    //     points.push(triangulation::Point { x, y });
    // }

    // let triangulation = triangulation::bowyer_watson(points);
    // for triangle in &triangulation {
    //     println!(
    //         "Triangle: {:?} Center: {:?}",
    //         triangle.vertices,
    //         triangle.center()
    //     );
    // }

    for vertex in triangulation.vertices() {
        println!("Vertex: {:?}", vertex.position());
    }

    for triangle in triangulation.inner_faces() {
        println!("Triangle: {:?}", triangle.positions());
    }

    Ok(triangulation)
}

#[cfg(test)]
mod lib_tests {
    use super::*;
    #[test]
    fn test_run() {
        let config = Config {
            dimension: Some(2),
            vertices: 32,
            timeslices: 3,
        };
        assert!(config.dimension.is_some());
        assert!(run(&config).is_ok());
    }

    #[test]
    fn points_is_number_of_vertices() {
        let config = Config {
            dimension: Some(2),
            vertices: 32,
            timeslices: 3,
        };
        let triangulation = run(&config).unwrap();
        assert_eq!(
            triangulation.num_vertices(),
            config.vertices.try_into().unwrap()
        );
    }
}
