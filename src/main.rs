use ggez::{conf, event, graphics, mint};
use legion::query::IntoQuery;

use std::env;
use std::path;

pub const ARENA_WIDTH: f32 = 600.0;
pub const ARENA_HEIGHT: f32 = 600.0;

pub const TILE_WIDTH: f32 = 32.0;
pub const TILE_HEIGHT: f32 = 32.0;

//////////////////////////////////
// Game states
//////////////////////////////////

struct Game {
    world: legion::World,
}

impl Game {
    fn new(ctx: &mut ggez::Context) -> ggez::GameResult<Self> {
        let mut world = legion::World::default();
        for y in 0..4 {
            create_floor(ctx, &mut world, Position { x: 0, y, z: 0 })?;
        }
        create_wall(ctx, &mut world, Position { x: 0, y: 0, z: 0 })?;
        create_box(ctx, &mut world, Position { x: 0, y: 1, z: 0 })?;
        create_box_spot(ctx, &mut world, Position { x: 0, y: 2, z: 0 })?;
        create_player(ctx, &mut world, Position { x: 0, y: 3, z: 0 })?;
        Ok(Self { world })
    }
}

impl event::EventHandler for Game {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, graphics::WHITE);

        let mut query = <(&Position, &Renderable)>::query();
        let mut render_data: Vec<(&Position, &Renderable)> = query.iter(&self.world).collect();
        render_data.sort_by_key(|&k| k.0.z);
        for (position, renderable) in render_data {
            let screen_dest = mint::Point2 {
                x: position.x as f32 * TILE_WIDTH,
                y: position.y as f32 * TILE_HEIGHT,
            };

            let draw_params = graphics::DrawParam::default().dest(screen_dest);
            graphics::draw(ctx, renderable, draw_params)?;
        }

        graphics::present(ctx)
    }
}

//////////////////////////////////
// Components
//////////////////////////////////

/// Path to the image that represents the entity
enum Renderable {
    Image(graphics::Image),
}

impl graphics::Drawable for Renderable {
    /// Draws the drawable onto the rendering target.
    fn draw(&self, ctx: &mut ggez::Context, param: graphics::DrawParam) -> ggez::GameResult {
        match self {
            Renderable::Image(image) => image.draw(ctx, param),
        }
    }

    /// Returns a bounding box in the form of a `Rect`.
    ///
    /// It returns `Option` because some `Drawable`s may have no bounding box
    /// (an empty `SpriteBatch` for example).
    fn dimensions(&self, _ctx: &mut ggez::Context) -> Option<graphics::Rect> {
        match self {
            Renderable::Image(image) => Some(image.dimensions()),
        }
    }

    /// Sets the blend mode to be used when drawing this drawable.
    /// This overrides the general [`graphics::set_blend_mode()`](fn.set_blend_mode.html).
    /// If `None` is set, defers to the blend mode set by
    /// `graphics::set_blend_mode()`.
    fn set_blend_mode(&mut self, mode: Option<graphics::BlendMode>) {
        match self {
            Renderable::Image(image) => image.set_blend_mode(mode),
        }
    }

    /// Gets the blend mode to be used when drawing this drawable.
    fn blend_mode(&self) -> Option<graphics::BlendMode> {
        match self {
            Renderable::Image(image) => image.blend_mode(),
        }
    }
}

/// Position of the entity on the game's grid
type Position = mint::Point3<u8>;

/// Marks an entity as a player
#[derive(Default)]
struct Player;

/// Marks an entity as a box
#[derive(Default)]
struct Box;

/// Marks an entity as a wall
#[derive(Default)]
struct Wall;

/// Marks an entity as a location where a box can be put into
#[derive(Default)]
struct BoxSpot;

//////////////////////////////////
// Entities
//////////////////////////////////

/// Creates a player entity
fn create_player(
    ctx: &mut ggez::Context,
    world: &mut legion::World,
    player_pos: Position,
) -> ggez::GameResult<legion::Entity> {
    let image_path = path::PathBuf::from("/images/player_1.png");
    let image = graphics::Image::new(ctx, image_path)?;
    let player_entity = world.push((
        Player,
        Position {
            z: 10,
            ..player_pos
        },
        Renderable::Image(image),
    ));
    Ok(player_entity)
}

/// Creates a box entity
fn create_box(
    ctx: &mut ggez::Context,
    world: &mut legion::World,
    box_pos: Position,
) -> ggez::GameResult<legion::Entity> {
    let image_path = path::PathBuf::from("/images/box_red_1.png");
    let image = graphics::Image::new(ctx, image_path)?;
    let box_entity = world.push((Box, Position { z: 10, ..box_pos }, Renderable::Image(image)));
    Ok(box_entity)
}

/// Creates a wall entity
fn create_wall(
    ctx: &mut ggez::Context,
    world: &mut legion::World,
    wall_pos: Position,
) -> ggez::GameResult<legion::Entity> {
    let image_path = path::PathBuf::from("/images/wall.png");
    let image = graphics::Image::new(ctx, image_path)?;
    let wall_entity = world.push((
        Wall,
        Position { z: 10, ..wall_pos },
        Renderable::Image(image),
    ));
    Ok(wall_entity)
}

/// Creates a box spot entity
fn create_box_spot(
    ctx: &mut ggez::Context,
    world: &mut legion::World,
    box_spot_pos: Position,
) -> ggez::GameResult<legion::Entity> {
    let image_path = path::PathBuf::from("/images/box_spot_red.png");
    let image = graphics::Image::new(ctx, image_path)?;
    let box_spot_entity = world.push((
        BoxSpot,
        Position {
            z: 9,
            ..box_spot_pos
        },
        Renderable::Image(image),
    ));
    Ok(box_spot_entity)
}

/// Creates a floor entity
fn create_floor(
    ctx: &mut ggez::Context,
    world: &mut legion::World,
    floor_pos: Position,
) -> ggez::GameResult<legion::Entity> {
    let image_path = path::PathBuf::from("/images/floor.png");
    let image = graphics::Image::new(ctx, image_path)?;
    let floor_entity = world.push((Position { z: 5, ..floor_pos }, Renderable::Image(image)));
    Ok(floor_entity)
}

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

    let game = &mut Game::new(ctx)?;
    event::run(ctx, evts_loop, game)
}
