use ggez::graphics::Point2;
use super::{MAPSIZE_MAX_X, MAPSIZE_MAX_Y };

pub struct Camera {
  position: Point2,
  zoomlevel: f32,
}

impl Camera {
  pub fn new() -> Self {
    Camera {
      position: Point2::new(MAPSIZE_MAX_X as f32 / 2.0, MAPSIZE_MAX_Y as f32 / 2.0),
      zoomlevel: 1.0,
    }
  }
}
