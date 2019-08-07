use super::{MAPSIZE_MAX_Y, MAPSIZE_MAX_X};
use super::entities::{Entity, Tile};
use rand::{Rng, thread_rng};
use ggez::nalgebra as na;
use noise::{ NoiseFn, Perlin };
use noise::Seedable;

const NOISESCALE: f64 = 0.05;

pub struct Map {
  pub tilemap: Vec<Tile>,
}

impl Map {
  pub fn new() -> Self {
    let mut tilemap = Vec::new();
    let mut map = Map { tilemap };
    map.generate_map(1000);

    map

  }

  fn generate_map(&mut self, seed: u32) {
    let mut perlin = Perlin::new();
    //perlin.set_seed(seed);
    for x in 0..MAPSIZE_MAX_X {
      for y in 0..MAPSIZE_MAX_Y {
        let val: f64 = perlin.get([x as f64 * NOISESCALE, y as f64 * NOISESCALE, 0.2 * NOISESCALE]);
        let val = val + 1.0;
        let val = val * 0.5;  // Now between 0...1
        let val = Map::island_mask(val, x, y);

        let t = Map::generate_tile(val, x, y);
        self.tilemap.push(t);
      }
    }
  }

  fn island_mask(val: f64, x: i32, y: i32) -> f64 {
    let x = x as f64;
    let y = y as f64;
    let x = (x - MAPSIZE_MAX_X as f64 * 0.5).abs();
    let y = (y - MAPSIZE_MAX_Y as f64 * 0.5).abs();
    let x = x / (MAPSIZE_MAX_X as f64 * 0.5);
    let y = y / (MAPSIZE_MAX_Y as f64 * 0.5);
    let mut d = x;
    if d < y { d = y; }

    (1.0 - d) * val
  }

  fn generate_tile(val: f64, x: i32, y: i32) -> Tile {
    let mut m: u32;
    match val {
      0.0...0.07 => m = 3, // Water
      0.07...0.5 => m = 0, // Grass
      0.5...0.7 => m = 1,  // Grass
      0.7...1.0 => m = 2, // Grass
      _ => m = 0,
    }
    Tile::new(m, x, y, 1.0)
  }
}
