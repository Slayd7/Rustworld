#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
extern crate ggez;
extern crate rand;
use ggez::*;
use rand::{thread_rng, Rng};

mod rw_tile_lib;
mod rw_import;

struct State {
}

impl ggez::event::EventHandler for State {
  fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
    Ok(())
  }
  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    graphics::clear(ctx);
    graphics::present(ctx);
    Ok(())
  }
}

fn main() 
{
  let state = &mut State { };
  let c = conf::Conf::new();
  let ctx = &mut Context::load_from_conf("title", "author", c).unwrap();
  let lib = &mut rw_tile_lib::Lookup::new().unwrap();

  if rw_import::import_tiles(ctx, lib) {
    event::run(ctx, state).unwrap();
  }
}

pub struct Lookup {
  pub floors: Vec<Tile>,
  pub walls: Vec<Tile>,
  pub terrain: Vec<Tile>,
  pub objects: Vec<Tile>,
}

impl Lookup {
  pub fn new() -> GameResult<Lookup> {
    
    Ok(Lookup {
      floors:  Vec::new(),
      walls:   Vec::new(),
      terrain: Vec::new(),
      objects: Vec::new()
    })
  }
}

pub struct Tile {
  pub name: ::std::string::String,
  pub description: ::std::string::String,

  pub sprite: ::ggez::graphics::Image,
//  stats: Tile_stats,
}

impl Tile {
  pub fn new(name: String, description: String, sprite: graphics::Image) -> Tile {
    Tile {
      name: name,
      description: description,
      sprite: sprite,
    }
  }
}

struct Tile_stats {
// TODO
}

fn visit_dirs(dir: &Path) -> GameResult<Vec<Tile>>{
  if dir.is_dir() {
    for entry in fs::read_dir(dir)? {
      let entry = entry?;
      let path = entry.path();
      println!("{}", path.display());

    }
  }
  Ok()

}

pub fn import_tiles(ctx: &mut ggez::Context, mut lib: &Lookup) -> bool
{
  let resourcesloc = ctx.filesystem.get_resources_dir();
  visit_dirs(resourcesloc);


  true
}

