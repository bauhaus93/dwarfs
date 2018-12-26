use gl::types::GLfloat;

#[derive(Clone)]
pub struct Vertex {
    pos: [GLfloat; 3],
    uv: [GLfloat; 3],
    normal: [GLfloat; 3]
}

impl Vertex {
    pub fn new(pos: [GLfloat; 3], uv: [GLfloat; 3], normal: [GLfloat; 3]) -> Vertex {
        Vertex {
            pos: pos,
            uv: uv,
            normal: normal
        }
    }

    pub fn get_pos(&self) -> &[GLfloat] {
        &self.pos
    }

    pub fn get_uv(&self) -> &[GLfloat] {
        &self.uv
    }

    pub fn get_normal(&self) -> &[GLfloat] {
        &self.normal
    }
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Vertex) -> bool {
        const THRESHOLD: GLfloat = 1e-3;
        self.pos.iter().chain(self.uv.iter()).chain(self.normal.iter())
            .zip(other.pos.iter().chain(other.uv.iter()).chain(other.normal.iter()))
            .all(|(&a, &b)| (a - b).abs() < THRESHOLD)
    }
}

impl Eq for Vertex {}


