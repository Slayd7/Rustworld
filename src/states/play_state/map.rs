use super::{MAPSIZE_MAX_Y, MAPSIZE_MAX_X};
use super::entities::{Entities, Entity, Tile, Wall};
use ggez::GameResult;
use ggez::GameError::ResourceLoadError;
use noise::{ NoiseFn, Perlin };
use noise::Seedable;
use pathfinding::grid::Grid;
use pathfinding::prelude::dijkstra;
use std::time::SystemTime;

const NOISESCALE: f64 = 0.05;

fn getmapvecidx(x: i32, y: i32) -> usize { (x + (MAPSIZE_MAX_X * y)) as usize }

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pos(pub i32, pub i32);

impl Pos { // Pathfinding is expensive :(
  fn successors(&self, costmap: &Vec<usize>) -> Vec<(Pos, usize)> {
    let &Pos(x, y) = self;
    let mut cost: usize = usize::min_value();
    let mut ret: Vec<(Pos, usize)> = Vec::new();
    if x > 0 {
      if y > 0 {
        cost = *costmap.get((x-1 + ((y-1) * MAPSIZE_MAX_Y)) as usize).unwrap();
        // Impassible?
        if cost < usize::max_value() { ret.push((Pos(x-1, y-1), cost)); }
      }
      if y < (MAPSIZE_MAX_Y - 1) {
        cost = *costmap.get((x-1 + ((y+1) * MAPSIZE_MAX_Y)) as usize).unwrap();
        if cost < usize::max_value() { ret.push((Pos(x-1, y+1), cost)); }
      }
      cost = *costmap.get((x-1 + ((y) * MAPSIZE_MAX_Y)) as usize).unwrap();
      if cost < usize::max_value() { ret.push((Pos(x-1, y),  cost)); }
    }
    if x < (MAPSIZE_MAX_X - 1) {
      if y > 0 {
        cost = *costmap.get((x+1 + ((y-1) * MAPSIZE_MAX_Y)) as usize).unwrap();
        if cost < usize::max_value() { ret.push((Pos(x+1, y-1),  cost)); }
      }
      if y < (MAPSIZE_MAX_Y - 1) {
        cost = *costmap.get((x+1 + ((y+1) * MAPSIZE_MAX_Y)) as usize).unwrap();
        if cost < usize::max_value() { ret.push((Pos(x+1, y+1),  cost)); }
      }
      cost = *costmap.get((x+1 + ((y) * MAPSIZE_MAX_Y)) as usize).unwrap();
      if cost < usize::max_value() { ret.push((Pos(x+1, y),  cost)); }
    }
    if y > 0 {
      cost = *costmap.get((x + ((y-1) * MAPSIZE_MAX_Y)) as usize).unwrap();
      if cost < usize::max_value() { ret.push((Pos(x, y-1), cost)); }
    }
    if y < (MAPSIZE_MAX_Y - 1) {
      cost = *costmap.get((x + ((y+1) * MAPSIZE_MAX_Y)) as usize).unwrap();
      if cost < usize::max_value() { ret.push((Pos(x, y+1),  cost)); }
    }
    ret
  }
  
}

pub struct Map {
  pub tilemap: Vec<Tile>,
  pub build_layer: Vec<Option<Wall>>,
  pub costmap: Vec<usize>,

}

impl Map {
  pub fn new() -> Self {
    let mut tilemap = Vec::new();
//    let mut build_layer = Vec::new();
    let mut build_layer = Vec::new();
    let mut costmap = Vec::new();
    let mut map = Map { tilemap, build_layer, costmap };
    let seed = SystemTime::now();
    match seed.duration_since(SystemTime::UNIX_EPOCH) {
      Ok(elapsed) => { map.generate_map(elapsed.as_secs() as u32); }
      Err(e) => { println!("Error: {:?}", e); }
    }
      
    map
  }

  pub fn getpath(&mut self, from: Pos, to: Pos) -> GameResult<Vec<Pos>> {
    let result = dijkstra(&from, |p| p.successors(&self.costmap), |p| *p == to);
    match result {
      Some((result, weight)) => { Ok(result) },
      None => { Err(ggez::GameError::UnknownError("No path found".to_string())) },
    }
  }

  fn generate_map(&mut self, seed: u32) {
    let mut perlin = Perlin::new();
    let mut perlin = perlin.set_seed(seed);
    for x in 0..MAPSIZE_MAX_X {
      for y in 0..MAPSIZE_MAX_Y {
        let val: f64 = perlin.get([x as f64 * NOISESCALE, y as f64 * NOISESCALE, 0.2 * NOISESCALE]);
        let val = val + 1.0;
        let val = val * 0.5;  // Now between 0...1
        let val = Map::island_mask(val, x, y);
        let t = Map::generate_tile(val, x, y);
        self.costmap.push(t.movecost);
        self.tilemap.push(t);
        self.build_layer.push(None);
      }
    }

  }

  pub fn get_tile_at(&self, x: i32, y: i32) -> GameResult<&Tile> {
    if !Map::check_bounds(x, y) {
      return Err(ResourceLoadError("Tile out of bounds".to_string()));
    }
    Ok(self.tilemap.get(getmapvecidx(x, y)).unwrap())
  }

  pub fn set_tile_at(&mut self, x: i32, y: i32, t: Tile) -> GameResult<()> {
    if !Map::check_bounds(x, y) {
         return Err(ResourceLoadError("Tile out of bounds".to_string()));
    }
    let idx = getmapvecidx(x, y);
    self.tilemap.remove(idx);
    self.tilemap.insert(idx, t);
    Ok(())

  }

  pub fn get_wall_at(&mut self, x: i32, y: i32) -> Option<Wall> {
    if !Map::check_bounds(x, y) {
      return None;
    }
    *self.build_layer.get(getmapvecidx(x,y)).unwrap()
  }

  pub fn set_wall_at(&mut self, x: i32, y: i32, w: &mut Wall, entities: &mut Entities) -> GameResult<()> {
    if !Map::check_bounds(x, y) {
      return Err(ResourceLoadError("Tile out of bounds".to_string()));
    }
    let idx = getmapvecidx(x, y);
    self.costmap.remove(idx);
    self.costmap.insert(idx, w.getmovecost());
    entities.add_wall(w);
    self.build_layer.remove(idx);
    self.build_layer.insert(idx, Some(*w));
    Ok(())
  }

  pub fn clear_wall_at(&mut self, x: i32, y: i32, entities: &mut Entities) -> GameResult<()> {
    if !Map::check_bounds(x, y) {
      return Err(ResourceLoadError("Tile out of bounds".to_string()));
    }
    let idx = getmapvecidx(x, y);
    let w = self.build_layer.remove(idx);
    self.build_layer.insert(idx, None);
    self.costmap.remove(idx);
    entities.remove_wall(&w.unwrap());
    self.costmap.insert(idx, self.get_tile_at(x, y).unwrap().getmovecost()); 
    Ok(())
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
    let mut cost = (m + 1) as usize;
    if cost == 4 { cost = usize::max_value(); }
    let mut t = Tile::new(m, x, y, 1.0);
    t.setmovecost(cost);
    t
  }

  fn check_bounds(x: i32, y: i32) -> bool {
    if x < 0 || x >= MAPSIZE_MAX_X ||
       y < 0 || y >= MAPSIZE_MAX_Y {
         return false;
    }
    true
  }
}
