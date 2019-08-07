mod camera;
mod input;
mod map;
mod entities;

use self::input::Input;
use self::camera::Camera;
use self::map::Map;
use self::entities::{ Entities, Entity, Actor };
use ggez::graphics::Point2;
use ggez::{graphics, Context, GameResult};
use std::time::Duration;
use ggez::event::{MouseButton, MouseState};
use rand::{Rng, thread_rng};

use crate::states::{Assets, State, Transition};
use std::vec::Vec;

const MAPSIZE_MAX_X: i32 = 300;
const MAPSIZE_MAX_Y: i32 = 300;
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
    let e = Actor::new(50, MAPSIZE_MAX_X / &2, MAPSIZE_MAX_Y / &2, 1.0);
    entities.add_actor(e);
    Ok( PlayState { camera, input, map, entities } )
  }

  pub fn scr_to_map(&self, x: i32, y: i32) -> (i32, i32) {
    let mut tx: i32 = -self.camera.position.x as i32;
    let mut ty: i32 = -self.camera.position.y as i32;
    let mut mx = x;
    let mut my = y;
    
    let scale: i32 = (TILESIZE as f32 * self.camera.zoomlevel).ceil() as i32;

    if tx < 0 { mx = mx + tx; tx = 0; }
    if ty < 0 { my = my + ty; ty = 0; }

    tx = (tx + mx) / scale;
    ty = (ty + my) / scale;

    (tx, ty)
  }

  pub fn map_to_scr(&mut self, x: i32, y: i32) -> (i32, i32)
  {
    let mut tx: i32 = -self.camera.position.x as i32;
    let mut ty: i32 = -self.camera.position.y as i32;
    let mut mx = x;
    let mut my = y;
    
    let scale: i32 = (TILESIZE as f32 * self.camera.zoomlevel).ceil() as i32;

    ((mx * scale) - tx, (my * scale) - ty)
  }

}

impl State for PlayState {
  fn update(&mut self, ctx: &mut Context, assets: &Assets, dt: Duration,) -> GameResult<Transition> {
    self.entities.update(ggez::timer::get_delta(ctx).subsec_millis(), self.camera.tsize);
    Ok(Transition::None)
  }

  fn draw(&mut self, ctx: &mut Context, assets: &mut Assets) -> GameResult<()> {
    let scale: Point2 = Point2::new(self.camera.zoomlevel, self.camera.zoomlevel);
    let camx = self.camera.position.x;
    let camy = self.camera.position.y;
    let tsize = (TILESIZE as f32 * self.camera.zoomlevel);

    let mut xdrawmin = ((-camx / tsize) - 1.0) as i32;
    if xdrawmin < 0 { xdrawmin = 0; }
    let mut xdrawmax = ((-camx / tsize) + 1.0 + (ctx.conf.window_mode.width as f32 / tsize)) as i32;
    if xdrawmax >= MAPSIZE_MAX_X { xdrawmax = MAPSIZE_MAX_X ; }

    let mut ydrawmin = ((-camy / tsize) - 1.0) as i32;
    if ydrawmin < 0 { ydrawmin = 0; }
    let mut ydrawmax = ((-camy / tsize) + 1.0 + (ctx.conf.window_mode.height as f32 / tsize)) as i32;
    if ydrawmax >= MAPSIZE_MAX_Y { ydrawmax = MAPSIZE_MAX_Y ; }

    for x in xdrawmin..xdrawmax {
      for y in ydrawmin..ydrawmax {

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
    self.entities.draw(camx as i32, camy as i32, scale, assets);

    Ok(())
  }

  fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: i32, y: i32) {
    match button {
      MouseButton::Left => {
        self.input.mousedown(1);
        let (a, b) = self.camera.mouse_to_tile(x, y);
        self.camera.tile_to_screen(a, b);
      }
      MouseButton::Right => {
        self.input.mousedown(2);
        println!("mouse x: {} y: {}", x, y);
        let (a, b) = self.camera.mouse_to_tile(x, y);
        println!("tile x: {} y: {}", a, b);
        self.entities.get_actor().setmovetarget(a, b, &mut self.camera);

      }
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
      self.camera.movestep(dx as f32, dy as f32);
    }
    self.input.setpos(x, y);
  }
  
  fn mouse_wheel_event(&mut self, ctx: &mut Context, _x: i32, y: i32) {
    let p = Point2::new(self.input.x as f32, self.input.y as f32);
    self.camera.zoom(y, p);
  }
}
