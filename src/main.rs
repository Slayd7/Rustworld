use ggez::*;

pub fn main() {
  let state = &mut State { };

  let c = conf::Conf::new();
  let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("Rustworld", "bhopper")
    .conf(c)
    .build()
    .unwrap();

  event::run(ctx, event_loop, state).unwrap();
}

struct State{}

// https://github.com/ggez/ggez/blob/master/docs/guides/HelloGgez.md
impl ggez::event::EventHandler for State {
  fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
    Ok(())
  }
  fn draw(&mut self, ctx:&mut Context) -> GameResult<()> {
    Ok(())
  }
}
