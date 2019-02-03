use std::cmp::Ordering;

use glm::Vector3;

use utility::Float;

pub fn cmp_vec(lhs: &Vector3<Float>, rhs: &Vector3<Float>) -> Ordering {
    const THRESHOLD: Float = 1e-3;
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