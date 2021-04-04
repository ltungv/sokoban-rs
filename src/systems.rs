use ggez::graphics;
use ggez::graphics::spritebatch;
use ggez::input::keyboard;
use ggez::mint;
use itertools::Itertools;
use legion::query::IntoQuery;
use legion::system;
use legion::world::EntityStore;

use std::cmp;
use std::collections;

use crate::components;
use crate::game::{MAP_HEIGHT, MAP_WIDTH, TILE_HEIGHT, TILE_WIDTH};
use crate::resources;

/// Draw all renderable entities and information in some resources to screen by creating render batches
/// from based on the renderable entity's data.
///
/// # Notes
///
/// The renderable entities are first separated by their z-axis levels, in each z-axis level, the renderable
/// entities are then separated by their resource's name. For each resource's name, there is a list of
/// parameters that specify how all the entities, that depend on the resource, are rendered. This ensures:
/// + Entities with lower z-axis level are rendered first.
/// + The images are loaded with minimal access to memory.
pub fn render_entities(
    ctx: &mut ggez::Context,
    world: &legion::World,
    resources: &legion::Resources,
) -> ggez::GameResult {
    if let Some(drawable_store) = resources.get::<resources::DrawableStore>() {
        let time_alive = resources
            .get::<resources::Time>()
            .and_then(|time| Some(time.alive))
            .unwrap_or_default();

        let mut renderable_batches = collections::HashMap::<
            u8,
            collections::HashMap<String, Vec<graphics::DrawParam>>,
        >::new();

        <(&components::Renderable, &components::Position)>::query()
            .iter(world)
            .for_each(|(renderable, position)| {
                let image_idx = match renderable.kind() {
                    components::RenderableKind::Static => 0,
                    components::RenderableKind::Animated => {
                        ((time_alive.as_millis() % 2000) / 500) as usize
                    }
                };
                let image_path = renderable.path(image_idx);

                let draw_dest = mint::Point2 {
                    x: position.x as f32 * TILE_WIDTH,
                    y: position.y as f32 * TILE_HEIGHT,
                };
                let draw_params = graphics::DrawParam::default().dest(draw_dest);

                renderable_batches
                    .entry(position.z)
                    .or_default()
                    .entry(image_path.to_string())
                    .or_default()
                    .push(draw_params);
            });

        for (_z, group) in renderable_batches
            .iter()
            .sorted_by(|a, b| cmp::Ord::cmp(&a.0, &b.0))
        {
            for (image_path, draw_params) in group {
                if let Some(image) = drawable_store.get_image(image_path) {
                    let mut sprite_batch = spritebatch::SpriteBatch::new(image.clone());
                    draw_params.iter().for_each(|p| {
                        let p = p.scale(mint::Vector2 {
                            x: TILE_WIDTH / image.width() as f32,
                            y: TILE_HEIGHT / image.height() as f32,
                        });
                        sprite_batch.add(p);
                    });
                    graphics::draw(ctx, &sprite_batch, graphics::DrawParam::new())?;
                }
            }
        }
    }
    Ok(())
}

/// Render the current state of the game and display whether the game's objectives have been accomplished.
///
/// # Examples
///
/// ```txt
/// Playing
/// Moves: 12
/// FPS: 44.7
/// ```
pub fn render_gameplay_data(
    ctx: &mut ggez::Context,
    resources: &legion::Resources,
) -> ggez::GameResult {
    if let Some(game_play) = resources.get::<resources::GamePlay>() {
        let text_color = graphics::Color::new(0.0, 0.0, 0.0, 1.0);

        let txt_gameplay_state =
            graphics::TextFragment::new(game_play.state.to_string()).color(text_color);
        let txt_steps_taken =
            graphics::TextFragment::new(format!("Moves: {}", game_play.steps_taken.to_string()))
                .color(text_color);
        let txt_fps = graphics::TextFragment::new(format!("FPS: {:.2}", ggez::timer::fps(ctx)))
            .color(text_color);

        let mut text = graphics::Text::default();
        text
            // State of the game: PLAYING | WON.
            .add(txt_gameplay_state)
            .add(graphics::TextFragment::new("\n"))
            // Number of moves that have been made.
            .add(txt_steps_taken)
            .add(graphics::TextFragment::new("\n"))
            // Number of frames per second that the game is rendered at.
            .add(txt_fps);

        let draw_dest = mint::Point2 {
            x: TILE_WIDTH * MAP_WIDTH as f32 + 50.0,
            y: (TILE_HEIGHT * MAP_HEIGHT as f32 - text.dimensions(ctx).1 as f32) / 2.0,
        };
        let draw_params = graphics::DrawParam::new().dest(draw_dest);

        graphics::draw(ctx, &text, draw_params)?;
    }
    Ok(())
}

