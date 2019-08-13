use crate::states::play_state::PlayState;
use ggez::event::MouseButton;
use ggez::graphics::Point2;
use ggez::{graphics, Context, GameResult};
use std::time::Duration;

use crate::states::{Assets, State, Transition};

// https://github.com/obsoke/rustris/ used as a guide

const NANOS_PER_SEC: f64 = 1_000_000_000.0;
pub trait DurationExt: Sized {
    fn as_subsec_millis(&self) -> f64;
}

impl DurationExt for Duration {
    fn as_subsec_millis(&self) -> f64 {
        f64::from(self.subsec_nanos()) / NANOS_PER_SEC
    }
}
const FADE_TIME: f32 = 3.0;

pub struct IntroState {
    intro_text: graphics::Text,
    author_text: graphics::Text,
    hit_any_key: bool,
    fader: f32,
    fade_in: f32,
    authorfade: f32,
    afader: f32,
}

impl IntroState {
    pub fn new(ctx: &mut Context, assets: &Assets) -> GameResult<Self> {
        let intro_text = graphics::Text::new(ctx, "Rustworld", assets.get_font("rns")?)?;
        let author_text = graphics::Text::new(ctx, "by Brad Hopper", assets.get_font("basic")?)?;

        Ok(IntroState {
            intro_text,
            author_text,
            hit_any_key: false,
            fader: 0.0,
            fade_in: 0.0,
            authorfade: 0.0,
            afader: 0.0,
        })
    }

    pub fn handle_input(&mut self) {
        self.hit_any_key = true;
    }
}

impl State for IntroState {
    fn update(
        &mut self,
        ctx: &mut Context,
        assets: &Assets,
        dt: Duration,
    ) -> GameResult<Transition> {
        if self.hit_any_key {
            graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0))?;
            return Ok(Transition::Swap(Box::new(PlayState::new(ctx, assets)?)));
        }

        if self.fade_in < FADE_TIME {
            self.fade_in += dt.as_subsec_millis() as f32;
            self.fader = self.fade_in;
        } else if self.authorfade < FADE_TIME {
            self.authorfade += dt.as_subsec_millis() as f32;
            self.afader = self.authorfade;
        }
        Ok(Transition::None)
    }

    fn draw(&mut self, ctx: &mut Context, _: &mut Assets) -> GameResult<()> {
        let coords = graphics::get_screen_coordinates(ctx);
        let text_offset = (self.intro_text.width() / 2) as f32;
        let author_offset = (self.author_text.width() / 2) as f32;

        let intro_text_dest = Point2::new(coords.w / 2.0 - text_offset, 300.0);
        let author_text_dest = Point2::new(coords.w / 2.0 - author_offset, 400.0);

        graphics::set_color(
            ctx,
            graphics::Color::new(1.0, 1.0, 1.0, self.fader / FADE_TIME),
        )?;
        graphics::draw(ctx, &self.intro_text, intro_text_dest, 0.0)?;
        graphics::set_color(
            ctx,
            graphics::Color::new(0.8, 0.8, 0.8, self.afader / FADE_TIME),
        )?;
        graphics::draw(ctx, &self.author_text, author_text_dest, 0.0)?;

        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: i32, _y: i32) {
        self.handle_input();
    }
}
