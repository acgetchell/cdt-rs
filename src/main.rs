use cdt_rs::{bowyer_watson, Point};

fn main() {
    let points = vec![
        Point { x: 0.0, y: 0.0 },
        Point { x: 1.0, y: 0.0 },
        Point { x: 0.5, y: 1.0 },
    ];

    let triangulation = bowyer_watson(points);
    for triangle in triangulation {
        println!(
            "Triangle: {:?} Center: {:?}",
            triangle.vertices,
            triangle.center()
        );
    }
}
