use crate::gameobj::GameObj;
use crate::ggez::*;

use crate::MAPSIZE_MAX_Y;
use crate::MAPSIZE_MAX_X;

use crate::tilelibrary::TileLibrary;

pub struct TileMap {
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
  pub fn at(&self, x: i32, y: i32) -> GameResult<&GameObj> {
    //Ok(&self.map.get(x as usize).unwrap().get(y as usize))
    let t = self.map.get(x as usize).unwrap().get(y as usize).unwrap();
    Ok(t)
  }
  pub fn set(self: &mut Self, x: i32, y: i32, newTile: &GameObj) -> GameResult<()> {
    if x > MAPSIZE_MAX_X || x < 0 || y > MAPSIZE_MAX_Y || y < 0
    {
      std::mem::replace(&mut self.at(x,y).unwrap().to_owned(), GameObj::from(newTile));
    }
    // This should be an err, need to figure out how to do that correctly
    Ok(())
    
  }
  pub fn generate_map(self: &mut Self, lib: &TileLibrary) -> GameResult<()> {
    for x in 0..MAPSIZE_MAX_X {
      for y in 0..MAPSIZE_MAX_Y {
        let mut tile = lib.terrain.first().unwrap();


        self.set(x,y, tile);
      }
    }
    Ok(())
  }

}
