use ggez::*;

pub struct Lookup {
  floors: Vec<Tile>,
  walls: Vec<Tile>,
  terrain: Vec<Tile>,
  objects: Vec<Tile>,

}

struct Tile {
  pub name: String,
  pub description: String,

  pub sprite: graphics::Image,
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
