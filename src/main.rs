#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
extern crate ggez;
extern crate rand;
use ggez::nalgebra::Point2;
use ggez::*;
//use rand::{thread_rng, Rng};

const MAPSIZE_MAX_X: i32 = 10;
const MAPSIZE_MAX_Y: i32 = 10;

struct State {
  tileLib: TileLibrary,
  tileMap: TileMap,
}

impl State {
  fn new(ctx: &mut Context, map: TileMap) -> GameResult<State> {
    let tLib = import_tiles(ctx);
    let s = State { tileLib: tLib.unwrap(), tileMap: map, };

    Ok(s)
  }
}


impl ggez::event::EventHandler for State {
  fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
    Ok(())
  }
  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    graphics::clear(ctx);
    let mut dst = Point2::new(0.0, 0.0);
    for i in 0..10 {
      for j in 0..10 {
        dst = Point2::new(i as f32 * 100.0, j as f32 * 100.0);
        graphics::draw(ctx, &self.tileLib.terrain.first().unwrap().sprite, dst, 0.0)?;
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
  let ctx = &mut Context::load_from_conf("title", "author", c).unwrap();
  let lib = import_tiles(ctx).unwrap();
  let mut map = TileMap::new().unwrap();

  let mut state = State::new(ctx, map).unwrap();
  event::run(ctx, &mut state).unwrap();
}

#[derive(Clone)]
pub struct GameObj {
  pub name: ::std::string::String,
  pub description: ::std::string::String,

  pub sprite: ::ggez::graphics::Image,

  pub position: nalgebra::Point2<f32>,
  pub rotation: f32,
  pub scale: f32,
}

impl From<&GameObj> for GameObj {
  fn from(item: &GameObj) -> Self {
    GameObj {
      name: item.name.to_owned(),
      description: item.description.to_owned(),
      sprite: item.sprite.to_owned(),
      position: item.position,
      rotation: item.rotation,
      scale: item.scale,
    }
  }
}

/// Game Object
/// This counts as anything that could appear as an actor or a prop on-screen.
/// Players, items and environment objects will inherit from this.
impl GameObj {
  pub fn new(name: String, description: String, sprite: graphics::Image) -> GameObj {
    GameObj {
      name: name,
      description: description,
      sprite: sprite,
      position: nalgebra::Point2::new(0.0f32, 0.0),
      rotation: 0.0,
      scale: 1.0,
    }
  }
  pub fn getpos(self) -> nalgebra::Point2<f32> { self.position.clone() }
  pub fn setpos(mut self, p: nalgebra::Point2<f32>) -> GameResult<()> { self.position = p.clone(); Ok(()) }
  pub fn getrot(self) -> f32 { self.rotation }
  pub fn setrot(mut self, r: f32) -> GameResult<()> { self.rotation = r; Ok(()) }
  pub fn getscale(self) -> f32 { self.scale }
  pub fn setscale(mut self, s: f32) -> GameResult<()> { self.scale = s; Ok(()) }

  pub fn getsprite(self) -> ggez::graphics::Image { self.sprite }
  pub fn getdescription(self) -> String { self.description }
  pub fn getname(self) -> String { self.name }
}

struct TileMap {
  pub map: Vec<Vec<GameObj>>,
}

impl From<&TileMap> for TileMap {
  fn from(item: &TileMap) -> Self {
    TileMap { map: item.map.to_owned() }
  }
}

impl TileMap {
  pub fn new() -> GameResult<TileMap> {
    let mut map: Vec<Vec<GameObj>>;
    map = Vec::new();
    for x in 0..MAPSIZE_MAX_X {
      map.push(Vec::new());
    }
      //x.push(tlib.objects.first().unwrap().into());
    Ok(TileMap { map: map } )
  }
  pub fn at(self, x: i32, y: i32) -> GameResult<GameObj> {
    Ok(self.map.get(x as usize).unwrap().get(y as usize).unwrap().clone())
  }

}

struct Tile_stats {
// TODO
}

#[derive(Clone)]
struct TileLibrary {
  floors: Vec<GameObj>,
  walls: Vec<GameObj>,
  terrain: Vec<GameObj>,
  objects: Vec<GameObj>,
}

impl TileLibrary {
  pub fn new() -> GameResult<TileLibrary> {
    Ok(TileLibrary {
      floors:  Vec::new(),
      walls:   Vec::new(),
      terrain: Vec::new(),
      objects: Vec::new()
    })
  }
  pub fn fill_table(&mut self, s: String, mut t: &Vec<GameObj>) -> GameResult<()> {

    match s.as_ref() {
      "floors" => { std::mem::replace(&mut self.floors, t.to_vec()); }
      "walls" => { std::mem::replace(&mut self.walls, t.to_vec()); }
      "terrain" => { std::mem::replace(&mut self.terrain, t.to_vec()); }
      "objects" => { std::mem::replace(&mut self.objects, t.to_vec()); }
      _ => { }
    }

    Ok(())
  }
}


fn import_tiles(ctx: &mut ggez::Context) -> GameResult<(TileLibrary)>
{
 
  let mut lib: TileLibrary;
  lib = TileLibrary::new().unwrap();
  // let mut resourcesloc = ctx.filesystem.get_resources_dir().to_owned();
  // I'm leaving this here in remembrance of the MASSIVE AMOUNT OF TIME IT STOLE FROM ME
  // AND I DON'T EVEN NEED IT :(

  let mut v = Vec::new();

  for entry in ctx.filesystem.read_dir("/floors/")? {
    let img = graphics::Image::new(ctx, entry);
    let mut t = GameObj::new("Floor".to_string(), "A simple, wooden floor.".to_string(), img.unwrap());
    v.push(t);
  }
  lib.fill_table("floors".to_string(), &v);
  v = Vec::new();
  for entry in ctx.filesystem.read_dir("/walls/")? {
    let img = graphics::Image::new(ctx, entry);
    let mut t = GameObj::new("Wall".to_string(), "A simple, wooden wall.".to_string(), img.unwrap());
    v.push(t);
  }
  lib.fill_table("walls".to_string(), &v);
  v = Vec::new();
  for entry in ctx.filesystem.read_dir("/terrain/")? {
    let img = graphics::Image::new(ctx, entry);
    let mut t = GameObj::new("Grass".to_string(), "A grassy field.".to_string(), img.unwrap());
    v.push(t);
  }
  lib.fill_table("terrain".to_string(), &v);
  v = Vec::new();
  for entry in ctx.filesystem.read_dir("/objects/")? {
    let img = graphics::Image::new(ctx, entry);
    let mut t = GameObj::new("Object".to_string(), "It's a thing! Who knows what, really.".to_string(), img.unwrap());
    v.push(t);
  }
  lib.fill_table("objects".to_string(), &v);
  Ok(lib)

}

