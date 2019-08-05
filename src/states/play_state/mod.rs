mod camera;
mod input;
mod map;
mod entities;

use self::input::Input;
use self::camera::Camera;
use self::map::Map;
use self::entities::{ Entities, Entity };
use ggez::graphics::Point2;
use ggez::{graphics, Context, GameResult};
use std::time::Duration;
use ggez::event::{MouseButton, MouseState};
use rand::{Rng, thread_rng};

use crate::states::{Assets, State, Transition};
use std::vec::Vec;

const MAPSIZE_MAX_X: i32 = 30;
const MAPSIZE_MAX_Y: i32 = 30;
const TILESIZE: i32 = 100; // side length of square pngs

pub struct PlayState {
  camera: Camera,
  input: Input,
  map: Map,
  entities: Entities,

}

impl PlayState {
  pub fn new(ctx: &mut Context, assets: &Assets) -> GameResult<Self> {
    let mut map = Map::new();
    let mut camera = Camera::new(ctx);
    let mut input = Input::new();
    let mut entities = Entities::new();
    let e = Entity::new(50, 50, 50);
    entities.add_entity(e);
    Ok( PlayState { camera, input, map, entities } )
  }
}

impl State for PlayState {
  fn update(&mut self, ctx: &mut Context, assets: &Assets, dt: Duration,) -> GameResult<Transition> {
    self.entities.update(1000 / ggez::timer::get_delta(ctx).subsec_millis());
    Ok(Transition::None)
  }

  fn draw(&mut self, ctx: &mut Context, assets: &mut Assets) -> GameResult<()> {
//    let coords = graphics::get_screen_coordinates(ctx);
    let scale: Point2 = Point2::new(self.camera.zoomlevel, self.camera.zoomlevel);
    let camx = self.camera.position.x as i32;
    let camy = self.camera.position.y as i32;
    let tsize = (TILESIZE as f32 * self.camera.zoomlevel).ceil() ;

    let mut xdrawmin = (-camx / tsize as i32);
    if xdrawmin < 0 { xdrawmin = 0; }
    let mut xdrawmax = xdrawmin + (ctx.conf.window_mode.width as i32 / tsize as i32) + 2;
    if xdrawmax >= MAPSIZE_MAX_X { xdrawmax = MAPSIZE_MAX_X - 1; }

    let mut ydrawmin = (-camy / tsize as i32);
    if ydrawmin < 0 { ydrawmin = 0; }
    let mut ydrawmax = ydrawmin + (ctx.conf.window_mode.height as i32 / tsize as i32) + 2;
    if ydrawmax >= MAPSIZE_MAX_Y { ydrawmax = MAPSIZE_MAX_Y - 1; }

    for x in xdrawmin..xdrawmax {
      for y in ydrawmin..ydrawmax {
        //let newx = (((x * TILESIZE) as f32 * self.camera.zoomlevel) as i32 + camx) as f32;
        //let newy = (((y * TILESIZE) as f32 * self.camera.zoomlevel) as i32 + camy) as f32;

        //if newx < -tsize || newx > ctx.conf.window_mode.width as f32 { continue; }
        //if newy < -tsize || newy > ctx.conf.window_mode.height as f32 { continue; }

        let p = graphics::DrawParam {
          dest: Point2::new(
                ((x * TILESIZE) as f32 * self.camera.zoomlevel) + camx as f32, 
                ((y * TILESIZE) as f32 * self.camera.zoomlevel) + camy as f32),
          scale: scale,
          ..Default::default()
        };
        match self.map.tilemap.get((x + (y * MAPSIZE_MAX_X)) as usize) {
          Some(i) => {
            assets.draw_image(&i.id, p);
          }
          _ => {},
        }
      }
    }
    self.entities.draw(camx, camy, scale, assets);

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
    if m_state.left() {
      self.camera.movestep(dx, dy);
    }
    self.input.setpos(x, y);
  }
  
  fn mouse_wheel_event(&mut self, ctx: &mut Context, _x: i32, y: i32) {
    let p = Point2::new(self.input.x as f32, self.input.y as f32);
    self.camera.zoom(y, p);
  }
}
