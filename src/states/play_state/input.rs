use ggez::graphics::Point2;
use super::{MAPSIZE_MAX_X, MAPSIZE_MAX_Y};
use super::camera;

pub struct Input {
  pub mouse1down: bool,
  pub mouse2down: bool,
  pub mouse3down: bool,

  pub mwheeldelta: f32,
  pub xdelta: i32,
  pub ydelta: i32,

  pub x: i32,
  pub y: i32,
}

impl Input {
  pub fn new() -> Input {
    let mut mouse1down = false;
    let mut mouse2down = false;
    let mut mouse3down = false;

    let mut mwheeldelta = 0.0;
    let mut xdelta = 0;
    let mut ydelta = 0;

    let mut x = 0;
    let mut y = 0;

    Input { mouse1down, mouse2down, mouse3down, mwheeldelta, xdelta, ydelta, x, y }
  }

  pub fn mousedown(&mut self, button: i32) {
    match &button {
      1 => self.mouse1down = true,
      2 => self.mouse2down = true,
      3 => self.mouse3down = true,
      _ => {},
    }
  }
    
  pub fn mouseup(&mut self, button: i32) {
    match &button {
      1 => self.mouse1down = false,
      2 => self.mouse2down = false,
      3 => self.mouse3down = false,
      _ => {},
    }
  }

  pub fn mwheel(&mut self, dir: f32) {
    self.mwheeldelta = dir;
  }

  pub fn getmwheeld(&mut self) -> f32 {
    let m = self.mwheeldelta;
    self.mwheeldelta = 0.0;
    m
  }

  pub fn getmouse1(&mut self) -> bool { self.mouse1down }
  pub fn getmouse2(&mut self) -> bool { self.mouse2down }
  pub fn getmouse3(&mut self) -> bool { self.mouse3down }

  pub fn getxdelta(&mut self) -> i32 {
    let XD = self.xdelta;
    self.xdelta = 0;
    XD
  }

  pub fn getydelta(&mut self) -> i32 {
    let yd = self.ydelta;
    self.ydelta = 0;
    yd
  }

  pub fn getpos(&mut self) -> (i32, i32) {
    (self.x, self.y)
  }

  pub fn setpos(&mut self, x: i32, y: i32) {
    self.xdelta = self.x - x;
    self.ydelta = self.y - y;
    self.x = x;
    self.y = y;
  }
}
