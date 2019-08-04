mod camera;

use self::camera::Camera;
use ggez::graphics::Point2;
use ggez::{graphics, Context, GameResult};
use std::time::Duration;
use ggez::event::{MouseButton, MouseState};
use rand::{Rng, thread_rng};

use crate::states::{Assets, State, Transition};
use std::vec::Vec;

const MAPSIZE_MAX_X: i32 = 50;
const MAPSIZE_MAX_Y: i32 = 50;

pub struct PlayState {
  spritemap: Vec<u32>,
  camera: Camera,

}

impl PlayState {
  pub fn new(ctx: &mut Context, assets: &Assets) -> GameResult<Self> {
    let mut spritemap = Vec::new();
    for x in 0..MAPSIZE_MAX_X {
      for y in 0..MAPSIZE_MAX_Y {
        let mut rng = thread_rng();
        let x: u32 = rng.gen();
        spritemap.push(x % 3);
      }
    }
    let mut cam = Camera::new();
    Ok( PlayState { spritemap, camera: cam } )
  }
}

impl State for PlayState {
  fn update(&mut self, ctx: &mut Context, assets: &Assets, dt: Duration,) -> GameResult<Transition> {
    Ok(Transition::None)
  }

  fn draw(&mut self, ctx: &mut Context, assets: &Assets) -> GameResult<()> {
    let coords = graphics::get_screen_coordinates(ctx);
    for x in 0..MAPSIZE_MAX_X {
      for y in 0..MAPSIZE_MAX_Y {
        match self.spritemap.get((x + (y * MAPSIZE_MAX_X)) as usize) {
          Some(i) => {
            let c = format!("grass{}", &i);
            graphics::draw(ctx, assets.get_image(&c)?, Point2::new(x as f32 * 100.0, y as f32 * 100.0), 0.0)?;
          }
          _ => {},
        }
      }
    }

    Ok(())
  }
}
