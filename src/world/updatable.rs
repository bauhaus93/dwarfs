
pub trait Updatable {
    fn tick(&mut self, last_tick: u32);
}
