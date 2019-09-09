use super::{TILESIZE, MAPSIZE_MAX_X} ;
use super::camera::Camera;
use crate::states::Assets;
use ggez::graphics::*;
use ggez::graphics::line;
use super::map::{Pos, Map};
use std::collections::HashMap;
use bresenham::Bresenham;

pub trait BuildableEntity: Buildable + Entity {}

pub trait Entity {
  fn getoccupiedtile(&self) -> (i32, i32); // Get tile position
  fn getid(&self) -> u32;
  fn getposition(&self) -> (f32, f32); // Get map position (where map position = tile position * TILESIZE)
  fn getrotation(&self) -> f32;
  fn getdrawparams(&self, camx: f32, camy: f32, scale: Point2) -> DrawParam { 
    let (x, y) = self.getposition();
    let p = DrawParam {
      dest: Point2::new(-camx as f32 + (x * scale.x ), -camy as f32 + (y * scale.y )),
      scale: scale,
      rotation: self.getrotation(),
      ..Default::default()
    };
    p
  }
}

pub struct Tile {
  pub id: u32,
  pub alt: u32,
  pub scrx: f32,
  pub scry: f32,
  x: i32,
  y: i32,
  pub movecost: usize,  // dijkstra weight
}

impl Tile {
  pub fn new(id: u32, x: i32, y: i32, s: f32) -> Self { 
    Tile { id: id, alt: 0 as u32, x: x, y: y,
      scrx: (TILESIZE * x) as f32 * s,
      scry: (TILESIZE * y) as f32 * s,
      movecost: 1 as usize,
    } }
  pub fn setalternate(&mut self, alt: &u32) { self.alt = *alt; }
  pub fn setmovecost(&mut self, cost: usize) { self.movecost = cost; }
  pub fn getmovecost(&self) -> usize { self.movecost }
}

impl Entity for Tile {
  fn getoccupiedtile(&self) -> (i32, i32) { (self.x, self.y) }
  fn getid(&self) -> u32 { self.id }
  fn getposition(&self) -> (f32, f32) { (self.scrx, self.scry) }
  fn getrotation(&self) -> f32 { 0.0 }
}

pub trait Buildable {
  fn setentityid(&mut self, i: u64);
  fn getentityid(&self) -> u64;
  fn getmovecost(&self) -> usize;
}

impl PartialEq for Buildable {
  fn eq(&self, other: &Self) -> bool {
    self.getentityid() == other.getentityid()
  }
}
  
#[derive(Copy, Clone)]
pub struct Wall {
  pub id: u32,
  pub scrx: f32,
  pub scry: f32,
  pub rotation: f32,
  x: i32,
  y: i32,
  pub crossable: bool,  // so we can use this for short barriers, doors, etc
  pub movecost: usize,  // dijkstra weight
  entityid: u64,
}

impl Wall {
  pub fn new(id: u32, x: i32, y: i32, s: f32, e: u64) -> Self {
    Wall {id: id, x: x, y: y,
      scrx: (TILESIZE * x) as f32 * s,
      scry: (TILESIZE * y) as f32 * s,
      rotation: 0.0,
      crossable: false,
      movecost: usize::max_value(),
      entityid: e,
    }
  }
}

impl Buildable for Wall {
  fn setentityid(&mut self, i: u64) { self.entityid = i; }
  fn getentityid(&self) -> u64 { self.entityid }
  fn getmovecost(&self) -> usize { self.movecost }
}

impl Buildable for &mut Wall {
  fn getentityid(&self) -> u64 { self.entityid }
  fn setentityid(&mut self, i: u64) { self.entityid = i; }
  fn getmovecost(&self) -> usize { self.movecost }
}

impl BuildableEntity for Wall {}

impl Entity for Wall {
  fn getoccupiedtile(&self) -> (i32, i32) { (self.x, self.y) }
  fn getid(&self) -> u32 { self.id }
  fn getposition(&self) -> (f32, f32) { (self.scrx, self.scry) }
  fn getrotation(&self) -> f32 { self.rotation }
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
  fn getid(&self) -> u32 { self.id }
  fn getoccupiedtile(&self) -> (i32, i32) { (self.scrx as i32 / TILESIZE, self.scry as i32 / TILESIZE) }
  fn getposition(&self) -> (f32, f32) { (self.scrx, self.scry) }
  fn getrotation(&self) -> f32 { 0.0 }
}

