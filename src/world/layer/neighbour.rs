use world::Direction;

pub const NEIGHBOUR_RELATION: [(Direction, [i32; 2]); 4] = [
    (Direction::NORTH, [0, -1]),
    (Direction::SOUTH, [0, 1]),
    (Direction::EAST, [1, 0]),
    (Direction::WEST, [-1, 0]),
];