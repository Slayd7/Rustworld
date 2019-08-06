use super::{MAPSIZE_MAX_Y, MAPSIZE_MAX_X};
use super::entities::{Entity, Tile};
use rand::{Rng, thread_rng};
use ggez::nalgebra as na;

pub struct Map {
  pub tilemap: Vec<Tile>,
}

impl Map {
  pub fn new() -> Self {
    let mut tilemap = Vec::new();
    let mut map = Map { tilemap };
    map.generate_map();

    map

  }

  fn generate_map(&mut self) {
    for x in 0..MAPSIZE_MAX_X {
      for y in 0..MAPSIZE_MAX_Y {
        let mut rng = thread_rng();
        let r: u32 = rng.gen();
        let t = Tile::new(r % 3, x, y, 1.0);

        self.tilemap.push(t);
      }
    }
  }
}
