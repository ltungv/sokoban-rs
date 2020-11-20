use ggez::audio::{self, SoundSource};
use ggez::input::keyboard;

use std::collections;
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

impl Default for GamePlayState {
    fn default() -> Self {
        Self::Playing
    }
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
#[derive(Default)]
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
    pub sounds: collections::HashMap<String, audio::Source>,
}

impl AudioStore {
    /// Play the audio asset at the given index
    pub fn play_sound(&mut self, sound: &str) {
        let _ = self
            .sounds
            .get_mut(sound)
            .expect("expected sound")
            .play_detached();
    }
}
