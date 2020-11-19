use ggez::graphics;

use std::path;

use crate::components;

/// Creates a player entity
pub fn create_player(
    ctx: &mut ggez::Context,
    world: &mut legion::World,
    player_pos: components::Position,
) -> ggez::GameResult<legion::Entity> {
    let image_path = path::PathBuf::from("/images/player_1.png");
    let mut image = graphics::Image::new(ctx, image_path)?;
    image.set_filter(graphics::FilterMode::Nearest);

    let player_entity = world.push((
        components::Player,
        components::Moveable,
        components::Position {
            z: 10,
            ..player_pos
        },
        components::Renderable::Image(image),
    ));
    Ok(player_entity)
}

/// Creates a box entity
pub fn create_box(
    ctx: &mut ggez::Context,
    world: &mut legion::World,
    box_pos: components::Position,
) -> ggez::GameResult<legion::Entity> {
    let image_path = path::PathBuf::from("/images/box_red_1.png");
    let mut image = graphics::Image::new(ctx, image_path)?;
    image.set_filter(graphics::FilterMode::Nearest);

    let box_entity = world.push((
        components::Box,
        components::Moveable,
        components::Position { z: 10, ..box_pos },
        components::Renderable::Image(image),
    ));
    Ok(box_entity)
}

/// Creates a wall entity
pub fn create_wall(
    ctx: &mut ggez::Context,
    world: &mut legion::World,
    wall_pos: components::Position,
) -> ggez::GameResult<legion::Entity> {
    let image_path = path::PathBuf::from("/images/wall.png");
    let mut image = graphics::Image::new(ctx, image_path)?;
    image.set_filter(graphics::FilterMode::Nearest);

    let wall_entity = world.push((
        components::Wall,
        components::Immovable,
        components::Position { z: 10, ..wall_pos },
        components::Renderable::Image(image),
    ));
    Ok(wall_entity)
}

/// Creates a box spot entity
pub fn create_box_spot(
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
pub fn create_floor(
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