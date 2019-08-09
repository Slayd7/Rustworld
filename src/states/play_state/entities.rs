use super::TILESIZE;
use super::camera::Camera;
use crate::states::Assets;
use ggez::graphics::{ DrawParam, Point2 };
use super::map::{Pos, Map};

pub trait Entity {
  fn new(id: u32, x: i32, y: i32, s: f32) -> Self;
  fn getoccupiedtile(&self) -> (i32, i32); // Get tile position
  fn getid(&self) -> u32;
  fn getposition(&self) -> (f32, f32); // Get map position (where map position = tile position * TILESIZE)
   
}

pub struct Tile {
  pub id: u32,
  pub scrx: f32,
  pub scry: f32,
  x: i32,
  y: i32,
  pub movecost: usize,
}

impl Tile {
  pub fn setmovecost(&mut self, cost: usize) {
    self.movecost = cost;
  }
}

impl Entity for Tile {
  fn new(id: u32, x: i32, y: i32, s: f32) -> Self { 
    Tile { id: id, x: x, y: y,
      scrx: TILESIZE as f32 * s,
      scry: TILESIZE as f32 * s,
      movecost: 1 as usize,
    } }
  fn getoccupiedtile(&self) -> (i32, i32) { (self.x, self.y) }
  fn getid(&self) -> u32 { self.id }
  fn getposition(&self) -> (f32, f32) { (self.scrx, self.scry) }
}


pub struct Actor {
  id: u32,
  scrx: f32,
  scry: f32,
  x: i32,
  y: i32,
  pub speed: f32,
  moving: bool,
  steps: Vec::<(i32, i32)>,
}

impl Entity for Actor {
  fn new(id: u32, x: i32, y: i32, s: f32) -> Self {
    let mut scrx = TILESIZE as f32 * x as f32;
    let mut scry = TILESIZE as f32 * y as f32;
    let mut s = 0.5;
    let mut m = false;
    let mut a = 0;
    let mut b = 0;
    let mut st = Vec::new();
    Actor { id: id, 
            scrx: scrx, 
            scry: scry,
            x: x,
            y: y,
            speed: s,
            moving: m,
            steps: st }
  }

  fn getid(&self) -> u32 { self.id }

  fn getoccupiedtile(&self) -> (i32, i32) {
    (self.scrx as i32 / TILESIZE, self.scry as i32 / TILESIZE)
  }
  fn getposition(&self) -> (f32, f32) { (self.scrx, self.scry) }
}

impl Actor {
  pub fn update(&mut self, deltaT: u32) {
    if self.moving {
        let a = self.steps.first();
        let (x, y) = a.unwrap();
        self.movestep(*x, *y, deltaT);
      }

  }


  /// Interprets grid tiles in x, y into map pixel coordinates and moves one step towards it
  fn movestep(&mut self, x: i32, y: i32, deltaT: u32) -> bool {
    let x = (x * TILESIZE) as f32;
    let y = (y * TILESIZE) as f32;
    let destx = x;
    let desty = y;


    let (x, y) = (-(self.scrx - x as f32), -(self.scry - y as f32));
    let mut a = Entities::normalize_withspeed(self, x, y);
    let (x, y) = a; 
    
    let x = self.scrx + (x * deltaT as f32) ;
    let y = self.scry + (y * deltaT as f32) ;


    self.scrx = x; 
    self.scry = y;
    if (self.scrx as i32 - destx as i32).abs() <= 2 &&
      (self.scry as i32 - desty as i32).abs() <= 2 {

      self.steps.remove(0);
      if self.steps.is_empty() { self.moving = false; }
    }

    let (x,y) = self.getoccupiedtile();
    self.x = x;
    self.y = y;
    true
  }

/// Set move target for actor in grid tiles (x, y)
  pub fn setmovetarget(&mut self, x: i32, y: i32, cam: &mut Camera, map: &mut Map) -> bool {
    if self.x == x && self.y == y { return false; }
    let steps = map.getpath(Pos(self.x, self.y), Pos(x, y));
    match steps {
      Ok(mut steps) => {
        self.steps.clear();
        self.moving = true;
        steps.remove(0);
        for s in steps {
          let Pos(x, y) = s;
          self.steps.push((x, y));
        }
        true
        }
      Err(e) => { false },
    }
  }

  pub fn clearmovetarget(&mut self) {
    self.steps.clear();
    self.steps.push(self.getoccupiedtile());
  }
}

pub struct Entities {
  tiles: Vec<Tile>,
  actors: Vec<Actor>,
}

impl Entities {
  pub fn new() -> Self {
    Entities { tiles: Vec::new(), actors: Vec::new(), }
  }

  pub fn add_tile(&mut self, tile: Tile) {
    self.tiles.push(tile);
  }

  pub fn add_actor(&mut self, act: Actor) {
    self.actors.push(act);
  }

  pub fn get_actor(&mut self) -> &mut Actor {
    self.actors.first_mut().unwrap()

  }

  pub fn update(&mut self, deltaT: u32, tsize: f32) {
    for v in self.actors.iter_mut() {
      v.update(deltaT);
    }
  }

  pub fn normalize_withspeed(a: &mut Actor, x: f32, y: f32) -> (f32, f32) {
    let s = (((x * x) + (y * y))).sqrt();

    ((x / s) * a.speed, (y / s) * a.speed)
  }
  
  pub fn draw(&mut self, camx: i32, camy: i32, scale: Point2, assets: &mut Assets) {
    for v in self.tiles.iter_mut() {
      let p = DrawParam {
        dest: Point2::new(camx as f32 + (v.scrx * scale.x), camy as f32 + (v.scry * scale.y)),
        scale: scale,
        ..Default::default()
      };
      assets.draw_image(&v.id, p);
    } 
    for v in self.actors.iter_mut() {
      let p = DrawParam {
        dest: Point2::new(camx as f32 + (v.scrx * scale.x), camy as f32 + (v.scry * scale.y)),
        scale: scale,
        ..Default::default()
      };
      assets.draw_actor_image(&v.id, p);
    } 
  }

}

#[cfg(test)]
mod tests {
  use super::*;

#[test]
  fn test_normalize() {
    let a = Entities::normalize(5555, 0);
    assert_eq!(a, (1.0, 0.0));
  }
}
