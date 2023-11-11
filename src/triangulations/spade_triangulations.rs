use spade::{validate_vertex, DelaunayTriangulation, InsertionError, Point2, Triangulation};

use crate::utilities::generate_random_float;

pub fn generate_random_delaunay2(
    number_of_vertices: u32,
) -> Result<spade::DelaunayTriangulation<spade::Point2<f64>>, InsertionError> {
    let mut triangulation: DelaunayTriangulation<_> = DelaunayTriangulation::new();

    for _n in 0..number_of_vertices {
        let point = generate_random_vertex(10.0);
        validate_vertex(&point)?;
        triangulation.insert(point)?;
    }

    Ok(triangulation)
}

fn generate_random_vertex(scale: f64) -> Point2<f64> {
    // let scale = 10.0; // The size of the grid
    let x = generate_random_float() * scale;
    let y = generate_random_float() * scale;
    Point2::new(x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_point_construction() {
        let scale = 10.0;
        let point = generate_random_vertex(scale);

        assert!(point.x > 0.0);
        assert!(point.x < scale);
        assert!(point.y > 0.0);
        assert!(point.y < scale);
    }

    #[test]
    fn spade_triangulation_construction() {
        // Spade stuff
        // let mut triangulation: DelaunayTriangulation<_> = DelaunayTriangulation::new();
        // triangulation.insert(Point2::new(0.0, 0.0)).unwrap();
        // triangulation.insert(Point2::new(1.0, 1.0)).unwrap();
        // triangulation.insert(Point2::new(0.5, -1.0)).unwrap();

        let triangulation =
            generate_random_delaunay2(3).expect("Failed to construct triangulation!");

        assert_eq!(triangulation.num_vertices(), 3);
        assert_eq!(triangulation.num_inner_faces(), 1);
        assert_eq!(triangulation.num_undirected_edges(), 3);
    }
}

#[cfg(kani)]
#[cfg(not(tarpaulin_include))]
mod verification {

    use super::*;
    use assert_cmd::assert;

    #[kani::proof]
    fn triangle_contains_vertex_constructed_from() {
        // let a = Point { x: 0.0, y: 0.0 };
        // let b = Point { x: 1.0, y: 0.0 };
        // let c = Point { x: 0.0, y: 1.0 };
        // let triangle = Triangle {
        //     vertices: [a, b, c],
        // };

        let a = Point2::new(0.0, 0.0);
        let b = Point2::new(1.0, 0.0);
        let c = Point2::new(0.0, 1.0);
        let mut triangle: DelaunayTriangulation<Point2<f64>> = DelaunayTriangulation::new();
        triangle.insert(a).unwrap();
        triangle.insert(b).unwrap();
        triangle.insert(c).unwrap();

        // assert!(triangle.contains_vertex(&a));
        // assert!(triangle.contains_vertex(&b));
        // assert!(triangle.contains_vertex(&c));

        assert!(triangle.num_vertices() == 3);
        assert!(triangle.num_inner_faces() == 1);
    }
}
