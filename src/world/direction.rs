use std::collections::BTreeMap;
use std::cmp::Ordering;

#[derive(Copy, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
    Up,
    Down
}

lazy_static! {
    pub static ref DIRECTION_VECTOR: BTreeMap<Direction, [i32; 3]> = {
        let mut map = BTreeMap::new();
        map.insert(Direction::North, [0, -1, 0]);
        map.insert(Direction::East, [1, 0, 0]);
        map.insert(Direction::South, [0, 1, 0]);
        map.insert(Direction::West, [-1, 0, 0]);
        map.insert(Direction::Up, [0, 0, 1]);
        map.insert(Direction::Down, [0, 0, -1]);
        map
    };
}

impl Ord for Direction {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.clone() as u8, other.clone() as u8) {
            (rhs, lhs) if rhs < lhs => Ordering::Less,
            (rhs, lhs) if rhs == lhs => Ordering::Equal,
            (rhs, lhs) => Ordering::Greater
        }
    }
}

impl PartialOrd for Direction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Direction {
    fn eq(&self, other: &Self) -> bool {
        self.clone() as u8 == other.clone() as u8
    }
}

impl Eq for Direction {}
