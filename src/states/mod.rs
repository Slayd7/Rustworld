use ggez::{graphics, GameResult, Context, timer};
use ggez::event::{EventHandler, MouseState, MouseButton};
use std::collections::HashMap;
use std::time::Duration;

pub mod play_state;
pub mod intro_state;
use crate::states::intro_state::IntroState;

pub trait DurationExt: Sized {
  fn as_subsec_millis(&self) -> f64;
}

impl DurationExt for Duration {
  fn as_subsec_millis(&self) -> f64 {
    f64::from(self.subsec_nanos())
  }
}

pub struct Assets {
  images: HashMap<String, graphics::Image>,
  font: HashMap<String, graphics::Font>,
}

impl Assets {
  pub fn new() -> Self {
    Self {
      images: HashMap::new(),
      font: HashMap::new(),
    }
  }

  pub fn add_image(&mut self, name: &str, image: graphics::Image) -> GameResult<()> {
    self.images.insert(name.to_string(), image);
    Ok(())
  }

  pub fn get_image(&self, name: &str) -> GameResult<&graphics::Image> {
    let img = self.images.get(name);
    Ok(img.unwrap())
  }

  pub fn add_font(&mut self, name: &str, font: graphics::Font) -> GameResult<()> {
    self.font.insert(name.to_string(), font);
    Ok(())
  }

  pub fn get_font(&self, name: &str) -> GameResult<&graphics::Font> {
    let font = self.font.get(name);
    Ok(font.unwrap())
  }
}

pub enum Transition {
  None,
  Push(Box<dyn State>),
  Swap(Box<dyn State>),
  Pop,
  Drain,
}

pub trait State {
  fn update(&mut self, ctx: &mut Context, assets: &Assets, dt: Duration,) -> GameResult<Transition>;
  fn draw(&mut self, ctx: &mut Context, assets: &Assets) -> GameResult<()>;
  fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: i32, _y: i32,) {}
  fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: i32, _y: i32,) {}
  fn mouse_motion_event(&mut self, _ctx: &mut Context, _button: MouseState, _x: i32, _y: i32, _xrel: i32, _yrel: i32) {}
  fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: i32, _y: i32) {}
  fn focus_event(&mut self, _ctx: &mut Context, _gained: bool) {}
  fn quit_event(&mut self, _ctx: &mut Context) -> bool { false }

} 

pub struct StateManager {
  assets: Assets,
  running: bool,
  states: Vec<Box<dyn State>>,
}

impl StateManager {
  pub fn new(ctx: &mut Context) -> StateManager {
    let assets = StateManager::initialize_assets(ctx).unwrap();
    let state = Box::new(IntroState::new(ctx, &assets).unwrap());

    StateManager {
      running: true,
      states: vec![state],
      assets,
    }
  }

// Long term, will want to turn this into an XML reader or something
  fn initialize_assets(ctx: &mut Context) -> GameResult<Assets> {
    let mut assets = Assets::new();
    assets.add_image("grass0", graphics::Image::new(ctx, "/terrain/grass0.png")?)?;
    assets.add_image("grass1", graphics::Image::new(ctx, "/terrain/grass1.png")?)?;
    assets.add_image("grass2", graphics::Image::new(ctx, "/terrain/grass2.png")?)?;
    assets.add_image("lemmy", graphics::Image::new(ctx, "/objects/lemmy.png")?)?;

    assets.add_font("title", graphics::Font::new(ctx, "/fonts/Rust_never_sleeps.ttf", 32)?,)?;
    assets.add_font("normal", graphics::Font::new(ctx, "/fonts/basic_sans_serif_7.ttf", 18)?,)?;
    Ok(assets)
  }

  pub fn quit(&mut self) {
    self.states.clear();
    self.running = false
  }

  fn handle_transition(&mut self, transition: Transition) {
    match transition {
      Transition::None => (),
      Transition::Pop => self.pop(),
      Transition::Swap(state) => self.swap(state),
      Transition::Push(state) => self.push(state),
      Transition::Drain => self.drain(),
    }
  }

  fn pop(&mut self) {
    self.states.pop();
    if self.states.is_empty() {
      self.quit();
    }
  }

  fn push(&mut self, boxed_state: Box<dyn State>) {
    self.states.push(boxed_state)
  }

  fn swap(&mut self, boxed_state: Box<dyn State>) {
    self.states.clear();
    self.push(boxed_state);
  }

  fn drain(&mut self) {
    self.states.clear();
    self.quit();
  }
}

impl EventHandler for StateManager {
  fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
    if !self.running {
      ctx.quit()?;
    }
    
    let dt = timer::get_delta(ctx);

    let transition = match self.states.last_mut() {
      Some(state) => state.update(ctx, &self.assets, dt),
      None => Ok(Transition::None),
    };

    self.handle_transition(transition?);

    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    graphics::set_background_color(ctx, graphics::Color::new(0.0, 0.0, 0.0, 255.0));
    graphics::clear(ctx);

    for (_, state) in self.states.iter_mut().enumerate() {
      state.draw(ctx, &self.assets)?;
    }

    graphics::present(ctx);
    timer::sleep(Duration::from_secs(0));
    Ok(())
  }

  fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: i32, y: i32) {
    if let Some(state) = self.states.last_mut() {
      state.mouse_button_down_event(ctx, button, x, y);
    }
  }

  fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: i32, y: i32) {
    if let Some(state) = self.states.last_mut() {
      state.mouse_button_up_event(ctx, button, x, y);
    }
  }

  fn mouse_motion_event(&mut self, ctx: &mut Context, m_state: MouseState, x: i32, y: i32, xrel: i32, yrel: i32) {
    if let Some(state) = self.states.last_mut() {
      state.mouse_motion_event(ctx, m_state, x, y, xrel, yrel);
    }
  }

  fn mouse_wheel_event(&mut self, ctx: &mut Context, _x: i32, y: i32){
    if let Some(state) = self.states.last_mut() {
      state.mouse_wheel_event(ctx, _x, y);
    }
  }

  fn focus_event(&mut self, ctx: &mut Context, gained: bool) {
    if let Some(state) = self.states.last_mut() {
      state.focus_event(ctx, gained);
    }
  }

  fn quit_event(&mut self, ctx: &mut Context) -> bool {
    match self.states.last_mut() {
      Some(state) => state.quit_event(ctx),
      None => false,
    }
  }
}
