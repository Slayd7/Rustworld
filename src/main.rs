#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
extern crate ggez;
extern crate rand;
use ggez::*;
use rand::{thread_rng, Rng};

mod rw_tile_lib;
mod rw_import;

struct State {
  shapes: Vec<Shape>,
}

enum Shape {
  Circle(graphics::Point2, f32),
  Rectangle(graphics::Rect),
}

impl ggez::event::EventHandler for State {
  fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
    Ok(())
  }
  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    graphics::clear(ctx);
    for shape in &self.shapes {
      match shape {
        &Shape::Rectangle(rect) => {
          graphics::rectangle(ctx, graphics::DrawMode::Fill, rect).unwrap();
        }
        &Shape::Circle(origin, radius) => {
          graphics::circle(ctx, graphics::DrawMode::Fill, origin, radius, 0.1).unwrap();
        }
      }
    }
    graphics::present(ctx);
    Ok(())
  }
}

fn main() {
  let mut shapes = Vec::new();
  for _ in 0..8 {
    if thread_rng().gen_range(0, 2) % 2 == 0 {
      shapes.push(Shape::Rectangle(ggez::graphics::Rect::new(
        thread_rng().gen_range(0.0, 800.0),
        thread_rng().gen_range(0.0, 600.0),
        thread_rng().gen_range(0.0, 800.0),
        thread_rng().gen_range(0.0, 600.0),
      )));
    } else {
      shapes.push(Shape::Circle(
        ggez::graphics::Point2::new(
          thread_rng().gen_range(0.0, 800.0),
          thread_rng().gen_range(0.0, 600.0),
      ),
      thread_rng().gen_range(0.0, 300.0),
    
  ));
    }
  }


  let state = &mut State { shapes: shapes };
  let c = conf::Conf::new();
  let ctx = &mut Context::load_from_conf("title", "author", c).unwrap();
  event::run(ctx, state).unwrap();
}
