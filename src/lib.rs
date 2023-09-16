use clap::Parser;

pub mod delaunay;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Config {
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

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let vertices = config.vertices;
    let timeslices = config.timeslices;
    println!("Number of vertices: {}", vertices);
    println!("Number of timeslices: {}", timeslices);
    let points = vec![
        delaunay::Point { x: 0.0, y: 0.0 },
        delaunay::Point { x: 1.0, y: 0.0 },
        delaunay::Point { x: 0.5, y: 1.0 },
    ];

    let triangulation = delaunay::bowyer_watson(points);
    for triangle in triangulation {
        println!(
            "Triangle: {:?} Center: {:?}",
            triangle.vertices,
            triangle.center()
        );
    }

    Ok(())
}

#[test]
fn test_run() {
    let config = Config {
        vertices: 32,
        timeslices: 3,
    };
    assert!(run(config).is_ok());
}
