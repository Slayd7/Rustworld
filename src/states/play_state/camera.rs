use ggez::graphics::Point2;
use super::{MAPSIZE_MAX_X, MAPSIZE_MAX_Y, TILESIZE };

const MAXZOOM: f32 = 1.3;
const MINZOOM: f32 = 0.3;

const ZOOMSTEP: f32 = 0.1;
const MOVESTEP: f32 = 2.0;

pub struct Camera {
  pub position: Point2,
  pub zoomlevel: f32,
  scrX: i32,
  scrY: i32,

  pub tsize: f32,

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

      tsize: TILESIZE as f32,

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
    
    let scale = (TILESIZE as f32 * self.zoomlevel);

    if tx < 0 { mx = mx + tx; tx = 0; }
    if ty < 0 { my = my + ty; ty = 0; }

    tx = ((tx + mx) as f32 / scale).floor() as i32;
    ty = ((ty + my) as f32 / scale).floor() as i32;

    (tx, ty)
  }

/// Tile_To_Screen (&mut self, x: i32, y: i32) -> (i32, i32)
/// Returns screenspace coordinates (x, y) of center of specified game tile
  pub fn tile_to_screen(&mut self, x: i32, y: i32) -> (i32, i32)
  {
    let mut tx: i32 = -(self.position.x / self.zoomlevel) as i32;
    let mut ty: i32 = -(self.position.y / self.zoomlevel) as i32;
    let mut mx = x;
    let mut my = y;
    
    let scale: i32 = (TILESIZE as f32 * self.zoomlevel).ceil() as i32;

    ((mx * scale) - tx, (my * scale) - ty)
  }

  pub fn zoom (&mut self, newzoom: i32, mousepos: Point2) -> bool {
    let mut z = self.zoomlevel + (newzoom as f32 * ZOOMSTEP * self.zoomlevel);
    if z > MAXZOOM {
      z = MAXZOOM;
    }
    else if z < MINZOOM {
      z = MINZOOM;
    }
    
    self.tsize = (TILESIZE as f32 * z);

    let mut x: i32 = -((MAPSIZE_MAX_X as f32 * self.tsize) - self.tsize) as i32 - self.scrX;
    let mut y: i32 = -((MAPSIZE_MAX_Y as f32 * self.tsize) - self.tsize) as i32 - self.scrY;

    if x > 0 { x = 0; }
    if y > 0 { y = 0; }
    self.maplimx = -(((MAPSIZE_MAX_X * TILESIZE) as f32 * z) as i32 - self.scrX + (TILESIZE as f32 * z) as i32);
    self.maplimy = -(((MAPSIZE_MAX_Y * TILESIZE) as f32 * z) as i32 - self.scrY + (TILESIZE as f32 * z) as i32);


    self.min_x = x;
    self.min_y = y;

    self.zoomlevel = z;
    let x = ((self.position.x * z) - (self.scrX as f32 / 2.0) + mousepos.x as f32);
    let y = ((self.position.y * z) - (self.scrY as f32 / 2.0) + mousepos.y as f32);
    self.moveto(-x, -y);
    true
  }

  pub fn movestep(&mut self, x: f32, y: f32) -> bool {
    let p = Point2::new(self.position.x + (x * MOVESTEP * self.zoomlevel), self.position.y + (y * MOVESTEP * self.zoomlevel));
    self.position = self.inbounds_point2(p);
    true
  }
  
  pub fn moveto(&mut self, x: f32, y: f32) -> bool {
    let p = Point2::new(x, y);
    self.position = self.inbounds_point2(p);
    true

  }

  fn inbounds_point2(&mut self, tocheck: Point2) -> Point2 {
    let mut x = tocheck.x;
    let mut y = tocheck.y;
    if &x <= &(self.maplimx as f32)
    {
      x = self.maplimx as f32;
    } else if &x > &self.tsize {
      x = self.tsize;
    }

    if &y <= &(self.maplimy as f32)
    {
      y = self.maplimy as f32;
    } else if &y > &self.tsize {
      y = self.tsize;
    }

    Point2::new(x as f32, y as f32)
  }

}

