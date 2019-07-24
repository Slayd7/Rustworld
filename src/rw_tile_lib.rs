use ggez::*;
use crate::rw_import::*;

pub struct Lookup {
  pub floors: ::std::vec::Vec<Tile>,
  pub walls: ::std::vec::Vec<Tile>,
  pub terrain: ::std::vec::Vec<Tile>,
  pub objects: ::std::vec::Vec<Tile>,
}

impl Lookup {
  pub fn new(mut self: Self) -> Lookup {
    self.floors = Vec::new();
    self.walls = Vec::new();
    self.terrain = Vec::new();
    self.objects = Vec::new();
    import_tiles(&self);
    self
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
