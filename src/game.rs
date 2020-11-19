use ggez::event;
use ggez::input::keyboard;
use ggez::timer;

use crate::components;
use crate::entities;
use crate::resources;
use crate::systems;

/// Tile's width when rendered to screen
pub const TILE_WIDTH: f32 = 48.0;

/// Tile's height when rendered to screen
pub const TILE_HEIGHT: f32 = 48.0;

/// Width of the grid system
pub const MAP_WIDTH: u8 = 9;

/// Height of the grid system
pub const MAP_HEIGHT: u8 = 9;

/// This structure holds access to the game's `World` and implements `EventHandler` to updates and
/// render entities on each game tick
pub struct Game {
    world: legion::World,
    resources: legion::Resources,
    update_schedule: legion::Schedule,
}

impl Game {
    /// Create a new game state based on the given context and initializes entities based on the
    /// given map represented in string
    pub fn new(ctx: &mut ggez::Context, map_str: &str) -> ggez::GameResult<Self> {
        let mut world = legion::World::default();
        load_map(ctx, &mut world, map_str)?;

        let mut resources = legion::Resources::default();
        resources.insert(resources::KeyBoardEventQueue::default());
        resources.insert(resources::GamePlay::default());

        let update_schedule = legion::Schedule::builder()
            .add_system(systems::input_handling_system())
            .add_system(systems::game_objective_system())
            .build();

        Ok(Self {
            world,
            resources,
            update_schedule,
        })
    }
}

impl event::EventHandler for Game {
    /// This method is run on each game tick to update the world's data
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        const FPS: u32 = 60;
        while timer::check_update_time(ctx, FPS) {
            self.update_schedule
                .execute(&mut self.world, &mut self.resources);
        }
        Ok(())
    }

    /// This method is run on each game tick to render the entities to screen
    /// based on the world's data
    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        systems::render(ctx, &self.world, &self.resources)
    }

    /// Handle keydown event
    fn key_down_event(
        &mut self,
        ctx: &mut ggez::Context,
        keycode: keyboard::KeyCode,
        _keymods: keyboard::KeyMods,
        _repeat: bool,
    ) {
        if keycode == keyboard::KeyCode::Escape {
            event::quit(ctx);
        }

        // Push key code the the event queue
        let maybe_keyboard_event_queue = self.resources.get_mut::<resources::KeyBoardEventQueue>();
        if let Some(mut keyboard_event_queue) = maybe_keyboard_event_queue {
            keyboard_event_queue.keys_pressed.push(keycode);
        };
    }
}

/// Parse the map that is given as a string and create entities based on the characters
/// in the map string
fn load_map(ctx: &mut ggez::Context, world: &mut legion::World, map_str: &str) -> ggez::GameResult {
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
                // PLAYER
                "P" => {
                    entities::create_floor(ctx, world, position)?;
                    entities::create_player(ctx, world, position)?;
                }
                // BOX
                "BB" => {
                    entities::create_floor(ctx, world, position)?;
                    entities::create_box(ctx, world, position, components::BoxColor::Blue)?;
                }
                "RB" => {
                    entities::create_floor(ctx, world, position)?;
                    entities::create_box(ctx, world, position, components::BoxColor::Red)?;
                }
                // WALL
                "W" => {
                    entities::create_wall(ctx, world, position)?;
                }
                // BOX SPOT
                "BS" => {
                    entities::create_floor(ctx, world, position)?;
                    entities::create_box_spot(ctx, world, position, components::BoxColor::Blue)?;
                }
                "RS" => {
                    entities::create_floor(ctx, world, position)?;
                    entities::create_box_spot(ctx, world, position, components::BoxColor::Red)?;
                }
                // NO ITEM
                "." => {
                    entities::create_floor(ctx, world, position)?;
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
