use crate::components;
use crate::game::{MAP_HEIGHT, MAP_WIDTH};

pub fn create_entities_from_map(
    world: &mut legion::World,
    map: Vec<(components::Position, &str)>,
) -> ggez::GameResult {
    for (position, val) in map {
        if position.x >= MAP_WIDTH || position.y > MAP_HEIGHT {
            return Err(ggez::GameError::ResourceLoadError(
                "Could not load game map!".to_string(),
            ));
        }
        match val {
            // blue box
            "BB" => {
                create_floor(world, position);
                create_box(world, position, components::BoxColor::Blue);
            }
            // red box
            "RB" => {
                create_floor(world, position);
                create_box(world, position, components::BoxColor::Red);
            }
            // blue box destination
            "BS" => {
                create_floor(world, position);
                create_box_spot(world, position, components::BoxColor::Blue);
            }
            // red box destination
            "RS" => {
                create_floor(world, position);
                create_box_spot(world, position, components::BoxColor::Red);
            }
            // player initial position
            "P" => {
                create_floor(world, position);
                create_player(world, position);
            }
            // wall
            "W" => {
                create_wall(world, position);
            }
            // no item
            "." => {
                create_floor(world, position);
            }
            // empty space
            "N" => {}
            // unknown
            c => panic!("Invalid map item {}", c),
        }
    }

    Ok(())
}

pub fn create_player(world: &mut legion::World, pos: components::Position) -> legion::Entity {
    world.push((
        components::Player,
        components::Movable,
        components::Position { z: 10, ..pos },
        components::Renderable::new_animated(vec![
            "/images/player_1.png".to_string(),
            "/images/player_2.png".to_string(),
            "/images/player_3.png".to_string(),
        ]),
    ))
}

pub fn create_box(
    world: &mut legion::World,
    pos: components::Position,
    color: components::BoxColor,
) -> legion::Entity {
    let paths = match color {
        components::BoxColor::Red => vec![
            "/images/box_red_1.png".to_string(),
            "/images/box_red_2.png".to_string(),
        ],
        components::BoxColor::Blue => vec![
            "/images/box_blue_1.png".to_string(),
            "/images/box_blue_2.png".to_string(),
        ],
    };

    world.push((
        components::Box { color },
        components::Movable,
        components::Position { z: 10, ..pos },
        components::Renderable::new_animated(paths),
    ))
}

pub fn create_wall(world: &mut legion::World, pos: components::Position) -> legion::Entity {
    world.push((
        components::Wall,
        components::Immovable,
        components::Position { z: 10, ..pos },
        components::Renderable::new_static("/images/wall.png".to_string()),
    ))
}

pub fn create_box_spot(
    world: &mut legion::World,
    pos: components::Position,
    color: components::BoxColor,
) -> legion::Entity {
    let path = match color {
        components::BoxColor::Red => "/images/box_spot_red.png".to_string(),
        components::BoxColor::Blue => "/images/box_spot_blue.png".to_string(),
    };

    world.push((
        components::BoxSpot { color },
        components::Position { z: 9, ..pos },
        components::Renderable::new_static(path),
    ))
}

pub fn create_floor(world: &mut legion::World, floor_pos: components::Position) -> legion::Entity {
    world.push((
        components::Position { z: 5, ..floor_pos },
        components::Renderable::new_static("/images/floor.png".to_string()),
    ))
}
