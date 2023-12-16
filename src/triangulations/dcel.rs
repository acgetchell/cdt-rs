// Define a struct to represent a vertex
#[derive(Debug)]
struct Vertex {
    id: usize,
    x: f64,
    y: f64,
}

// Define a struct to represent a half-edge
#[derive(Debug)]
struct HalfEdge {
    id: usize,
    origin: usize,
    twin: Option<usize>,
    next: Option<usize>,
    prev: Option<usize>,
    face: Option<usize>,
}

// Define a struct to represent a face
#[derive(Debug)]
struct Face {
    id: usize,
    edge: Option<usize>,
}

// Define the DCEL struct
#[derive(Debug)]
struct DCEL {
    vertices: Vec<Vertex>,
    half_edges: Vec<HalfEdge>,
    faces: Vec<Face>,
}

impl DCEL {
    // Add a new vertex to the DCEL
    fn add_vertex(&mut self, x: f64, y: f64) -> usize {
        let id = self.vertices.len();
        self.vertices.push(Vertex { id, x, y });
        id
    }

    // Add a new half-edge to the DCEL
    fn add_half_edge(&mut self, origin: usize) -> usize {
        let id = self.half_edges.len();
        self.half_edges.push(HalfEdge {
            id,
            origin,
            twin: None,
            next: None,
            prev: None,
            face: None,
        });
        id
    }

    // Add a new face to the DCEL
    fn add_face(&mut self) -> usize {
        let id = self.faces.len();
        self.faces.push(Face { id, edge: None });
        id
    }
}

fn main() {
    // Create a new DCEL
    let mut dcel = DCEL {
        vertices: Vec::new(),
        half_edges: Vec::new(),
        faces: Vec::new(),
    };

    // Add some vertices
    let v1 = dcel.add_vertex(0.0, 0.0);
    let v2 = dcel.add_vertex(1.0, 0.0);
    let v3 = dcel.add_vertex(1.0, 1.0);

    // Add some half-edges
    let e1 = dcel.add_half_edge(v1);
    let e2 = dcel.add_half_edge(v2);
    let e3 = dcel.add_half_edge(v3);

    // Set the twin, next, and prev references for the half-edges
    dcel.half_edges[e1].twin = Some(e2);
    dcel.half_edges[e2].twin = Some(e1);
    dcel.half_edges[e1].next = Some(e2);
    dcel.half_edges[e2].prev = Some(e1);
    dcel.half_edges[e2].next = Some(e3);
    dcel.half_edges[e3].prev = Some(e2);
    dcel.half_edges[e3].next = Some(e1);
    dcel.half_edges[e1].prev = Some(e3);

    // Add a face and set the edge reference
    let f1 = dcel.add_face();
    dcel.faces[f1].edge = Some(e1);

    // Print the DCEL
    println!("{:#?}", dcel);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_vertex() {
        let mut dcel = DCEL {
            vertices: Vec::new(),
            half_edges: Vec::new(),
            faces: Vec::new(),
        };
        let v1 = dcel.add_vertex(0.0, 0.0);
        assert_eq!(dcel.vertices[v1].id, 0);
        assert_eq!(dcel.vertices[v1].x, 0.0);
        assert_eq!(dcel.vertices[v1].y, 0.0);
    }

    #[test]
    fn test_add_half_edge() {
        let mut dcel = DCEL {
            vertices: Vec::new(),
            half_edges: Vec::new(),
            faces: Vec::new(),
        };
        let v1 = dcel.add_vertex(0.0, 0.0);
        let e1 = dcel.add_half_edge(v1);
        assert_eq!(dcel.half_edges[e1].id, 0);
        assert_eq!(dcel.half_edges[e1].origin, 0);
        assert_eq!(dcel.half_edges[e1].twin, None);
        assert_eq!(dcel.half_edges[e1].next, None);
        assert_eq!(dcel.half_edges[e1].prev, None);
        assert_eq!(dcel.half_edges[e1].face, None);
    }

    #[test]
    fn test_add_face() {
        let mut dcel = DCEL {
            vertices: Vec::new(),
            half_edges: Vec::new(),
            faces: Vec::new(),
        };
        let f1 = dcel.add_face();
        assert_eq!(dcel.faces[f1].id, 0);
        assert_eq!(dcel.faces[f1].edge, None);
    }
}