/// Consume key pressed events from queue and modify the player's sprite position
/// based on the received keycode. If a player pushes a moveable item into an
/// immovable item, then both the player and the moveable item will not change
/// position. If a player pushes a moveable item into another moveable item or
/// an empty position, then the player and all the moveable items will change
/// position
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
    let mut query_movables =
        <(&components::Movable, &components::Position, legion::Entity)>::query();

    let movables = query_movables
        .iter(world)
        .map(|(_m, position, entity)| ((position.x, position.y), *entity))
        .collect::<collections::HashMap<(u8, u8), legion::Entity>>();

    let mut query_immovables = <(
        &components::Immovable,
        &components::Position,
        legion::Entity,
    )>::query();

    let immovables = query_immovables
        .iter(world)
        .map(|(_m, position, entity)| ((position.x, position.y), *entity))
        .collect::<collections::HashMap<(u8, u8), legion::Entity>>();

    // Iterate through all entities starting from the player's position on the game map
    // and moving along the axis that is defined by the keyboard input, and check for each
    // entity if it can be moved
    if let Some(keycode) = key_pressed_events.queue.pop() {
        let mut to_move = Vec::new();

        <(&components::Player, &components::Position)>::query()
            .iter(world)
            .for_each(|(_p, player_pos)| {
                // Determine the range and axis to move along base on the input
                let (start, end, is_xaxis) = match keycode {
                    keyboard::KeyCode::Up => (player_pos.y, 0, false),
                    keyboard::KeyCode::Down => (player_pos.y, MAP_HEIGHT, false),
                    keyboard::KeyCode::Left => (player_pos.x, 0, true),
                    keyboard::KeyCode::Right => (player_pos.x, MAP_WIDTH, true),
                    _ => return,
                };

                let range = if start < end {
                    (start..=end).collect::<Vec<_>>()
                } else {
                    (end..=start).rev().collect::<Vec<_>>()
                };

                for x_or_y in range {
                    let pos = if is_xaxis {
                        (x_or_y, player_pos.y)
                    } else {
                        (player_pos.x, x_or_y)
                    };

                    match movables.get(&pos) {
                        // If encounter a movable entity, add it to list of movable entities
                        Some(movable) => to_move.push(movable),
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
            });

        if !to_move.is_empty() {
            gameplay.steps_taken += 1;
        }

        // Move all entities that can be moved
        for movable in to_move {
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
    let boxes = boxes_query
        .iter(world)
        .map(|(b, position)| ((position.x, position.y), b))
        .collect::<collections::HashMap<(u8, u8), &components::Box>>();

    game_play.state = resources::GamePlayState::Playing;
    let mut box_spots_query = <(&components::BoxSpot, &components::Position)>::query();
    for (box_spot, box_spot_position) in box_spots_query.iter(world) {
        match boxes.get(&(box_spot_position.x, box_spot_position.y)) {
            Some(the_box) => {
                if the_box.color != box_spot.color {
                    return;
                }
            }
            None => return,
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
    gameplay_events.queue.drain(..).for_each(|evt| match evt {
        resources::GamePlayEvent::HitObstacle => audio_store.play_sound("/sounds/wall.wav"),
        resources::GamePlayEvent::EntityMoved(entity) => {
            if let Ok(entry) = world.entry_ref(entity) {
                if let (Ok(the_box), Ok(box_pos)) = (
                    entry.get_component::<components::Box>(),
                    entry.get_component::<components::Position>(),
                ) {
                    let mut box_spots_query =
                        <(&components::BoxSpot, &components::Position)>::query();
                    box_spots_query
                        .iter(world)
                        .for_each(|(box_spot, box_spot_pos)| {
                            if box_pos.x == box_spot_pos.x && box_pos.y == box_spot_pos.y {
                                new_events.push(resources::GamePlayEvent::BoxSpacedOnSpot(
                                    box_spot.color == the_box.color,
                                ));
                            }
                        });
                }
            }
        }
        resources::GamePlayEvent::BoxSpacedOnSpot(is_same_color) => {
            audio_store.play_sound(if is_same_color {
                "/sounds/correct.wav"
            } else {
                "/sounds/incorrect.wav"
            })
        }
    });

    gameplay_events.queue.append(&mut new_events);
}
