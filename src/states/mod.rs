use ggez::{graphics, GameResult, Context, timer};
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::event::{EventHandler, MouseState, MouseButton};
use std::collections::HashMap;
use std::time::Duration;
use std::fs::File;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};

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

struct Asset {
    spritebatch: SpriteBatch,
    alternates: Vec<SpriteBatch>,
}

impl Asset {
  fn new(sprite: SpriteBatch) -> Self {
    Asset { spritebatch: sprite, alternates: Vec::new() }
  }
  
  fn addalternate(&mut self, sprite: SpriteBatch) {
    self.alternates.push(sprite);
  }
}

pub struct Assets {

  images: HashMap<u32, Asset>,
  actorimages: HashMap<u32, Asset>,
  buildingimages: HashMap<u32, Asset>,
  names: HashMap<String, u32>,
  font: HashMap<String, graphics::Font>,
}

impl Assets {
  pub fn new() -> Self {
    Self {
      images: HashMap::new(),
      actorimages: HashMap::new(),
      buildingimages: HashMap::new(),
      names: HashMap::new(),
      font: HashMap::new(),
    }
  }

  pub fn add_image(&mut self, name: &str, id: &u32, image: graphics::Image) -> GameResult<()> {
    self.images.insert(*id, Asset::new(SpriteBatch::new(image)));
    self.names.insert(name.to_string(), *id);
    Ok(())
  }
  
  pub fn add_alt_image(&mut self, id: &u32, image: graphics::Image) -> GameResult<()> {
    self.images.get_mut(id).unwrap().addalternate(SpriteBatch::new(image));
    Ok(())
  }

  pub fn add_actor_image(&mut self, name: &str, id: &u32, image: graphics::Image) -> GameResult<()> {
    self.actorimages.insert(*id, Asset::new(SpriteBatch::new(image)));
    self.names.insert(name.to_string(), *id);
    Ok(())
  }

  pub fn add_actor_alt_image(&mut self, id: &u32, image: graphics::Image) -> GameResult<()> {
    self.actorimages.get_mut(id).unwrap().addalternate(SpriteBatch::new(image));
    Ok(())
  }

  pub fn add_building_image(&mut self, name: &str, id: &u32, image: graphics::Image) -> GameResult<()> {
    self.buildingimages.insert(*id, Asset::new(SpriteBatch::new(image)));
    self.names.insert(name.to_string(), *id);
    Ok(())
  }

  pub fn get_image(&self, id: &u32) -> GameResult<&SpriteBatch> {
    let img = self.images.get(id);
    Ok(&img.unwrap().spritebatch)
  }

  pub fn get_alt_image(&self, id: &u32, altid: usize) -> GameResult<&SpriteBatch> {
    let img = self.images.get(id).unwrap().alternates.get(altid);
    Ok(img.unwrap())
  }

  pub fn get_actor_image(&self, id: &u32) -> GameResult<&SpriteBatch> {
    let img = self.actorimages.get(id);
    Ok(&img.unwrap().spritebatch)
  }

  pub fn get_building_image(&self, id: &u32) -> GameResult<&SpriteBatch> {
    let img = self.buildingimages.get(id);
    Ok(&img.unwrap().spritebatch)
  }

  pub fn get_id(&self, name: String) -> GameResult<u32> {
    let id = self.names.get(&name);
    let a = *id.unwrap();
    Ok(*id.unwrap())

  }

  pub fn draw_image(&mut self, id: &u32, p: graphics::DrawParam) { //
    self.images.get_mut(id).unwrap().spritebatch.add(p);
  }

  pub fn draw_alt_image(&mut self, id: &u32, alt: usize, p: graphics::DrawParam) {
    self.images.get_mut(id).unwrap().alternates.get_mut(alt).unwrap().add(p);
  }

  pub fn draw_actor_image(&mut self, id: &u32, p: graphics::DrawParam) {
    self.actorimages.get_mut(id).unwrap().spritebatch.add(p);
  }

  pub fn draw_building_image(&mut self, id: &u32, p: graphics::DrawParam) {
    self.buildingimages.get_mut(id).unwrap().spritebatch.add(p);
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
  fn draw(&mut self, ctx: &mut Context, assets: &mut Assets) -> GameResult<()>;
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
    let mut assets = StateManager::initialize_assets(ctx).unwrap();
    let state = Box::new(IntroState::new(ctx, &assets).unwrap());

    StateManager {
      running: true,
      states: vec![state],
      assets,
    }
  }

