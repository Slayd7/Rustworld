use ggez::graphics::Point2;
use super::{MAPSIZE_MAX_X, MAPSIZE_MAX_Y };

const MAXZOOM: f32 = 2.0;
const MINZOOM: f32 = 0.1;

const ZOOMSTEP: f32 = 0.1;
const MOVESTEP: f32 = 1.5;

pub struct Camera {
  pub position: Point2,
  pub zoomlevel: f32,
}

impl Camera {
  pub fn new() -> Self {
    Camera {
      position: Point2::new(MAPSIZE_MAX_X as f32 / 2.0, MAPSIZE_MAX_Y as f32 / 2.0),
      zoomlevel: 1.0,
    }
  }

  pub fn move_to(&mut self, newpos: Point2) -> bool {

    self.position = self.inbounds_point2(newpos);

    true
  }

  pub fn zoom (&mut self, newzoom: i32) -> bool {
    let z = self.zoomlevel + (newzoom as f32 * ZOOMSTEP);
    if (z > MAXZOOM) || (z < MINZOOM) {
      return false;
    }

    self.zoomlevel = z;

    true
  }

  pub fn movestep(&mut self, x: i32, y: i32) -> bool {
    let p = Point2::new(self.position.x - (x as f32 * MOVESTEP), self.position.y - (y as f32 * MOVESTEP));
    self.position = self.inbounds_point2(p);
    true
  }

  fn inbounds_point2(&mut self, tocheck: Point2) -> Point2 {
    let mut x: f32 = tocheck.x;
    let mut y: f32 = tocheck.y;
    let scale: f32 = 100.0 * self.zoomlevel;
    if &x < &((MAPSIZE_MAX_X as f32 + 10.0) * -1.0 * &scale) 
    {
      x = (MAPSIZE_MAX_X as f32 + 10.0) * -1.0 * &scale;
    } else if &x > &(10.0 * 1.0 * &scale) {
      x = 10.0 * &scale;
    }

    if &y < &((MAPSIZE_MAX_Y as f32 + 10.0) * -1.0 * &scale)
    {
      y = (MAPSIZE_MAX_Y as f32 + 10.0) * -1.0 * &scale;
    } else if &y > &(10.0 * 1.0 * &scale) {
      y = 10.0 * &scale;
    }

    Point2::new(x, y)
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

#[test]
  fn test_ibp2() {
    assert_eq!(Camera::inbounds_point2(Point2::new(5000.0, 5000.0)), Point2::new(MAPSIZE_MAX_X as f32, MAPSIZE_MAX_Y as f32));

  }
}
