use ggez::{conf, event};

use std::env;
use std::path;

mod components;
mod entities;
mod game;
mod resources;
mod systems;

/// Window's width
pub const ARENA_WIDTH: f32 = 720.0;

/// Window's height
pub const ARENA_HEIGHT: f32 = 640.0;

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

    const MAP: &str = "\
        N N W W W W W W W
        W W W . . . . . W
        W . . . BB . . . W
        W . . RB . . . . W 
        W . P . . . . . W
        W . . . . RS . . W
        W . . BS . . . . W
        W . . . . . . . W
        W W W W W W W W W";

    let game = &mut game::Game::new(ctx, MAP)?;
    event::run(ctx, evts_loop, game)
}
