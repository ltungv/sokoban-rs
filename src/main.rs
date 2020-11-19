use ggez::{conf, event, mint};

use std::env;
use std::path;

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
fn main() {
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

    let state = &mut MainState::new(ctx);
    if let Err(e) = event::run(ctx, evts_loop, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}

//////////////////////////////////
// Components
//////////////////////////////////

type Position = mint::Point3<u8>;

struct Renderable(path::PathBuf);

#[derive(Default)]
struct Player;

#[derive(Default)]
struct Box;

#[derive(Default)]
struct Wall;

#[derive(Default)]
struct BoxSpot;

//////////////////////////////////
// Entities
//////////////////////////////////

/// Creates a player entity
fn create_player(world: &mut legion::World, player_pos: Position) -> legion::Entity {
    world.push((
        Player,
        Position {
            z: 10,
            ..player_pos
        },
        Renderable(path::PathBuf::from("/images/player_1.png")),
    ))
}

/// Creates a box entity
fn create_box(world: &mut legion::World, box_pos: Position) -> legion::Entity {
    world.push((
        Box,
        Position { z: 10, ..box_pos },
        Renderable(path::PathBuf::from("/images/box_red_1.png")),
    ))
}

/// Creates a wall entity
fn create_wall(world: &mut legion::World, wall_pos: Position) -> legion::Entity {
    world.push((
        Wall,
        Position { z: 10, ..wall_pos },
        Renderable(path::PathBuf::from("/images/wall.png")),
    ))
}

/// Creates a box spot entity
fn create_box_spot(world: &mut legion::World, box_spot_pos: Position) -> legion::Entity {
    world.push((
        BoxSpot,
        Position {
            z: 9,
            ..box_spot_pos
        },
        Renderable(path::PathBuf::from("/images/box_spot_red.png")),
    ))
}

/// Creates a floor entity
fn create_floor(world: &mut legion::World, floor_pos: Position) -> legion::Entity {
    world.push((
        Position { z: 5, ..floor_pos },
        Renderable(path::PathBuf::from("/images/floor.png")),
    ))
}

//////////////////////////////////
// Game states
//////////////////////////////////

struct MainState;

impl MainState {
    fn new(_ctx: &mut ggez::Context) -> Self {
        Self {}
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }
}
