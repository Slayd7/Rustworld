mod camera;
mod input;

use self::input::Input;
use self::camera::Camera;
use ggez::graphics::Point2;
use ggez::{graphics, Context, GameResult};
use std::time::Duration;
use ggez::event::{MouseButton, MouseState};
use rand::{Rng, thread_rng};

use crate::states::{Assets, State, Transition};
use std::vec::Vec;

const MAPSIZE_MAX_X: i32 = 10;
const MAPSIZE_MAX_Y: i32 = 10;

pub struct PlayState {
  spritemap: Vec<u32>,
  camera: Camera,
  input: Input,

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
    let mut camera = Camera::new();
    let mut input = Input::new();
    Ok( PlayState { spritemap, camera, input } )
  }
}

impl State for PlayState {
  fn update(&mut self, ctx: &mut Context, assets: &Assets, dt: Duration,) -> GameResult<Transition> {
    Ok(Transition::None)
  }

  fn draw(&mut self, ctx: &mut Context, assets: &Assets) -> GameResult<()> {
    let coords = graphics::get_screen_coordinates(ctx);
    let scale: Point2 = Point2::new(self.camera.zoomlevel, self.camera.zoomlevel);
    let camx = self.camera.position.x as f32;
    let camy = self.camera.position.y as f32;


    for x in 0..MAPSIZE_MAX_X {
      for y in 0..MAPSIZE_MAX_Y {
        let p = graphics::DrawParam {
          dest: Point2::new(
                ((x as f32 * 100.0) + camx) * self.camera.zoomlevel, 
                ((y as f32 * 100.0) + camy) * self.camera.zoomlevel),
          scale: scale,
          ..Default::default()
        };
        match self.spritemap.get((x + (y * MAPSIZE_MAX_X)) as usize) {
          Some(i) => {
            let c = format!("grass{}", &i);
            graphics::draw_ex(ctx, assets.get_image(&c)?, p);
          }
          _ => {},
        }
      }
    }

    Ok(())
  }

  fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: i32, y: i32) {
    match button {
      MouseButton::Left => self.input.mousedown(1),
      MouseButton::Right => self.input.mousedown(2),
      MouseButton::Middle => self.input.mousedown(3),
      _ => {},
    }
    self.input.setpos(x, y);
  }

  fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: i32, y: i32) {
    match button {
      MouseButton::Left => self.input.mouseup(1),
      MouseButton::Right => self.input.mouseup(2),
      MouseButton::Middle => self.input.mouseup(3),
      _ => {},
    }
    self.input.setpos(x, y);
  }

  fn mouse_motion_event(&mut self, ctx: &mut Context, m_state: MouseState, x: i32, y: i32, dx: i32, dy: i32) {
    if m_state.middle() {
      self.camera.movestep(dx, dy);
    }
  }
  
  fn mouse_wheel_event(&mut self, ctx: &mut Context, _x: i32, y: i32) {
    self.camera.zoom(y);
  }
}