  fn initialize_assets(ctx: &mut Context) -> GameResult<Assets> {
    //TODO: Get resources dir, right now dependent on where you're running it from
    let file = File::open("import.xml").unwrap();
    let file = BufReader::new(file);

    let mut parser = EventReader::new(file);

    enum Elements { name, location, alternate };

    enum SpriteTypes { tile, actor, building, }

    enum Types { sprite, font, };

    struct SpriteStruct {
      pub name: String,
      pub sprite: String,
      pub altsprites: Vec<String>,
      pub typ: SpriteTypes,
    }

    impl SpriteStruct {
      fn new(name: &String, spr: &String, typ: SpriteTypes) -> Self {
        SpriteStruct { name: name.to_string(), sprite: spr.to_string(), altsprites: Vec::new(), typ }
      }
    }

    struct FontStruct {
      pub name: String,
      pub font: graphics::Font,
    }

    impl FontStruct {
      fn new(name: &String, font: graphics::Font) -> Self {
        FontStruct { name: name.to_string(), font }
      }
    }
    
    let mut sprtyp = SpriteTypes::tile;
    let mut elm = Elements::name;
    let mut typ = Types::sprite;
    let mut is: Vec<SpriteStruct> = Vec::new(); // Sprite Struct
    let mut fs = Vec::new(); // Font struct
    let mut id: u32 = 0;

    let mut name = String::new();

    for e in parser { // XML parsing. Messy, but works
      match e {
        Ok(XmlEvent::Characters(e)) => {
          match elm {
            Elements::name => {
              name = e.to_string();
            }
            Elements::location => {
              match typ {
                Types::sprite => {
                  match sprtyp {
                    SpriteTypes::tile => {
                      is.push(SpriteStruct::new(&name, &e.to_string(), SpriteTypes::tile));
                    }
                    SpriteTypes::actor => {
                      is.push(SpriteStruct::new(&name, &e.to_string(), SpriteTypes::actor));
                    }
                    SpriteTypes::building => {

                      is.push(SpriteStruct::new(&name, &e.to_string(), SpriteTypes::building));
                    }
                  }
                }
                Types::font => {
                  fs.push(FontStruct::new(&name, graphics::Font::new(ctx, e.to_string(), 32)?));
                }
              }
              let id = id + 1;
            }
            Elements::alternate => {
              match typ {
                Types::sprite => {
                      is.get_mut((&id - 1) as usize).unwrap().altsprites.push(e.to_string());
                  }
                
                _ => {}
              }
            }
          }
        }
        Ok(XmlEvent::StartElement { name, .. }) => {
          match name.to_string().as_ref() {
            "location" => { elm = Elements::location; }
            "name" => { elm = Elements::name; }
            "alternate" => { elm = Elements::alternate; }
            "sprites" => { typ = Types::sprite; }
            "fonts" => { typ = Types::font; }
            "tiles" => { sprtyp = SpriteTypes::tile; }
            "actors" => { sprtyp = SpriteTypes::actor; }
            "buildings" => { sprtyp = SpriteTypes::building; }
            _ => { }
          }
        }
        Err(e) => {
          println!("Error: {}", e);
          break;
        }
        _ => {}
      }
    }

    let mut assets = Assets::new();

    let mut id1 = 0;
    let mut id2 = 0;
    let mut id3 = 0;
    for mut i in is {
      match i.typ {
        SpriteTypes::tile => { 
          assets.add_image(&i.name, &id1, graphics::Image::new(ctx, &i.sprite).unwrap()); 
          for j in i.altsprites.iter_mut() {
            assets.add_alt_image(&id1, graphics::Image::new(ctx, &j).unwrap());
          }
          id1 = id1 + 1;
        }
        SpriteTypes::actor => { 
          assets.add_actor_image(&i.name, &id2, graphics::Image::new(ctx, &i.sprite).unwrap());
          for j in i.altsprites.iter_mut() {
            // Add alts
          }
          id2 = id2 + 1;
        }
        SpriteTypes::building => { 
          assets.add_building_image(&i.name, &id3, graphics::Image::new(ctx, &i.sprite).unwrap()); 
          for j in i.altsprites.iter_mut() {
            // Add alts
          }
          id3 = id3 + 1;
        }
      }
    }

    for f in fs {
      assets.add_font(&f.name, f.font);
    }
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
      state.draw(ctx, &mut self.assets)?;
    }
    let p = graphics::DrawParam {
      ..Default::default()
    };

    for (_, (_, spr)) in self.assets.images.iter_mut().enumerate() {
      graphics::draw_ex(ctx, &spr.spritebatch, p)?;
      for (_, a) in spr.alternates.iter_mut().enumerate() {
        graphics::draw_ex(ctx, a, p)?;
        a.clear();
      }
      spr.spritebatch.clear();
    }

    for (_, (_, spr)) in self.assets.actorimages.iter_mut().enumerate() {
      graphics::draw_ex(ctx, &spr.spritebatch, p)?;
      for (_, a) in spr.alternates.iter_mut().enumerate() {
        graphics::draw_ex(ctx, a, p)?;
        a.clear();
      }
      spr.spritebatch.clear();
    }

    for (_, (_, spr)) in self.assets.buildingimages.iter_mut().enumerate() {
      graphics::draw_ex(ctx, &spr.spritebatch, p)?;
      for (_, a) in spr.alternates.iter_mut().enumerate() {
        graphics::draw_ex(ctx, a, p)?;
        a.clear();
      }
      spr.spritebatch.clear();
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
