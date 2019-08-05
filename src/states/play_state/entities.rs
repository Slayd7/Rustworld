use super::map::GameObj;
use super::TILESIZE;
use super::camera::Camera;
use crate::states::Assets;
use ggez::graphics::{ DrawParam, Point2 };
use ggez::timer;

pub struct Entity {
  pub id: u32,
  pub x: i32,
  pub y: i32,
}

impl Entity {
  pub fn new(id: u32, x: i32, y: i32) -> Self {
    Entity { id: id, x: x, y: y }
  }

  pub fn getoccupiedtile(&self, cam: Camera) -> (i32, i32) {
    (self.x / (cam.zoomlevel * TILESIZE as f32) as i32, self.y / (cam.zoomlevel * TILESIZE as f32) as i32)
  }
}

pub struct Actor {
  pub entity: Entity,
  pub speed: f32,
  moving: bool,
  dest: (i32, i32),
  steps: Vec::<(i32, i32)>,
}

impl Actor {
  pub fn new(id: u32, x: i32, y: i32) -> Self {
    let mut e = Entity { id: id, x: x, y: y };
    let mut s = 1.0;
    let mut m = false;
    let mut a = 0;
    let mut b = 0;
    let mut st = Vec::new();
    Actor { entity: e, speed: s, moving: m, dest: (a, b), steps: st }
  }

  pub fn movestep(&mut self, x: i32, y: i32, deltaT: u32) -> bool {
    
    let mut a = Entities::normalize(x, y);
    let (a, b) = a; 

    self.entity.x = self.entity.x + ((a as u32 * (1000 / deltaT)) as f32 * self.speed) as i32;
    self.entity.y = self.entity.y + ((b as u32 * (1000 / deltaT))  as f32 * self.speed) as i32;
    if (self.entity.x, self.entity.y) == (x, y) {
      self.steps.pop();
      if self.steps.is_empty() { self.moving = false; }
    }
    true
  }

  pub fn setmovetarget(&mut self, x: i32, y: i32, cam: &mut Camera) -> bool {
    let (nx, ny) = cam.mouse_to_tile(x, y);
    self.dest = (nx, ny);
    self.moving = true;
    true
  }
}

pub struct Entities {
  entities: Vec<Entity>,
  actors: Vec<Actor>,
}

impl Entities {
  pub fn new() -> Self {
    Entities { entities: Vec::new(), actors: Vec::new(), }
  }

  pub fn add_entity(&mut self, ent: Entity) {
    self.entities.push(ent);
  }

  pub fn add_actor(&mut self, act: Actor) {
    self.actors.push(act);
  }

  pub fn update(&mut self, deltaT: u32) {
    for v in self.actors.iter_mut() {
      if v.moving {
        let a = v.steps.first();
        let (x, y) = a.unwrap();
        v.movestep(*x, *y, deltaT);
      }

    }
  }

  pub fn draw(&mut self, camx: i32, camy: i32, scale: Point2, assets: &mut Assets) {
    for v in self.entities.iter_mut() {
      let p = DrawParam {
        dest: Point2::new(camx as f32 + (v.x as f32 * scale.x), camy as f32 + (v.y as f32 * scale.y)),
        scale: scale,
        ..Default::default()
      };
      assets.draw_image(&v.id, p);
    } 
    for v in self.actors.iter_mut() {
      let p = DrawParam {
        dest: Point2::new(camx as f32 + (v.entity.x as f32 * scale.x), camy as f32 + (v.entity.y as f32 * scale.y)),
        scale: scale,
        ..Default::default()
      };
      assets.draw_image(&v.entity.id, p);
    } 
  }

  pub fn normalize(x: i32, y: i32) -> (i32, i32) {
    let s = (((x * x) + (y * y)) as f32).sqrt();
    ((x as f32 / s) as i32, (y as f32 / s) as i32)


  }
}
