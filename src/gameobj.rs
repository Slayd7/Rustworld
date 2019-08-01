use ggez::nalgebra::Point2;
use ggez::*;

#[derive(Clone)]
pub struct GameObj {
  pub name: ::std::string::String,
  pub description: ::std::string::String,

  pub sprite: ::ggez::graphics::Image,
  pub position: nalgebra::Point2<f32>,
  pub rotation: f32,
  pub scale: f32,
}

impl From<&GameObj> for GameObj {
  fn from(item: &GameObj) -> Self {
    GameObj {
      name: item.name.to_owned(),
      description: item.description.to_owned(),
      sprite: item.sprite.to_owned(),
      position: item.position,
      rotation: item.rotation,
      scale: item.scale,
    }
  }
}


/// Game Object
/// This counts as anything that could appear as an actor or a prop on-screen.
/// Players, items and environment objects will inherit from this.
impl GameObj {
  pub fn new(name: String, description: String, sprite: graphics::Image) -> GameObj {
    GameObj {
      name: name,
      description: description,
      sprite: sprite,
      position: nalgebra::Point2::new(0.0f32, 0.0),
      rotation: 0.0,
      scale: 1.0,
    }
  }
  pub fn getpos(self) -> nalgebra::Point2<f32> { self.position.clone() }
  pub fn setpos(mut self, p: nalgebra::Point2<f32>) -> GameResult<()> { self.position = p.clone(); Ok(()) }
  pub fn getrot(self) -> f32 { self.rotation }
  pub fn setrot(mut self, r: f32) -> GameResult<()> { self.rotation = r; Ok(()) }
  pub fn getscale(self) -> f32 { self.scale }
  pub fn setscale(mut self, s: f32) -> GameResult<()> { self.scale = s; Ok(()) }

  pub fn getsprite(self) -> ggez::graphics::Image { self.sprite }
  pub fn getdescription(self) -> String { self.description }
  pub fn getname(self) -> String { self.name }
}

