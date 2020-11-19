use ggez::event;
use ggez::graphics::{self, Drawable};
use ggez::mint;
use legion::query::IntoQuery;

use std::path;

use crate::components;

pub const TILE_WIDTH: f32 = 48.0;
pub const TILE_HEIGHT: f32 = 48.0;

pub struct Game {
    world: legion::World,
}

impl Game {
    pub fn new(ctx: &mut ggez::Context) -> ggez::GameResult<Self> {
        let mut world = legion::World::default();
        for x in 0..4 {
            create_floor(ctx, &mut world, components::Position { x, y: 0, z: 0 })?;
        }
        create_wall(ctx, &mut world, components::Position { x: 0, y: 0, z: 0 })?;
        create_box(ctx, &mut world, components::Position { x: 1, y: 0, z: 0 })?;
        create_box_spot(ctx, &mut world, components::Position { x: 2, y: 0, z: 0 })?;
        create_player(ctx, &mut world, components::Position { x: 3, y: 0, z: 0 })?;
        Ok(Self { world })
    }
}

impl event::EventHandler for Game {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, graphics::WHITE);

        let mut query = <(&components::Position, &components::Renderable)>::query();
        let mut render_data: Vec<(&components::Position, &components::Renderable)> =
            query.iter(&self.world).collect();
        render_data.sort_by_key(|&k| k.0.z);
        for (position, renderable) in render_data {
            let screen_dest = mint::Point2 {
                x: position.x as f32 * TILE_WIDTH,
                y: position.y as f32 * TILE_HEIGHT,
            };

            let mut draw_params = graphics::DrawParam::default().dest(screen_dest);
            if let Some(renderable_dims) = renderable.dimensions(ctx) {
                draw_params = draw_params.scale(mint::Vector2 {
                    x: TILE_WIDTH / renderable_dims.w,
                    y: TILE_HEIGHT / renderable_dims.h,
                });
            }

            graphics::draw(ctx, renderable, draw_params)?;
        }

        graphics::present(ctx)
    }
}

/// Creates a player entity
fn create_player(
    ctx: &mut ggez::Context,
    world: &mut legion::World,
    player_pos: components::Position,
) -> ggez::GameResult<legion::Entity> {
    let image_path = path::PathBuf::from("/images/player_1.png");
    let mut image = graphics::Image::new(ctx, image_path)?;
    image.set_filter(graphics::FilterMode::Nearest);

    let player_entity = world.push((
        components::Player,
        components::Position {
            z: 10,
            ..player_pos
        },
        components::Renderable::Image(image),
    ));
    Ok(player_entity)
}

/// Creates a box entity
fn create_box(
    ctx: &mut ggez::Context,
    world: &mut legion::World,
    box_pos: components::Position,
) -> ggez::GameResult<legion::Entity> {
    let image_path = path::PathBuf::from("/images/box_red_1.png");
    let mut image = graphics::Image::new(ctx, image_path)?;
    image.set_filter(graphics::FilterMode::Nearest);

    let box_entity = world.push((
        components::Box,
        components::Position { z: 10, ..box_pos },
        components::Renderable::Image(image),
    ));
    Ok(box_entity)
}

/// Creates a wall entity
fn create_wall(
    ctx: &mut ggez::Context,
    world: &mut legion::World,
    wall_pos: components::Position,
) -> ggez::GameResult<legion::Entity> {
    let image_path = path::PathBuf::from("/images/wall.png");
    let mut image = graphics::Image::new(ctx, image_path)?;
    image.set_filter(graphics::FilterMode::Nearest);

    let wall_entity = world.push((
        components::Wall,
        components::Position { z: 10, ..wall_pos },
        components::Renderable::Image(image),
    ));
    Ok(wall_entity)
}

/// Creates a box spot entity
fn create_box_spot(
    ctx: &mut ggez::Context,
    world: &mut legion::World,
    box_spot_pos: components::Position,
) -> ggez::GameResult<legion::Entity> {
    let image_path = path::PathBuf::from("/images/box_spot_red.png");
    let mut image = graphics::Image::new(ctx, image_path)?;
    image.set_filter(graphics::FilterMode::Nearest);

    let box_spot_entity = world.push((
        components::BoxSpot,
        components::Position {
            z: 9,
            ..box_spot_pos
        },
        components::Renderable::Image(image),
    ));
    Ok(box_spot_entity)
}

/// Creates a floor entity
fn create_floor(
    ctx: &mut ggez::Context,
    world: &mut legion::World,
    floor_pos: components::Position,
) -> ggez::GameResult<legion::Entity> {
    let image_path = path::PathBuf::from("/images/floor.png");
    let mut image = graphics::Image::new(ctx, image_path)?;
    image.set_filter(graphics::FilterMode::Nearest);

    let floor_entity = world.push((
        components::Position { z: 5, ..floor_pos },
        components::Renderable::Image(image),
    ));
    Ok(floor_entity)
}
