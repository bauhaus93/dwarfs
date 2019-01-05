
use super::{ Noise, SimplexNoise };

const DEFAULT_OCTAVES: u8 = 6;
const DEFAULT_ROUGHNESS: f32 = 0.5;
const DEFAULT_SCALE: f32 = 2.5e-3;
const DEFAULT_RANGE: (f32, f32) = (-1., 1.);

pub struct OctavedNoise {
    noise: Box<Noise>,
    octaves: u8,
    roughness: f32,
    scale: f32,
    range: (f32, f32)
}

impl OctavedNoise {
    pub fn new(noise: Box<Noise>) -> Self {
        Self {
            noise: noise,
            octaves: DEFAULT_OCTAVES,
            roughness: DEFAULT_ROUGHNESS,
            scale: DEFAULT_SCALE,
            range: DEFAULT_RANGE
        }
    }
}

/*
    Octave calculation based on code by
    matheus23 @ http://www.java-gaming.org/index.php?topic=31637.0
*/

impl Noise for OctavedNoise {
    fn get_noise(&self, p: (f32, f32)) -> f32 {
        let mut sum: f32 = 0.;
        let mut freq = self.scale;
        let mut weight: f32 = 1.;
        let mut weight_sum: f32 = 0.;

        for _oct in 0..self.octaves {
            sum += self.noise.get_noise((p.0 * freq, p.1 * freq)) * weight;
            freq *= 2.;
            weight_sum += weight;
            weight *= self.roughness;
        }
        let sub_range = self.noise.get_range();
        self.range.0 + (self.range.1 - self.range.0) * (-sub_range.0 + (sum / weight_sum) / (sub_range.1 - sub_range.0))
    }

    fn get_range(&self) -> (f32, f32) {
        self.range
    }
}

impl Default for OctavedNoise {
    fn default() -> Self {
        Self {
            noise: Box::new(SimplexNoise::default()),
            octaves: DEFAULT_OCTAVES,
            roughness: DEFAULT_ROUGHNESS,
            scale: DEFAULT_SCALE,
            range: DEFAULT_RANGE
        }
    }
}
