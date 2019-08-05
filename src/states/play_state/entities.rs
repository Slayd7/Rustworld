use super::map::GameObj;
use crate::states::Assets;
use ggez::graphics::{ DrawParam, Point2 };

pub struct Entity {
  pub id: u32,
  pub x: i32,
  pub y: i32,
}

impl Entity {
  pub fn new(id: u32, x: i32, y: i32) -> Self {
    Entity { id: id, x: x, y: y }
  }
}

pub struct Entities {
  entities: Vec<Entity>,
}

impl Entities {
  pub fn new() -> Self {
    Entities { entities: Vec::new() }
  }

  pub fn add_entity(&mut self, ent: Entity) {
    self.entities.push(ent);

  }

  pub fn draw(&mut self, camx: i32, camy: i32, scale: Point2, assets: &mut Assets) {
    for v in self.entities.iter_mut() {
      let p = DrawParam {
        dest: Point2::new((camx + v.x) as f32, (camy + v.y) as f32),
        scale: scale,
        ..Default::default()
      };
      assets.draw_image(&v.id, p);
    } 
  }
}
