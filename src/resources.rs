use ggez::audio::{self, SoundSource};
use ggez::input::keyboard;

use std::collections;
use std::time;

/// Queue of keyboard events
#[derive(Default)]
pub struct KeyPressedEventQueue {
    pub queue: Vec<keyboard::KeyCode>,
}

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

#[derive(Default)]
pub struct GamePlay {
    pub state: GamePlayState,
    pub steps_taken: u32,
}

#[derive(Default)]
pub struct Time {
    pub alive: time::Duration,
}

pub enum GamePlayEvent {
    HitObstacle,
    EntityMoved(legion::Entity),
    BoxSpacedOnSpot(bool),
}

#[derive(Default)]
pub struct GamePlayEventQueue {
    pub queue: Vec<GamePlayEvent>,
}

#[derive(Default)]
pub struct AudioStore {
    pub sounds: collections::HashMap<String, audio::Source>,
}

impl AudioStore {
    pub fn play_sound(&mut self, sound: &str) {
        let _ = self
            .sounds
            .get_mut(sound)
            .expect("expected sound")
            .play_detached();
    }
}
