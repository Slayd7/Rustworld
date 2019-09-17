use super::*;

pub struct MapGenerator {

}

enum TileType {
  DeepWater,
  Water,
  Sand,
  Grass,
  Rock,
}

impl MapGenerator {

  fn island_mask(val: f64, x: i32, y: i32) -> f64 {
    let x = x as f64;
    let y = y as f64;
    let x = (x - MAPSIZE_MAX_X as f64 * 0.5).abs();
    let y = (y - MAPSIZE_MAX_Y as f64 * 0.5).abs();
    let x = x / (MAPSIZE_MAX_X as f64 * 0.5);
    let y = y / (MAPSIZE_MAX_Y as f64 * 0.5);
    let mut d = x;
    if d < y { d = y; }

    (1.0 - d) * val
  }

  fn generate_tile(e: f64) -> TileType {
    let mut m: TileType;
    match e {
      0.0...0.03 => m = TileType::DeepWater,
      0.03...0.08 => m = TileType::Water, // Water
      0.08...0.2 => m = TileType::Sand, // Grass
      0.5...0.8 => m = TileType::Grass,  // Grass
      0.8...1.0 => m = TileType::Grass, //Rock, // TODO
      _ => m = TileType::Grass,//Rock,
    }
    m
  }

  pub fn generate_map(seed: u32, assets: &Assets) -> Map {

    let mut tilemap: Vec<(Tile, u32)> = Vec::new();
    let mut build_layer = Vec::new();
    let mut costmap = Vec::new();

    let mut elevation = Perlin::new();
    let mut elevation = elevation.set_seed(seed);
    let mut moisture = Perlin::new();
    let mut moisture = moisture.set_seed(seed + 1);


    for x in 0..MAPSIZE_MAX_X { // Tile gen
      for y in 0..MAPSIZE_MAX_Y {
        let mut val: f64 = elevation.get([x as f64 * NOISESCALE, y as f64 * NOISESCALE]);
        val = val + (0.5 * elevation.get([x as f64 * (2.0 * NOISESCALE), y as f64 * (2.0 * NOISESCALE)]));
        val = val + (0.25 * elevation.get([x as f64 * (4.0 * NOISESCALE), y as f64 * (4.0 * NOISESCALE)]));
        val = val + 1.0;
        val = val * 0.5;  // Now between 0...1
        
        let mut m: f64 = moisture.get([x as f64 * NOISESCALE, y as f64 * NOISESCALE]);
        m = m + (0.5 * moisture.get([x as f64 * (2.0 * NOISESCALE), y as f64 * (2.0 * NOISESCALE)]));
        m = m + (0.25 * moisture.get([x as f64 * (4.0 * NOISESCALE), y as f64 * (4.0 * NOISESCALE)]));
        m = m + 1.0;
        m = m * 0.5;

        m = m.powf(1.3);
        
        let mut id = 0;
        let mut cost: usize = 1;

        val = MapGenerator::island_mask(val, x, y);
        let t = MapGenerator::generate_tile(val);
        match t {
          TileType::Water => {
            match m {
              0.0...0.5 => { id = assets.get_id("water0".to_string()).unwrap(); }
              0.5...1.0 => { id = assets.get_id("water1".to_string()).unwrap(); }
              _ => {}
            } 
            cost = usize::max_value();
          }
          TileType::DeepWater => {
            match m {
              0.0...0.5 => { id = assets.get_id("deepwater0".to_string()).unwrap(); }
              0.5...1.0 => { id = assets.get_id("deepwater1".to_string()).unwrap(); }
              _ => {}
            }
            cost = usize::max_value();
          }
          TileType::Sand => {
            match m {
              0.0...0.5 => { id = assets.get_id("drysand".to_string()).unwrap(); }
              0.5...1.0 => { id = assets.get_id("dirt".to_string()).unwrap(); }
              _ => {}
            }
            cost = 2;
          }

          TileType::Grass => {
            match m {
              0.0...0.2 => { id = assets.get_id("grass0".to_string()).unwrap(); }
              0.2...0.35 => { id = assets.get_id("grass1".to_string()).unwrap(); }
              0.35...0.55 => { id = assets.get_id("grass2".to_string()).unwrap(); }
              0.55...0.75 => { id = assets.get_id("grass3".to_string()).unwrap(); }
              0.75...0.9 => { id = assets.get_id("grass4".to_string()).unwrap(); }
              0.9...1.0 => { id = assets.get_id("grass5".to_string()).unwrap(); }
              _ => {}
            }
          }

          TileType::Rock => {

          }
          _ => {}
        }
        let t = Tile::new(id, x, y, 1.0);
        let alt = 0 as u32;

        costmap.push(cost);
        tilemap.push((t, alt));
        build_layer.push(None);
      }
    }

    // Edges pass
    for x in 0..MAPSIZE_MAX_X {
      for y in 0..MAPSIZE_MAX_Y {
        let id = tilemap.get(getmapvecidx(x, y)).unwrap().0.id;

             //         North, East,  South,  West
        let mut edge = (false, false, false, false);
        if x > 0 { // West
          if tilemap.get(getmapvecidx(x-1, y)).unwrap().0.id != id {
            edge.3 = true;
          }
        }
        if x < (MAPSIZE_MAX_X - 1) { // East
          if tilemap.get(getmapvecidx(x+1, y)).unwrap().0.id != id {
            edge.1 = true;
          }
        }
        if y > 0 { // North
          if tilemap.get(getmapvecidx(x, y-1)).unwrap().0.id != id {
            edge.0 = true;
          }
        }
        if y < (MAPSIZE_MAX_Y - 1) { // South
          if tilemap.get(getmapvecidx(x, y+1)).unwrap().0.id != id {
            edge.2 = true;
          }
        }
        println!("({}, {}, {}, {})", edge.0, edge.1, edge.2, edge.3);

      }
    }

    Map { tilemap, build_layer, costmap }
  }

}
