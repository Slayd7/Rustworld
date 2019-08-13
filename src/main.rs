#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

extern crate ggez;
extern crate rand;
extern crate xml;
use ggez::*;


mod states;
use crate::states::StateManager;

fn main() {
    let cb = ggez::ContextBuilder::new("rustworld", "Brad Hopper")
        .window_setup(conf::WindowSetup::default().title("Rustworld"))
        .window_mode(conf::WindowMode::default().dimensions(1920, 1080));

    let ctx = &mut cb.build().unwrap();

    graphics::set_screen_coordinates(ctx, graphics::Rect::new_i32(0, 0, 1920, 1080))
        .expect("Failed");

    let mut state = StateManager::new(ctx);
    if let Err(e) = event::run(ctx, &mut state) {
        println!("Error running: {}", e);
    }
}
