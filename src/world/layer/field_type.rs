use world::Direction;

#[derive(Copy, Clone)]
pub enum FieldType {
    CUBE,
    SLOPE(Direction)
}