impl Actor {
  pub fn new(id: u32, x: i32, y: i32, s: f32) -> Self {
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

  pub fn update(&mut self, deltaT: u32) {
    if self.moving {
        let a = self.steps.first();
        let (x, y) = a.unwrap();
        self.movestep(*x, *y, deltaT);
      }

  }

  pub fn lineofsight_vis(&mut self, x: i32, y: i32, map: &mut Map) -> bool {
    for (x, y) in Bresenham::new((self.x as isize, self.y as isize), (x as isize, y as isize)) {
      
      

    }
    true
  }

  pub fn lineofsight_mov(x1: i32, y1: i32, x: i32, y: i32, costmap: &mut Vec<usize>) -> bool {
    for (x, y) in Bresenham::new((x1 as isize, y1 as isize), (x as isize, y as isize)) {
      match costmap.get((x + (y * MAPSIZE_MAX_X as isize)) as usize) {
        Some(&a) => { if a == usize::max_value() { return false } }
        _ => { return false }
      }
    }
    true

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
    let mut steps;
    if self.moving {
      let i = self.steps[0];
      steps = map.getpath(Pos(i.0, i.1), Pos(x, y));
    } else {
      steps = map.getpath(Pos(self.x, self.y), Pos(x, y));
    }
    
    match steps {
      Ok(mut steps) => {
        self.steps.clear();
        self.moving = true;
        let mut j = 0;
        let mut i = (self.x, self.y);
        let mut x1 = steps[0].0;
        let mut y1 = steps[0].1;
        steps.remove(0);
        
        for s in steps {
          let Pos(x, y) = s;
          if Actor::lineofsight_mov(x1, y1, x, y, &mut map.costmap){
            i = (x, y);
          } else {
            self.steps.push(i);
            x1 = i.0;
            y1 = i.1;
          }
          j = j + 1;
          if j % 5 == 0 {
            self.steps.push(i);
            x1 = i.0;
            y1 = i.1;
          }
        }
        
        self.steps.push((x, y));
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

struct UI {
  lines: Vec<((f32, f32), (f32, f32))>,
}

impl UI {
  pub fn new() -> Self {
    UI { lines: Vec::new(), }
  }
  pub fn addline(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
    self.lines.push(((x1, y1), (x2, y2)));
  }
  pub fn clearlines(&mut self) {
    self.lines.clear();
  }
}

pub struct Entities {
  tiles: Vec<Tile>,
  buildings: HashMap<u64, Box<BuildableEntity>>,
  actors: Vec<Actor>,
  entityindex: u64,
  UI: UI,
}

impl Entities {
  pub fn new() -> Self {
    let mut e: u64 = 0;
    Entities { tiles: Vec::new(), buildings: HashMap::new(), actors: Vec::new(), entityindex: e, UI: UI::new(), }
  }

  pub fn getindex(&self) -> u64 { self.entityindex }

  pub fn add_tile(&mut self, tile: Tile) {
    self.tiles.push(tile);
  }

  pub fn add_actor(&mut self, act: Actor) {
    self.actors.push(act);
  }

//TEMPORARY
  pub fn get_actor(&mut self) -> &mut Actor {
    self.actors.first_mut().unwrap()

  }

  pub fn add_building<T: BuildableEntity + 'static> (&mut self, mut bldg: T) {
    bldg.setentityid(self.entityindex);
    let mut b = Box::new(bldg);
    b.setentityid(self.entityindex);
    self.buildings.insert(self.entityindex, b);
    self.entityindex = self.entityindex + 1;
  }

  pub fn remove_building (&mut self, id: u64) {
    self.buildings.remove(&id);
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
    let lineoffsetx = (TILESIZE / 2) as f32 * scale.x;
    let lineoffsety = (TILESIZE / 2) as f32 * scale.y;
    for v in self.tiles.iter_mut() {
      let p = &v.getdrawparams(camx as f32, camy as f32, scale);
      assets.draw_image(&v.id, *p);
    } 
    for v in self.buildings.iter_mut() {
      let (a, b) = v;
      let p = &b.getdrawparams(camx as f32, camy as f32, scale);
      assets.draw_building_image(&b.getid(), *p);
    }
    for v in self.actors.iter_mut() {
      let p = &v.getdrawparams(camx as f32, camy as f32, scale);
      if v.moving {
        let mut p1 = v.getposition();
        p1 = (((p1.0 * scale.x) - camx as f32) + lineoffsetx,
              ((p1.1 * scale.y) - camy as f32) + lineoffsety);
        let mut p2 = (0.0, 0.0);
        for s in v.steps.iter_mut() {
          p2 = ((((s.0 * TILESIZE ) as f32 * scale.x) - camx as f32) + lineoffsetx, (((s.1 * TILESIZE) as f32 * scale.y) - camy as f32) + lineoffsety);
          assets.draw_UI_line((p1, p2));
          p1 = p2;
          

        }

      }
      assets.draw_actor_image(&v.id, *p);
    } 
    for v in self.UI.lines.iter_mut() {
      assets.draw_UI_line(*v);
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
