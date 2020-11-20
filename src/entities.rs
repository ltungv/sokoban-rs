use crate::components;
use crate::game::{MAP_HEIGHT, MAP_WIDTH};

/// Parse the map that is given as a string and create entities based on the characters
/// in the map string
pub fn load_from_map_str(world: &mut legion::World, map_str: &str) -> ggez::GameResult {
    let rows: Vec<&str> = map_str.split('\n').map(|x| x.trim()).collect();
    for (y, row) in rows.iter().enumerate() {
        let cols: Vec<&str> = row.split(' ').collect();
        if rows.len() != MAP_HEIGHT as usize || cols.len() != MAP_WIDTH as usize {
            panic!("Incorrect map dimensions");
        }

        for (x, col) in cols.iter().enumerate() {
            let position = components::Position {
                x: x as u8,
                y: y as u8,
                z: 0, // this will be modified when the entity is created
            };

            match *col {
                // BOX
                "BB" => {
                    create_floor(world, position)?;
                    create_box(world, position, components::BoxColor::Blue)?;
                }
                "RB" => {
                    create_floor(world, position)?;
                    create_box(world, position, components::BoxColor::Red)?;
                }
                // BOX SPOT
                "BS" => {
                    create_floor(world, position)?;
                    create_box_spot(world, position, components::BoxColor::Blue)?;
                }
                "RS" => {
                    create_floor(world, position)?;
                    create_box_spot(world, position, components::BoxColor::Red)?;
                }
                // PLAYER
                "P" => {
                    create_floor(world, position)?;
                    create_player(world, position)?;
                }
                // WALL
                "W" => {
                    create_wall(world, position)?;
                }
                // NO ITEM
                "." => {
                    create_floor(world, position)?;
                }
                // NOTHING
                "N" => {}
                // ERROR
                c => panic!("Invalid map item {}", c),
            }
        }
    }
    Ok(())
}

/// Creates a player entity
pub fn create_player(
    world: &mut legion::World,
    player_pos: components::Position,
) -> ggez::GameResult<legion::Entity> {
    // Load the sprites that are used for animation
    let mut paths = Vec::new();
    for i in 1..4 {
        paths.push(format!("/images/player_{}.png", i));
    }

    // Create a player entity
    let player_entity = world.push((
        components::Player,
        components::Movable,
        components::Position {
            z: 10,
            ..player_pos
        },
        components::Renderable::new_animated(paths),
    ));
    Ok(player_entity)
}

/// Creates a box entity
pub fn create_box(
    world: &mut legion::World,
    box_pos: components::Position,
    color: components::BoxColor,
) -> ggez::GameResult<legion::Entity> {
    // Load the sprites that are used for animation
    let mut paths = Vec::new();
    for i in 1..3 {
        paths.push(match color {
            components::BoxColor::Red => format!("/images/box_red_{}.png", i),
            components::BoxColor::Blue => format!("/images/box_blue_{}.png", i),
        });
    }

    // Create a new box entity
    let box_entity = world.push((
        components::Box { color },
        components::Movable,
        components::Position { z: 10, ..box_pos },
        components::Renderable::new_animated(paths),
    ));
    Ok(box_entity)
}

/// Creates a wall entity
pub fn create_wall(
    world: &mut legion::World,
    wall_pos: components::Position,
) -> ggez::GameResult<legion::Entity> {
    // Create a wall entity
    let wall_entity = world.push((
        components::Wall,
        components::Immovable,
        components::Position { z: 10, ..wall_pos },
        components::Renderable::new_static("/images/wall.png"),
    ));
    Ok(wall_entity)
}

/// Creates a box spot entity
pub fn create_box_spot(
    world: &mut legion::World,
    box_spot_pos: components::Position,
    color: components::BoxColor,
) -> ggez::GameResult<legion::Entity> {
    // Load static sprite
    let path = match color {
        components::BoxColor::Red => "/images/box_spot_red.png",
        components::BoxColor::Blue => "/images/box_spot_blue.png",
    };
    // Create a box spot entity
    let box_spot_entity = world.push((
        components::BoxSpot { color },
        components::Position {
            z: 9,
            ..box_spot_pos
        },
        components::Renderable::new_static(path),
    ));
    Ok(box_spot_entity)
}

/// Creates a floor entity
pub fn create_floor(
    world: &mut legion::World,
    floor_pos: components::Position,
) -> ggez::GameResult<legion::Entity> {
    // Create a floor entity
    let floor_entity = world.push((
        components::Position { z: 5, ..floor_pos },
        components::Renderable::new_static("/images/floor.png"),
    ));
    Ok(floor_entity)
}
