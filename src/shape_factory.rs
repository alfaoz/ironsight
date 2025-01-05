use crate::geometry::Mesh;

pub struct ShapeFactory;


impl ShapeFactory {
    pub fn create_cube(size: f64) -> Mesh {
        Mesh::create_cube(size)
    }
}
