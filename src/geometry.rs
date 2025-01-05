use crate::math::{Mat4, Vec2, Vec3};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub uv: Vec2,
}

#[derive(Debug, Clone)]
pub struct Face {
    pub vertices: [usize; 3],  // Indices into vertex array
    pub normal: Vec3,
}

#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub faces: Vec<Face>,
    pub transform: Mat4,
}

#[derive(Debug, Clone)]
pub struct BoundingBox {
    pub min: Vec3,
    pub max: Vec3,
}

impl Vertex {
    pub fn new(position: Vec3, normal: Vec3, uv: Vec2) -> Self {
        Self {
            position,
            normal: normal.normalize(),
            uv,
        }
    }

    pub fn transform(&self, matrix: &Mat4) -> Self {
        Self {
            position: matrix.transform_vec3(&self.position),
            normal: matrix.transform_vec3(&self.normal).normalize(),
            uv: self.uv,
        }
    }
}

impl Face {
    pub fn new(vertices: [usize; 3]) -> Self {
        Self {
            vertices,
            normal: Vec3::zero(), // Will be calculated later
        }
    }

    pub fn calculate_normal(&mut self, vertices: &[Vertex]) {
        let v0 = &vertices[self.vertices[0]].position;
        let v1 = &vertices[self.vertices[1]].position;
        let v2 = &vertices[self.vertices[2]].position;

        let edge1 = *v1 - *v0;
        let edge2 = *v2 - *v0;
        self.normal = edge1.cross(&edge2).normalize();
    }
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            faces: Vec::new(),
            transform: Mat4::identity(),
        }
    }

    pub fn with_capacity(vertex_count: usize, face_count: usize) -> Self {
        Self {
            vertices: Vec::with_capacity(vertex_count),
            faces: Vec::with_capacity(face_count),
            transform: Mat4::identity(),
        }
    }

    pub fn add_vertex(&mut self, vertex: Vertex) -> usize {
        let index = self.vertices.len();
        self.vertices.push(vertex);
        index
    }

    pub fn add_face(&mut self, vertices: [usize; 3]) {
        let mut face = Face::new(vertices);
        face.calculate_normal(&self.vertices);
        self.faces.push(face);
    }

    pub fn transform(&mut self, matrix: Mat4) {
        self.transform = matrix.multiply(&self.transform);
    }

    pub fn get_transformed_vertices(&self) -> Vec<Vertex> {
        self.vertices.iter()
            .map(|v| v.transform(&self.transform))
            .collect()
    }

    pub fn calculate_bounding_box(&self) -> BoundingBox {
        if self.vertices.is_empty() {
            return BoundingBox {
                min: Vec3::zero(),
                max: Vec3::zero(),
            };
        }

        let transformed = self.get_transformed_vertices();
        let mut min = transformed[0].position;
        let mut max = transformed[0].position;

        for vertex in transformed.iter().skip(1) {
            min.x = min.x.min(vertex.position.x);
            min.y = min.y.min(vertex.position.y);
            min.z = min.z.min(vertex.position.z);

            max.x = max.x.max(vertex.position.x);
            max.y = max.y.max(vertex.position.y);
            max.z = max.z.max(vertex.position.z);
        }

        BoundingBox { min, max }
    }

    pub fn generate_vertex_normals(&mut self) {
        // Initialize normal accumulators
        let mut vertex_normals = HashMap::new();
        let mut normal_counts = HashMap::new();

        // Accumulate face normals for each vertex
        for face in &self.faces {
            for &vertex_idx in &face.vertices {
                vertex_normals
                    .entry(vertex_idx)
                    .and_modify(|n: &mut Vec3| *n = *n + face.normal)
                    .or_insert(face.normal);

                *normal_counts
                    .entry(vertex_idx)
                    .or_insert(0) += 1;
            }
        }

        // Average the normals
        for (vertex_idx, normal) in vertex_normals {
            let count = *normal_counts.get(&vertex_idx).unwrap_or(&1) as f64;
            self.vertices[vertex_idx].normal = (normal / count).normalize();
        }
    }
}

