use ggez::input::keyboard;
use legion::query::IntoQuery;
use legion::system;
use legion::world::EntityStore;

use std::collections;

use crate::components;
use crate::game::{MAP_HEIGHT, MAP_WIDTH};
use crate::resources;

/// Drain the key pressed events queue and modify the player's sprite position
/// based on the received keycode
#[system]
#[read_component(components::Player)]
#[read_component(components::Moveable)]
#[read_component(components::Immovable)]
#[write_component(components::Position)]
pub fn input_handling(
    world: &mut legion::world::SubWorld,
    #[resource] keyboard_events: &mut resources::KeyBoardEventQueue,
) {
    let mut to_move = Vec::new();
    let mut players_query = <(&components::Player, &components::Position)>::query();

    for (_p, player_position) in players_query.iter(world) {
        if let Some(keycode) = keyboard_events.keys_pressed.pop() {
            let mut moveables_query =
                <(&components::Moveable, &components::Position, legion::Entity)>::query();
            let moveables: collections::HashMap<(u8, u8), legion::Entity> = moveables_query
                .iter(world)
                .map(|(_m, position, entity)| ((position.x, position.y), *entity))
                .collect();

            let mut immovables_query = <(
                &components::Immovable,
                &components::Position,
                legion::Entity,
            )>::query();
            let immovables: collections::HashMap<(u8, u8), legion::Entity> = immovables_query
                .iter(world)
                .map(|(_m, position, entity)| ((position.x, position.y), *entity))
                .collect();

            let (start, end, is_xaxis) = match keycode {
                keyboard::KeyCode::Up => (player_position.y, 0, false),
                keyboard::KeyCode::Down => (player_position.y, MAP_HEIGHT, false),
                keyboard::KeyCode::Left => (player_position.x, 0, true),
                keyboard::KeyCode::Right => (player_position.x, MAP_WIDTH, true),
                _ => continue,
            };

            let range = if start < end {
                (start..=end).collect::<Vec<_>>()
            } else {
                (end..=start).rev().collect::<Vec<_>>()
            };

            for x_or_y in range {
                let pos = if is_xaxis {
                    (x_or_y, player_position.y)
                } else {
                    (player_position.x, x_or_y)
                };

                match moveables.get(&pos) {
                    Some(movable) => to_move.push((keycode, *movable)),
                    None => match immovables.get(&pos) {
                        Some(_) => to_move.clear(),
                        None => break,
                    },
                }
            }
        }
    }

    for (keycode, movable) in to_move {
        if let Ok(mut entry) = world.entry_mut(movable) {
            if let Ok(mut position) = entry.get_component_mut::<components::Position>() {
                match keycode {
                    keyboard::KeyCode::Up => position.y = position.y.saturating_sub(1),
                    keyboard::KeyCode::Down => position.y = position.y.saturating_add(1),
                    keyboard::KeyCode::Left => position.x = position.x.saturating_sub(1),
                    keyboard::KeyCode::Right => position.x = position.x.saturating_add(1),
                    _ => continue,
                }
            }
        }
    }
}
