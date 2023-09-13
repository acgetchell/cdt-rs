#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(PartialEq, Clone, Copy)]
pub struct Triangle {
    pub vertices: [Point; 3],
}

impl Triangle {
    fn contains_vertex(&self, point: &Point) -> bool {
        self.vertices.contains(point)
    }

    fn circumcircle_contains(&self, point: &Point) -> bool {
        let [pa, pb, pc] = self.vertices;

        let d = 2.0 * (pa.x * (pb.y - pc.y) + pb.x * (pc.y - pa.y) + pc.x * (pa.y - pb.y));
        let ux = ((pa.x * pa.x + pa.y * pa.y) * (pb.y - pc.y)
            + (pb.x * pb.x + pb.y * pb.y) * (pc.y - pa.y)
            + (pc.x * pc.x + pc.y * pc.y) * (pa.y - pb.y))
            / d;
        let uy = ((pa.x * pa.x + pa.y * pa.y) * (pc.x - pb.x)
            + (pb.x * pb.x + pb.y * pb.y) * (pa.x - pc.x)
            + (pc.x * pc.x + pc.y * pc.y) * (pb.x - pa.x))
            / d;
        let _center = Point { x: ux, y: uy };
        let radius = ((pa.x - ux).powi(2) + (pa.y - uy).powi(2)).sqrt();

        let distance = ((point.x - ux).powi(2) + (point.y - uy).powi(2)).sqrt();
        distance < radius
    }
}

pub fn bowyer_watson(points: Vec<Point>) -> Vec<Triangle> {
    let mut triangles = Vec::new();

    // Create a super-triangle that contains all points
    let st = Triangle {
        vertices: [
            Point { x: -1e5, y: -1e5 },
            Point { x: 1e5, y: -1e5 },
            Point { x: 0.0, y: 1e5 },
        ],
    };
    triangles.push(st);

    for point in points.iter() {
        let mut bad_triangles = Vec::new();

        for triangle in &triangles {
            if triangle.circumcircle_contains(point) {
                bad_triangles.push(*triangle);
            }
        }

        let mut polygon = Vec::new();
        for bt in &bad_triangles {
            for i in 0..3 {
                let edge = (bt.vertices[i], bt.vertices[(i + 1) % 3]);
                let is_shared = bad_triangles.iter().any(|ot| {
                    ot != bt && ot.contains_vertex(&edge.0) && ot.contains_vertex(&edge.1)
                });

                if !is_shared {
                    polygon.push(edge);
                }
            }
        }

        triangles.retain(|t| !bad_triangles.contains(t));

        for edge in &polygon {
            triangles.push(Triangle {
                vertices: [edge.0, edge.1, *point],
            });
        }
    }

    // Remove triangles that contain vertices of super-triangle
    triangles.retain(|t| {
        !t.contains_vertex(&st.vertices[0])
            && !t.contains_vertex(&st.vertices[1])
            && !t.contains_vertex(&st.vertices[2])
    });

    triangles
}

#[test]
fn delaunay_triangulation_construction() {
    let points = vec![
        Point { x: 0.0, y: 0.0 },
        Point { x: 1.0, y: 0.0 },
        Point { x: 0.5, y: 1.0 },
    ];

    let triangulation = bowyer_watson(points);

    assert_eq!(triangulation.len(), 1);
    assert!(triangulation.contains(&Triangle {
        vertices: [
            Point { x: 0.0, y: 0.0 },
            Point { x: 1.0, y: 0.0 },
            Point { x: 0.5, y: 1.0 },
        ]
    }));
}
