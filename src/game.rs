use ggez::event;
use ggez::graphics;
use ggez::input::keyboard;
use ggez::timer;

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

const SOUNDS: &[&str] = &[
    "/sounds/wall.wav",
    "/sounds/correct.wav",
    "/sounds/incorrect.wav",
];

const IMAGES: &[&str] = &[
    "/images/box_blue_1.png",
    "/images/box_blue_2.png",
    "/images/box_red_1.png",
    "/images/box_red_2.png",
    "/images/box_spot_blue.png",
    "/images/box_spot_red.png",
    "/images/floor.png",
    "/images/player_1.png",
    "/images/player_2.png",
    "/images/player_3.png",
    "/images/wall.png",
];

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
        entities::load_from_map_str(&mut world, map_str)?;

        let mut audio_store = resources::AudioStore::default();
        load_sounds(ctx, &mut audio_store, SOUNDS)?;

        let mut drawable_store = resources::DrawableStore::default();
        load_images(ctx, &mut drawable_store, IMAGES)?;

        let mut resources = legion::Resources::default();
        resources.insert(resources::Time::default());
        resources.insert(resources::GamePlay {
            state: resources::GamePlayState::Playing,
            steps_taken: 0,
        });
        resources.insert(resources::KeyPressedEventQueue::default());
        resources.insert(resources::GamePlayEventQueue::default());
        resources.insert(audio_store);
        resources.insert(drawable_store);

        let update_schedule = legion::Schedule::builder()
            .add_system(systems::input_handling_system())
            .add_system(systems::game_objective_system())
            .add_system(systems::consume_gameplay_events_system())
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
            if let Some(mut time) = self.resources.get_mut::<resources::Time>() {
                time.alive += timer::delta(ctx);
            }
            self.update_schedule
                .execute(&mut self.world, &mut self.resources);
        }
        Ok(())
    }

    /// This method is run on each game tick to render the entities to screen
    /// based on the world's data
    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, graphics::WHITE);
        systems::render_entities(ctx, &self.world, &self.resources)?;
        systems::render_gameplay_data(ctx, &self.resources)?;
        graphics::present(ctx)
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
        let key_pressed_events = self.resources.get_mut::<resources::KeyPressedEventQueue>();
        if let Some(mut key_pressed_events) = key_pressed_events {
            key_pressed_events.queue.push(keycode);
        };
    }
}

fn load_sounds(
    ctx: &mut ggez::Context,
    audio_store: &mut resources::AudioStore,
    sounds: &[&str],
) -> ggez::GameResult {
    for sound_path in sounds.iter() {
        audio_store.add_sound(ctx, sound_path)?;
    }
    Ok(())
}

fn load_images(
    ctx: &mut ggez::Context,
    drawable_store: &mut resources::DrawableStore,
    images: &[&str],
) -> ggez::GameResult {
    for image_path in images.iter() {
        drawable_store.add_image(ctx, image_path, graphics::FilterMode::Nearest)?;
    }
    Ok(())
}
