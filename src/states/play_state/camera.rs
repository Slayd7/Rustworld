use ggez::graphics::Point2;
use super::{MAPSIZE_MAX_X, MAPSIZE_MAX_Y, TILESIZE };

const ZOOMLEVELS: usize = 14;
const ZOOMSTEP: [f32; ZOOMLEVELS] = [0.3, 0.33, 0.36, 0.4, 0.45, 0.5, 0.55, 0.6, 0.65, 0.7, 0.75, 0.8, 0.9, 1.0];
const MOVESTEP: f32 = 1.0;

pub struct Camera {
  pub position: Point2,
  pub zoomlevel: f32,
  zoomstep: i32,
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
      zoomstep: 9,

      scrX: ctx.conf.window_mode.width as i32,
      scrY: ctx.conf.window_mode.height as i32,

      tsize: TILESIZE as f32,

      maplimx: -((MAPSIZE_MAX_X * TILESIZE) - ctx.conf.window_mode.width as i32 - TILESIZE),
      maplimy: -((MAPSIZE_MAX_Y * TILESIZE) - ctx.conf.window_mode.height as i32 - TILESIZE),

      min_x: -((MAPSIZE_MAX_X * TILESIZE) - ctx.conf.window_mode.width as i32 - TILESIZE), 
      min_y: -((MAPSIZE_MAX_Y * TILESIZE) - ctx.conf.window_mode.height as i32 - TILESIZE), 

    }
  }

/// Mouse_To_Tile (&mut self, x: i32, y: i32) -> (i32, i32)
/// Returns map tile coordinates (x, y) of specified screenspace coordinates
  pub fn mouse_to_tile(&mut self, x: i32, y: i32) -> (i32, i32)
  {
    (-((self.position.x as i32 - x) as f32 / self.tsize) as i32,
     -((self.position.y as i32 - y) as f32 / self.tsize) as i32)
  }

/// Tile_To_Screen (&mut self, x: i32, y: i32) -> (i32, i32)
/// Returns screenspace coordinates (x, y) of center of specified game tile
  pub fn tile_to_screen(&mut self, x: i32, y: i32) -> (i32, i32)
  {
    (-x * self.tsize as i32, -y * self.tsize as i32)
  }

  pub fn zoom (&mut self, newzoom: i32, mousepos: Point2) -> bool {
    self.zoomstep = self.zoomstep + newzoom;
    if self.zoomstep < 0 { self.zoomstep = 0; return false; }
    if self.zoomstep >= ZOOMLEVELS as i32 { self.zoomstep = ZOOMLEVELS as i32 - 1; return false; }

    let (mx, my) = self.mouse_to_tile(mousepos.x as i32, mousepos.y as i32);
    
    self.zoomlevel = ZOOMSTEP[self.zoomstep as usize];
    
    self.tsize = TILESIZE as f32 * self.zoomlevel;

    let mut x: i32 = -((MAPSIZE_MAX_X as f32 * self.tsize) - self.tsize) as i32 - self.scrX;
    let mut y: i32 = -((MAPSIZE_MAX_Y as f32 * self.tsize) - self.tsize) as i32 - self.scrY;

    if x > 0 { x = 0; }
    if y > 0 { y = 0; }
    self.maplimx = -((MAPSIZE_MAX_X as f32 * self.tsize) as i32 - self.scrX + (self.tsize) as i32);
    self.maplimy = -((MAPSIZE_MAX_Y as f32 * self.tsize) as i32 - self.scrY + (self.tsize) as i32);

    self.min_x = x;
    self.min_y = y;
    self.movetotile(mx, my);
    true
  }

  pub fn movestep(&mut self, x: f32, y: f32) -> bool {
    let p = Point2::new(self.position.x + (x * MOVESTEP ), self.position.y + (y * MOVESTEP ));
    self.position = self.inbounds_point2(p);
    true
  }
  
  pub fn movetotile(&mut self, x: i32, y: i32) -> bool {
    let (x, y) = self.tile_to_screen(x, y);
    let p = Point2::new((x + (self.scrX / 2) ) as f32, (y + (self.scrY / 2)) as f32);
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

