extern crate gl;

#[macro_use]
extern crate glium;

pub struct Vector3 {
    x: f32,
    y: f32,
    z: f32
}

impl From<Vector3> for [f32; 3] {
    fn from(value: Vector3) -> Self {
        [value.x, value.y, value.z]
    }
}

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}

implement_vertex!(Vertex, position, normal);

impl Vertex {
    pub fn from_position(position: Vector3, normal: Vector3) -> Self {
        Self {
            position: position.into(),
            normal: normal.into()
        }
    }
}
