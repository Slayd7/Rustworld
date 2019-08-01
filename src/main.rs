#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
extern crate ggez;
extern crate rand;
use ggez::nalgebra::Point2;
use ggez::*;

mod gameobj;
use gameobj::GameObj;

mod tilemap;
use tilemap::TileMap;

mod tilelibrary;
use tilelibrary::TileLibrary;

//use rand::{thread_rng, Rng};


// mod count; // Need both of these to import other files
// use count::Count;

const MAPSIZE_MAX_X: i32 = 10;
const MAPSIZE_MAX_Y: i32 = 10;

struct State {
  tileLib: TileLibrary,
  tileMap: TileMap,
}

impl State {
  fn new(ctx: &mut Context, map: TileMap, lib: TileLibrary) -> GameResult<State> {
    let s = State { tileLib: lib, tileMap: map, };

    Ok(s)
  }
}


impl ggez::event::EventHandler for State {
  fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
    Ok(())
  }
  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    graphics::clear(ctx);
    println!("Got to draw");
    let mut dst = Point2::new(0.0, 0.0);
    for x in 0..MAPSIZE_MAX_X {
      for y in 0..MAPSIZE_MAX_Y {
        dst = Point2::new(x as f32 * 100.0, y as f32 * 100.0);
        graphics::draw(ctx, &self.tileMap.at(x, y).unwrap().sprite, dst, 0.0)?;
      }
    }
    dst = Point2::new(30.0, 30.0);
    graphics::draw(ctx, &self.tileLib.objects.first().unwrap().sprite, dst, 0.0)?;
    graphics::present(ctx);
    Ok(())
  }
}

fn main() 
{
  let c = conf::Conf::new();
  let ctx = &mut Context::load_from_conf("Rustworld", "Brad Hopper", c).unwrap();
  let lib = TileLibrary::import_objs(ctx).unwrap();
  let mut map = TileMap::new().unwrap();
  map.generate_map(&lib).unwrap();
  let mut state = State::new(ctx, map, lib).unwrap();


  event::run(ctx, &mut state).unwrap();
}

