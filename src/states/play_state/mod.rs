mod camera;
mod input;
mod map;

use self::input::Input;
use self::camera::Camera;
use self::map::Map;
use ggez::graphics::Point2;
use ggez::{graphics, Context, GameResult};
use std::time::Duration;
use ggez::event::{MouseButton, MouseState};
use rand::{Rng, thread_rng};

use crate::states::{Assets, State, Transition};
use std::vec::Vec;

const MAPSIZE_MAX_X: i32 = 250;
const MAPSIZE_MAX_Y: i32 = 250;
const TILESIZE: i32 = 100; // side length of square pngs

pub struct PlayState {
  camera: Camera,
  input: Input,
  map: Map,

}

impl PlayState {
  pub fn new(ctx: &mut Context, assets: &Assets) -> GameResult<Self> {
    let mut map = Map::new();
    let mut camera = Camera::new(ctx);
    let mut input = Input::new();
    Ok( PlayState { camera, input, map } )
  }
}

impl State for PlayState {
  fn update(&mut self, ctx: &mut Context, assets: &Assets, dt: Duration,) -> GameResult<Transition> {
    Ok(Transition::None)
  }

  fn draw(&mut self, ctx: &mut Context, assets: &mut Assets) -> GameResult<()> {
    let coords = graphics::get_screen_coordinates(ctx);
    let scale: Point2 = Point2::new(self.camera.zoomlevel, self.camera.zoomlevel);
    let camx = self.camera.position.x as f32;
    let camy = self.camera.position.y as f32;


    for x in 0..MAPSIZE_MAX_X {
      for y in 0..MAPSIZE_MAX_Y {
        let p = graphics::DrawParam {
          dest: Point2::new(
                ((x * TILESIZE) as f32 * self.camera.zoomlevel) + camx, 
                ((y * TILESIZE) as f32 * self.camera.zoomlevel) + camy),
          scale: scale,
          ..Default::default()
        };
        match self.map.tilemap.get((x + (y * MAPSIZE_MAX_X)) as usize) {
          Some(i) => {
            let c = format!("grass{}", &i.id);
            assets.draw_image(&c, p);
//            assets.get_image(&c)?.add(p);
//            graphics::draw_ex(ctx, assets.get_image(&c)?, p);
          }
          _ => {},
        }
      }
    }

    Ok(())
  }

  fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: i32, y: i32) {
    match button {
      MouseButton::Left => {
        self.input.mousedown(1);
        let (a, b) = self.camera.mouse_to_tile(x, y);
        self.camera.tile_to_screen(a, b);
      }
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
    self.input.setpos(x, y);
  }
  
  fn mouse_wheel_event(&mut self, ctx: &mut Context, _x: i32, y: i32) {
    let p = Point2::new(self.input.x as f32, self.input.y as f32);
    self.camera.zoom(y, p);
  }
}
