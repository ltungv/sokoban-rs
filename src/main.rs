use ggez::{conf, event};

use std::env;
use std::path;

mod components;
mod entities;
mod game;
mod resources;
mod systems;

pub const ARENA_WIDTH: f32 = 720.0;
pub const ARENA_HEIGHT: f32 = game::MAP_HEIGHT as f32 * game::TILE_HEIGHT;

/// Load the game's resources and initialize the game. The path to the resources
/// is relative to the directory that contains the project's manifest, otherwise,
/// it is relative to the current position where the project is run.
fn main() -> ggez::GameResult {
    // TODO: Load map from persistence
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

    let resource_dir = env::var("CARGO_MANIFEST_DIR")
        .map(|dir| {
            let mut path = path::PathBuf::from(dir);
            path.push("resources");
            path
        })
        .unwrap_or_else(|_| path::PathBuf::from("./resources"));
    println!("Resource dir: {:?}", resource_dir);

    let (ctx, evts_loop) = &mut ggez::ContextBuilder::new("sokoban", "tlv")
        .window_setup(conf::WindowSetup::default().title("Sokoban"))
        .window_mode(conf::WindowMode::default().dimensions(ARENA_WIDTH, ARENA_HEIGHT))
        .add_resource_path(&resource_dir)
        .build()?;
    let game = &mut game::Game::new(ctx, MAP)?;
    event::run(ctx, evts_loop, game)
}
