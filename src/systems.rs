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

/// Draw all renderable entities and information in some resources to screen
pub fn render_entities(
    ctx: &mut ggez::Context,
    world: &legion::World,
    resources: &legion::Resources,
) -> ggez::GameResult {
    // Renderable components to be queried
    type RenderableArchetype<'a> = (
        &'a components::Renderable<graphics::Image>,
        &'a components::Position,
        &'a components::Scale,
    );
    // Query for getting renderable entities
    let mut renderables_data = <RenderableArchetype>::query()
        .iter(world)
        .collect::<Vec<RenderableArchetype>>();
    renderables_data.sort_by_key(|&k| k.1.z);

    // Show all renderables
    for (renderable, position, scale) in renderables_data {
        let screen_dest = mint::Point2 {
            x: position.x as f32 * TILE_WIDTH,
            y: position.y as f32 * TILE_HEIGHT,
        };
        let draw_params = graphics::DrawParam::default()
            .dest(screen_dest)
            .scale(*scale);
        let time_alive = match resources.get::<resources::Time>() {
            Some(time) => time.alive,
            None => std::time::Duration::default(),
        };

        // Determine the keyframe of the animation based on the time the has passed since the game
        // was first started
        let drawable = renderable.drawable(match renderable.kind() {
            components::RenderableKind::Static => 0,
            components::RenderableKind::Animated => {
                ((time_alive.as_millis() % 2000) / 500) as usize
            }
        });
        graphics::draw(ctx, &drawable, draw_params)?;
    }
    Ok(())
}

pub fn render_gameplay_data(
    ctx: &mut ggez::Context,
    resources: &legion::Resources,
) -> ggez::GameResult {
    // Show number of moves that have been taken and whether the player has won
    if let Some(game_play) = resources.get::<resources::GamePlay>() {
        let text_color = graphics::Color::new(0.0, 0.0, 0.0, 1.0);
        let mut text = graphics::Text::default();
        text.add(graphics::TextFragment::new(game_play.state.to_string()).color(text_color))
            .add(graphics::TextFragment::new("\n"))
            .add(graphics::TextFragment::new(game_play.steps_taken.to_string()).color(text_color));

        let text_draw_dest = mint::Point2 {
            x: (TILE_WIDTH * MAP_WIDTH as f32 - text.dimensions(ctx).0 as f32) / 2.0,
            y: (TILE_HEIGHT * MAP_HEIGHT as f32 - text.dimensions(ctx).1 as f32) / 2.0,
        };
        let draw_params = graphics::DrawParam::new().dest(text_draw_dest);
        graphics::draw(ctx, &text, draw_params)?;
    }
    Ok(())
}

/// Consume key pressed events from queue and modify the player's sprite position
/// based on the received keycode
#[system]
#[read_component(components::Player)]
#[read_component(components::Movable)]
#[read_component(components::Immovable)]
#[write_component(components::Position)]
pub fn input_handling(
    world: &mut legion::world::SubWorld,
    #[resource] key_pressed_events: &mut resources::KeyPressedEventQueue,
    #[resource] gameplay_events: &mut resources::GamePlayEventQueue,
    #[resource] gameplay: &mut resources::GamePlay,
) {
    type PositionEntityHashMap = collections::HashMap<(u8, u8), legion::Entity>;
    type MovableArchetype<'a> = (
        &'a components::Movable,
        &'a components::Position,
        legion::Entity,
    );
    type ImmovableArchetype<'a> = (
        &'a components::Immovable,
        &'a components::Position,
        legion::Entity,
    );
    type PlayerArchetype<'a> = (&'a components::Player, &'a components::Position);

    // Get all movable entities
    let movables = <MovableArchetype>::query()
        .iter(world)
        .map(|(_m, position, entity)| ((position.x, position.y), *entity))
        .collect::<PositionEntityHashMap>();
    // Get all immovable entities
    let immovables = <ImmovableArchetype>::query()
        .iter(world)
        .map(|(_m, position, entity)| ((position.x, position.y), *entity))
        .collect::<PositionEntityHashMap>();

    // Iterate through all entities starting from the player's position on the game map
    // and moving along the axis that is defined by the keyboard input, and check for each
    // entity if it can be moved
    let mut to_move = Vec::new();
    for (_p, player_position) in <PlayerArchetype>::query().iter(world) {
        if let Some(keycode) = key_pressed_events.queue.pop() {
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

                match movables.get(&pos) {
                    // If encounter a movable entity, add it to list of movable entities
                    Some(movable) => to_move.push((keycode, movable)),
                    // Otherwise, check if the entity is immovable
                    None => {
                        if immovables.get(&pos).is_some() {
                            gameplay_events
                                .queue
                                .push(resources::GamePlayEvent::HitObstacle);
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

    // Move all entities that can be moved
    for (keycode, movable) in to_move {
        gameplay_events
            .queue
            .push(resources::GamePlayEvent::EntityMoved(*movable));
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

/// Check if all the boxes have been put in their correct position, if so, change the state of the
/// game from `Playing` to `Won`
#[system]
#[read_component(components::Box)]
#[read_component(components::BoxSpot)]
#[read_component(components::Position)]
pub fn game_objective(
    world: &mut legion::world::SubWorld,
    #[resource] game_play: &mut resources::GamePlay,
) {
    let mut boxes_query = <(&components::Box, &components::Position)>::query();
    let boxes: collections::HashMap<(u8, u8), &components::Box> = boxes_query
        .iter(world)
        .map(|(b, position)| ((position.x, position.y), b))
        .collect();

    game_play.state = resources::GamePlayState::Playing;
    let mut box_spots_query = <(&components::BoxSpot, &components::Position)>::query();
    for (box_spot, box_spot_position) in box_spots_query.iter(world) {
        if let Some(the_box) = boxes.get(&(box_spot_position.x, box_spot_position.y)) {
            if the_box.color == box_spot.color {
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

/// Consume all events that were generated by the game play
#[system]
#[read_component(components::Box)]
#[read_component(components::BoxSpot)]
#[read_component(components::Position)]
pub fn consume_gameplay_events(
    world: &mut legion::world::SubWorld,
    #[resource] gameplay_events: &mut resources::GamePlayEventQueue,
    #[resource] audio_store: &mut resources::AudioStore,
) {
    let mut new_events = Vec::new();
    for event in gameplay_events.queue.drain(..) {
        match event {
            resources::GamePlayEvent::HitObstacle => audio_store.play_sound("wall"),
            resources::GamePlayEvent::EntityMoved(entity) => {
                if let Ok(entry) = world.entry_ref(entity) {
                    if let Ok(the_box) = entry.get_component::<components::Box>() {
                        if let Ok(box_position) = entry.get_component::<components::Position>() {
                            let mut box_spots_query =
                                <(&components::BoxSpot, &components::Position)>::query();
                            let box_spots: collections::HashMap<(u8, u8), &components::BoxSpot> =
                                box_spots_query
                                    .iter(world)
                                    .map(|(b, position)| ((position.x, position.y), b))
                                    .collect();
                            if let Some(box_spot) = box_spots.get(&(box_position.x, box_position.y))
                            {
                                new_events.push(resources::GamePlayEvent::BoxSpacedOnSpot(
                                    box_spot.color == the_box.color,
                                ));
                            }
                        }
                    }
                }
            }
            resources::GamePlayEvent::BoxSpacedOnSpot(is_same_color) => {
                audio_store.play_sound(if is_same_color {
                    "correct"
                } else {
                    "incorrect"
                })
            }
        }
    }
    gameplay_events.queue.append(&mut new_events);
}