// Helper function to create primitive shapes
impl Mesh {
    pub fn create_cube(size: f64) -> Self {
        let mut mesh = Mesh::with_capacity(8, 12);
        let half = size / 2.0;

        // Define vertices with initial normals
        let vertices = vec![
            Vertex::new(
                Vec3::new(-half, -half, half),
                Vec3::new(-1.0, -1.0, 1.0).normalize(),
                Vec2::new(0.0, 0.0)
            ),
            Vertex::new(
                Vec3::new(half, -half, half),
                Vec3::new(1.0, -1.0, 1.0).normalize(),
                Vec2::new(1.0, 0.0)
            ),
            Vertex::new(
                Vec3::new(half, half, half),
                Vec3::new(1.0, 1.0, 1.0).normalize(),
                Vec2::new(1.0, 1.0)
            ),
            Vertex::new(
                Vec3::new(-half, half, half),
                Vec3::new(-1.0, 1.0, 1.0).normalize(),
                Vec2::new(0.0, 1.0)
            ),
            // Back face
            Vertex::new(
                Vec3::new(-half, -half, -half),
                Vec3::new(-1.0, -1.0, -1.0).normalize(),
                Vec2::new(1.0, 0.0)
            ),
            Vertex::new(
                Vec3::new(half, -half, -half),
                Vec3::new(1.0, -1.0, -1.0).normalize(),
                Vec2::new(0.0, 0.0)
            ),
            Vertex::new(
                Vec3::new(half, half, -half),
                Vec3::new(1.0, 1.0, -1.0).normalize(),
                Vec2::new(0.0, 1.0)
            ),
            Vertex::new(
                Vec3::new(-half, half, -half),
                Vec3::new(-1.0, 1.0, -1.0).normalize(),
                Vec2::new(1.0, 1.0)
            ),
        ];

        // Add vertices to mesh
        for vertex in vertices {
            mesh.add_vertex(vertex);
        }

        // Define faces
        let faces = [
            [0, 1, 2], [0, 2, 3],  // Front
            [5, 4, 7], [5, 7, 6],  // Back
            [4, 0, 3], [4, 3, 7],  // Left
            [1, 5, 6], [1, 6, 2],  // Right
            [3, 2, 6], [3, 6, 7],  // Top
            [4, 5, 1], [4, 1, 0],  // Bottom
        ];

        // Add faces to mesh
        for face in faces.iter() {
            mesh.add_face(*face);
        }

        // Calculate proper vertex normals
        mesh.generate_vertex_normals();
        mesh
    }}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_creation() {
        let mut mesh = Mesh::new();

        // Add vertices
        let v1 = mesh.add_vertex(Vertex::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec2::new(0.0, 0.0)
        ));
        let v2 = mesh.add_vertex(Vertex::new(
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec2::new(1.0, 0.0)
        ));
        let v3 = mesh.add_vertex(Vertex::new(
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec2::new(0.0, 1.0)
        ));

        // Add face
        mesh.add_face([v1, v2, v3]);

        assert_eq!(mesh.vertices.len(), 3);
        assert_eq!(mesh.faces.len(), 1);
    }

    #[test]
    fn test_mesh_transformation() {
        let mut cube = Mesh::create_cube(2.0);
        let original_position = cube.vertices[0].position;

        // Apply translation
        cube.transform(Mat4::translation(1.0, 0.0, 0.0));
        let transformed = cube.get_transformed_vertices();

        assert!((transformed[0].position.x - (original_position.x + 1.0)).abs() < 1e-10);
    }

    #[test]
    fn test_bounding_box() {
        let cube = Mesh::create_cube(2.0);
        let bbox = cube.calculate_bounding_box();

        assert!((bbox.min.x + 1.0).abs() < 1e-10);
        assert!((bbox.max.x - 1.0).abs() < 1e-10);
        assert!((bbox.min.y + 1.0).abs() < 1e-10);
        assert!((bbox.max.y - 1.0).abs() < 1e-10);
        assert!((bbox.min.z + 1.0).abs() < 1e-10);
        assert!((bbox.max.z - 1.0).abs() < 1e-10);
    }
}
