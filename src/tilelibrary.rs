use ggez::*;

use crate::gameobj::GameObj;

#[derive(Clone)]
pub struct TileLibrary {
  pub floors: Vec<GameObj>,
  pub walls: Vec<GameObj>,
  pub terrain: Vec<GameObj>,
  pub objects: Vec<GameObj>,
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

  pub fn import_objs(ctx: &mut ggez::Context) -> GameResult<(TileLibrary)>
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
}
