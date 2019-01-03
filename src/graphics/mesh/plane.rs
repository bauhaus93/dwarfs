use gl::types::GLfloat;

use graphics::GraphicsError;
use super::{ Mesh, MeshBuilder, Vertex, Quad };

const VERTICES: [[GLfloat; 3]; 4] = [
    [-0.5, -0.5, 0.],
    [0.5, -0.5, 0.],
    [0.5, 0.5, 0.],
    [-0.5, 0.5, 0.]
];

const UVS: [[GLfloat; 3]; 4] = [
    [0., 0., 0.],
    [1., 0., 0.],
    [1., 1., 0.],
    [0., 1., 0.]
];
const NORMAL: [GLfloat; 3] = [0., 0., 1.];

pub fn create_plane() -> Result<Mesh, GraphicsError> {
    let mut quad = Quad::default();
    for (index, (vert, uv)) in VERTICES.iter().zip(UVS.iter()).enumerate() {
        let vertex = Vertex::new(*vert, *uv, NORMAL);
        quad.set_vertex(index, vertex);
    }

    let mesh = MeshBuilder::new()
        .add_quad(quad)
        .finish()?;
    
    Ok(mesh)    
}
