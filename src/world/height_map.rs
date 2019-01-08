use std::collections::HashMap;

use world::noise::Noise;

pub type HeightMap = HashMap<(i32, i32), i32>;

pub fn create_height_map(layer_size: (i32, i32), height_noise: &Noise) -> HeightMap {
    let mut height_map = HeightMap::with_capacity((layer_size.0 * layer_size.1) as usize);
    for y in 0..layer_size.1 {
        for x in 0..layer_size.0 {
            height_map.insert((x, y), height_noise.get_noise((x as f32, y as f32)) as i32);
        }
    }
    height_map
}
