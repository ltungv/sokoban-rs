use ggez::audio::{self, SoundSource};
use ggez::graphics;
use ggez::input::keyboard;

use std::collections as colls;
use std::time;

/// Queue of keyboard events
#[derive(Default)]
pub struct KeyPressedEventQueue {
    pub queue: Vec<keyboard::KeyCode>,
}

/// Determines whether the player has won the game
pub enum GamePlayState {
    Playing,
    Won,
}

impl std::fmt::Display for GamePlayState {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            GamePlayState::Playing => "Playing",
            GamePlayState::Won => "Won",
        })?;
        Ok(())
    }
}

/// State of the game
pub struct GamePlay {
    pub state: GamePlayState,
    pub steps_taken: u32,
}

/// Time resources
#[derive(Default)]
pub struct Time {
    pub alive: time::Duration,
}

/// Events that can happen while playing the game
pub enum GamePlayEvent {
    HitObstacle,
    EntityMoved(legion::Entity),
    BoxSpacedOnSpot(bool),
}

/// Queue of the events that were generated
#[derive(Default)]
pub struct GamePlayEventQueue {
    pub queue: Vec<GamePlayEvent>,
}

/// Mapping to the audio file
#[derive(Default)]
pub struct AudioStore {
    sounds: colls::HashMap<String, audio::Source>,
}

impl AudioStore {
    pub fn add_sound(&mut self, ctx: &mut ggez::Context, sound_path: &str) -> ggez::GameResult {
        let sound_source = audio::Source::new(ctx, sound_path)?;
        self.sounds.insert(sound_path.to_string(), sound_source);
        Ok(())
    }

    /// Play the audio asset at the given index
    pub fn play_sound(&mut self, sound_path: &str) {
        if let Some(sound) = self.sounds.get_mut(sound_path) {
            if sound.play_detached().is_err() { /* ignore */ };
        }
    }
}

/// Mapping to the image
#[derive(Default)]
pub struct DrawableStore {
    images: colls::HashMap<String, graphics::Image>,
}

impl DrawableStore {
    pub fn add_image(
        &mut self,
        ctx: &mut ggez::Context,
        image_path: &str,
        filter: graphics::FilterMode,
    ) -> ggez::GameResult {
        let mut image = graphics::Image::new(ctx, image_path)?;
        image.set_filter(filter);
        self.images.insert(image_path.to_string(), image);
        Ok(())
    }

    /// Play the audio asset at the given index
    pub fn get_image(&self, image_path: &str) -> Option<&graphics::Image> {
        self.images.get(image_path)
    }
}
