use super::{MAPSIZE_MAX_Y, MAPSIZE_MAX_X};
use rand::{Rng, thread_rng};

pub trait GameObj {
  fn new(id: u32) -> Self;
}

pub struct Tile {
  pub id: u32,
}

impl GameObj for Tile {
  fn new(id: u32) -> Self {
    Tile { id }
  }
}

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

  pub fn new_seeded(seed: i32) -> Self {

    let mut tilemap = Vec::new();
    Map { tilemap }
  }

  fn generate_map(&mut self) {
    for x in 0..MAPSIZE_MAX_X {
      for y in 0..MAPSIZE_MAX_Y {
        let mut rng = thread_rng();
        let r: u32 = rng.gen();
        let t = Tile { id: r % 3 };
        self.tilemap.push(t);
      }
    }

  }


}
