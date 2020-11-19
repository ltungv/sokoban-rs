use ggez::graphics;
use ggez::input::keyboard;
use ggez::mint;
use legion::query::IntoQuery;
use legion::system;
use legion::world::EntityStore;

use std::collections;

use crate::components;
use crate::game::{MAP_HEIGHT, MAP_WIDTH, TILE_HEIGHT, TILE_WIDTH};
use crate::resources;

pub fn render(
    ctx: &mut ggez::Context,
    world: &legion::World,
    resources: &legion::Resources,
) -> ggez::GameResult {
    graphics::clear(ctx, graphics::WHITE);

    // Go through the entities that can be rendered to screen and get their data
    let mut renderables_query = <(&components::Position, &components::Renderable)>::query();
    let mut renderables_data = renderables_query
        .iter(world)
        .collect::<Vec<(&components::Position, &components::Renderable)>>();
    renderables_data.sort_by_key(|&k| k.0.z);

    for (position, renderable) in renderables_data {
        // draw position
        let screen_dest = mint::Point2 {
            x: position.x as f32 * TILE_WIDTH,
            y: position.y as f32 * TILE_HEIGHT,
        };
        let mut draw_params = graphics::DrawParam::default().dest(screen_dest);

        match renderable {
            components::Renderable::Image(image) => {
                // scale sprite to tile size
                let renderable_dims = image.dimensions();
                draw_params = draw_params.scale(mint::Vector2 {
                    x: TILE_WIDTH / renderable_dims.w,
                    y: TILE_HEIGHT / renderable_dims.h,
                });
                graphics::draw(ctx, image, draw_params)?;
            }
        }
    }

    // Show number of moves that have been taken and whether the player has won
    if let Some(game_play) = resources.get::<resources::GamePlay>() {
        draw_text(ctx, &game_play.state.to_string(), 525.0, 80.0)?;
        draw_text(ctx, &game_play.steps_taken.to_string(), 525.0, 100.0)?;
    }

    graphics::present(ctx)
}

pub fn draw_text(ctx: &mut ggez::Context, text_string: &str, x: f32, y: f32) -> ggez::GameResult {
    let text = graphics::Text::new(
        graphics::TextFragment::new(text_string).color(graphics::Color::new(0.0, 0.0, 0.0, 1.0)),
    );
    let dest = mint::Point2 { x, y };
    graphics::draw(ctx, &text, graphics::DrawParam::new().dest(dest))
}

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
    #[resource] gameplay: &mut resources::GamePlay,
) {
    // This vector contains entities whose position is changed by the input event
    let mut to_move = Vec::new();

    // Get all moveable entities
    let mut moveables_query =
        <(&components::Moveable, &components::Position, legion::Entity)>::query();
    let moveables: collections::HashMap<(u8, u8), legion::Entity> = moveables_query
        .iter(world)
        .map(|(_m, position, entity)| ((position.x, position.y), *entity))
        .collect();

    // Get all immoveable entities
    let mut immovables_query = <(
        &components::Immovable,
        &components::Position,
        legion::Entity,
    )>::query();
    let immovables: collections::HashMap<(u8, u8), &legion::Entity> = immovables_query
        .iter(world)
        .map(|(_m, position, entity)| ((position.x, position.y), entity))
        .collect();

    // Iterate through all entities starting from the player's position on the game map
    // and moving along the axis that is defined by the keyboard input, and check for each
    // entity if it can be moved
    let mut players_query = <(&components::Player, &components::Position)>::query();
    for (_p, player_position) in players_query.iter(world) {
        if let Some(keycode) = keyboard_events.keys_pressed.pop() {
            // Determine the range and axis to move along base on the input
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
                    // If encounter a movable entity, add it to list of moveable entities
                    Some(movable) => to_move.push((keycode, movable)),
                    // Otherwise, check if the entity is immovable
                    None => {
                        if immovables.get(&pos).is_some() {
                            to_move.clear();
                        }
                        break;
                    }
                }
            }
        }
    }

    if !to_move.is_empty() {
        gameplay.steps_taken += 1;
    }

    for (keycode, movable) in to_move {
        if let Ok(mut entry) = world.entry_mut(*movable) {
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

#[system]
#[read_component(components::Box)]
#[read_component(components::BoxSpot)]
#[read_component(components::Position)]
pub fn game_objective(
    world: &mut legion::world::SubWorld,
    #[resource] game_play: &mut resources::GamePlay,
) {
    let mut boxes_query = <(&components::Box, &components::Position)>::query();
    let boxes: collections::HashMap<(u8, u8), components::BoxColor> = boxes_query
        .iter(world)
        .map(|(b, position)| ((position.x, position.y), b.color))
        .collect();

    game_play.state = resources::GamePlayState::Playing;
    let mut box_spots_query = <(&components::BoxSpot, &components::Position)>::query();
    for (box_spot, box_spot_position) in box_spots_query.iter(world) {
        if let Some(color) = boxes.get(&(box_spot_position.x, box_spot_position.y)) {
            if *color == box_spot.color {
                continue;
            } else {
                return;
            }
        } else {
            return;
        }
    }
    game_play.state = resources::GamePlayState::Won;
}
