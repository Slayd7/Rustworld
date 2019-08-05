use ggez::graphics::Point2;
use super::{MAPSIZE_MAX_X, MAPSIZE_MAX_Y, TILESIZE };

const MAXZOOM: f32 = 1.3;
const MINZOOM: f32 = 0.3;

const ZOOMSTEP: f32 = 0.1;
const MOVESTEP: f32 = 1.0;

pub struct Camera {
  pub position: Point2,
  pub zoomlevel: f32,
  scrX: i32,
  scrY: i32,

  tsize: i32,

  maplimx: i32,
  maplimy: i32,

  min_x: i32,
  min_y: i32,
}

impl Camera {
  pub fn new(ctx: &mut ggez::Context) -> Self {
    Camera {
      position: Point2::new((MAPSIZE_MAX_X as f32 / 2.0) * -TILESIZE as f32, (MAPSIZE_MAX_Y as f32 / 2.0) * -TILESIZE as f32),
      zoomlevel: 1.0,

      scrX: ctx.conf.window_mode.width as i32,
      scrY: ctx.conf.window_mode.height as i32,

      tsize: TILESIZE,

      maplimx: -((MAPSIZE_MAX_X * TILESIZE) - ctx.conf.window_mode.width as i32 - TILESIZE),
      maplimy: -((MAPSIZE_MAX_Y * TILESIZE) - ctx.conf.window_mode.height as i32 - TILESIZE),

      min_x: -((MAPSIZE_MAX_X * TILESIZE) - ctx.conf.window_mode.width as i32 - TILESIZE), 
      min_y: -((MAPSIZE_MAX_Y * TILESIZE) - ctx.conf.window_mode.height as i32 - TILESIZE), 

    }
  }

  pub fn move_to(&mut self, newpos: Point2) -> bool {

    self.position = self.inbounds_point2(newpos);

    true
  }

/// Mouse_To_Tile (&mut self, x: i32, y: i32) -> (i32, i32)
/// Returns map tile coordinates (x, y) of specified screenspace coordinates
  pub fn mouse_to_tile(&mut self, x: i32, y: i32) -> (i32, i32)
  {
    let mut tx: i32 = -self.position.x as i32;
    let mut ty: i32 = -self.position.y as i32;
    let mut mx = x;
    let mut my = y;
    
    let scale: i32 = (TILESIZE as f32 * self.zoomlevel).ceil() as i32;

    if tx < 0 { mx = mx + tx; tx = 0; }
    if ty < 0 { my = my + ty; ty = 0; }

    tx = (tx + mx) / scale;
    ty = (ty + my) / scale;

    (tx, ty)
  }

/// Tile_To_Screen (&mut self, x: i32, y: i32) -> (i32, i32)
/// Returns screenspace coordinates (x, y) of center of specified game tile
  pub fn tile_to_screen(&mut self, x: i32, y: i32) -> (i32, i32)
  {
    let mut tx: i32 = -self.position.x as i32;
    let mut ty: i32 = -self.position.y as i32;
    let mut mx = x;
    let mut my = y;
    
    let scale: i32 = (TILESIZE as f32 * self.zoomlevel).ceil() as i32;

    ((mx * scale) - tx, (my * scale) - ty)
  }

  pub fn zoom (&mut self, newzoom: i32, mousepos: Point2) -> bool {
    let z = self.zoomlevel + (newzoom as f32 * ZOOMSTEP * self.zoomlevel);
    if (z > MAXZOOM) || (z < MINZOOM) {
      return false;
    }
    
    self.tsize = (TILESIZE as f32 * z).ceil() as i32;

    let mut x: i32 = -((MAPSIZE_MAX_X * self.tsize) - self.scrX + self.tsize);
    let mut y: i32 = -((MAPSIZE_MAX_Y * self.tsize) - self.scrY + self.tsize);

    if x > 0 { x = 0; }
    if y > 0 { y = 0; }

    self.min_x = x;
    self.min_y = y;

    self.zoomlevel = z;
    self.movestep((-(self.scrX / 2) + mousepos.x as i32) * z as i32, (-(self.scrY / 2) + mousepos.y as i32) * z as i32);

    true
  }

  pub fn movestep(&mut self, x: i32, y: i32) -> bool {
    let p = Point2::new(self.position.x - (x as f32 * MOVESTEP * self.zoomlevel), self.position.y - (y as f32 * MOVESTEP * self.zoomlevel));
    self.position = self.inbounds_point2(p);
    true
  }

  fn inbounds_point2(&mut self, tocheck: Point2) -> Point2 {
    let mut x: i32 = tocheck.x as i32;
    let mut y: i32 = tocheck.y as i32;
    if &x < &self.min_x
    {
      x = self.min_x;
    } else if &x > &self.tsize {
      x = self.tsize;
    }

    if &y < &self.min_y
    {
      y = self.min_y;
    } else if &y > &self.tsize {
      y = self.tsize;
    }

    Point2::new(x as f32, y as f32)
  }

}

#[cfg(test)]
mod tests {
  use super::*;

#[test]
  fn test_move_to() {
    let mut c = Camera::new();
    c.move_to(Point2::new(10.0, 10.0));
    assert_eq!(c.position, Point2::new(10.0, 10.0));
  }

#[test]
  fn test_zoom() {
    let mut c = Camera::new();
    c.zoom(0.8);
    assert_eq!(c.zoomlevel, 0.8);
  }
}
