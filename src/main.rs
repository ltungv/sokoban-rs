use ggez::{conf, event};

use std::env;
use std::path;

mod components;
mod game;

pub const ARENA_WIDTH: f32 = 600.0;
pub const ARENA_HEIGHT: f32 = 600.0;

/// The game will contains the following entities:
/// + Moveable entities
///     1. Player: mint::Point3, Renderable, Moveable
///     2. Box:    mint::Point3, Renderable, Moveable
/// + Immoveable entities:
///     1. Wall:     mint::Point3, Renderable
///     2. Floor:    mint::Point3, Renderable
///     3. Box spot: mint::Point3, Renderable
fn main() -> ggez::GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };
    println!("Resource dir: {:?}", resource_dir);

    let (ctx, evts_loop) = &mut ggez::ContextBuilder::new("sokoban", "tlv")
        .window_setup(conf::WindowSetup::default().title("Sokoban"))
        .window_mode(conf::WindowMode::default().dimensions(ARENA_WIDTH, ARENA_HEIGHT))
        .add_resource_path(&resource_dir)
        .build()
        .unwrap();

    let game = &mut game::Game::new(ctx)?;
    event::run(ctx, evts_loop, game)
}
