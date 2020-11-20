use ggez::graphics;

use std::path;

use crate::components;
use crate::game::{TILE_HEIGHT, TILE_WIDTH};

/// Creates a player entity
pub fn create_player(
    ctx: &mut ggez::Context,
    world: &mut legion::World,
    player_pos: components::Position,
) -> ggez::GameResult<legion::Entity> {
    // Load the sprites that are used for animation
    let mut animations = Vec::new();
    for i in 1..4 {
        let image_path = path::PathBuf::from(format!("/images/player_{}.png", i));
        let mut image = graphics::Image::new(ctx, image_path)?;
        image.set_filter(graphics::FilterMode::Nearest);
        animations.push(image);
    }
    let image_dims = animations[0].dimensions();

    // Create a player entity
    let player_entity = world.push((
        components::Player,
        components::Movable,
        components::Position {
            z: 10,
            ..player_pos
        },
        components::Scale {
            x: TILE_WIDTH / image_dims.w,
            y: TILE_HEIGHT / image_dims.h,
        },
        components::Renderable::new_animated(animations),
    ));
    Ok(player_entity)
}

/// Creates a box entity
pub fn create_box(
    ctx: &mut ggez::Context,
    world: &mut legion::World,
    box_pos: components::Position,
    color: components::BoxColor,
) -> ggez::GameResult<legion::Entity> {
    // Load the sprites that are used for animation
    let mut animations = Vec::new();
    for i in 1..3 {
        let image_path = match color {
            components::BoxColor::Red => path::PathBuf::from(format!("/images/box_red_{}.png", i)),
            components::BoxColor::Blue => {
                path::PathBuf::from(format!("/images/box_blue_{}.png", i))
            }
        };
        let mut image = graphics::Image::new(ctx, image_path)?;
        image.set_filter(graphics::FilterMode::Nearest);
        animations.push(image);
    }
    let image_dims = animations[0].dimensions();

    // Create a new box entity
    let box_entity = world.push((
        components::Box { color },
        components::Movable,
        components::Position { z: 10, ..box_pos },
        components::Scale {
            x: TILE_WIDTH / image_dims.w,
            y: TILE_HEIGHT / image_dims.h,
        },
        components::Renderable::new_animated(animations),
    ));
    Ok(box_entity)
}

/// Creates a wall entity
pub fn create_wall(
    ctx: &mut ggez::Context,
    world: &mut legion::World,
    wall_pos: components::Position,
) -> ggez::GameResult<legion::Entity> {
    // Load static sprite
    let image_path = path::PathBuf::from("/images/wall.png");
    let mut image = graphics::Image::new(ctx, image_path)?;
    image.set_filter(graphics::FilterMode::Nearest);
    let image_dims = image.dimensions();

    // Create a wall entity
    let wall_entity = world.push((
        components::Wall,
        components::Immovable,
        components::Position { z: 10, ..wall_pos },
        components::Scale {
            x: TILE_WIDTH / image_dims.w,
            y: TILE_HEIGHT / image_dims.h,
        },
        components::Renderable::new_static(image),
    ));
    Ok(wall_entity)
}

/// Creates a box spot entity
pub fn create_box_spot(
    ctx: &mut ggez::Context,
    world: &mut legion::World,
    box_spot_pos: components::Position,
    color: components::BoxColor,
) -> ggez::GameResult<legion::Entity> {
    // Load static sprite
    let image_path = match color {
        components::BoxColor::Red => path::PathBuf::from("/images/box_spot_red.png"),
        components::BoxColor::Blue => path::PathBuf::from("/images/box_spot_blue.png"),
    };
    let mut image = graphics::Image::new(ctx, image_path)?;
    image.set_filter(graphics::FilterMode::Nearest);
    let image_dims = image.dimensions();

    // Create a box spot entity
    let box_spot_entity = world.push((
        components::BoxSpot { color },
        components::Position {
            z: 9,
            ..box_spot_pos
        },
        components::Scale {
            x: TILE_WIDTH / image_dims.w,
            y: TILE_HEIGHT / image_dims.h,
        },
        components::Renderable::new_static(image),
    ));
    Ok(box_spot_entity)
}

/// Creates a floor entity
pub fn create_floor(
    ctx: &mut ggez::Context,
    world: &mut legion::World,
    floor_pos: components::Position,
) -> ggez::GameResult<legion::Entity> {
    // Load static sprite
    let image_path = path::PathBuf::from("/images/floor.png");
    let mut image = graphics::Image::new(ctx, image_path)?;
    image.set_filter(graphics::FilterMode::Nearest);
    let image_dims = image.dimensions();

    // Create a floor entity
    let floor_entity = world.push((
        components::Position { z: 5, ..floor_pos },
        components::Scale {
            x: TILE_WIDTH / image_dims.w,
            y: TILE_HEIGHT / image_dims.h,
        },
        components::Renderable::new_static(image),
    ));
    Ok(floor_entity)
}
