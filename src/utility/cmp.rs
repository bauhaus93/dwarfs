use std::cmp::Ordering;

use glm::Vector3;
use gl::types::GLfloat;

pub fn cmp_vec(lhs: &Vector3<GLfloat>, rhs: &Vector3<GLfloat>) -> Ordering {
    const THRESHOLD: GLfloat = 1e-3;
    for i in 0..3 {
        let diff = lhs[i] - rhs[i];
        if diff < -THRESHOLD {
            return Ordering::Less;
        } else if diff > THRESHOLD {
            return Ordering::Greater;
        }
    }
    Ordering::Equal
}