use ggez::audio::{self, SoundSource};
use ggez::graphics;
use ggez::input::keyboard;

use std::collections as colls;
use std::time;

#[derive(Debug, Default)]
pub struct Time {
    pub alive: time::Duration,
}

#[derive(Debug)]
pub enum GamePlayState {
    Playing,
    Won,
}

impl std::fmt::Display for GamePlayState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            GamePlayState::Playing => "Playing",
            GamePlayState::Won => "Won",
        })
    }
}

#[derive(Debug)]
pub enum GamePlayEvent {
    HitObstacle,
    EntityMoved(legion::Entity),
    BoxSpacedOnSpot(bool),
}

#[derive(Debug)]
pub struct GamePlay {
    pub state: GamePlayState,
    pub steps_taken: u32,
}

impl Default for GamePlay {
    fn default() -> Self {
        Self {
            state: GamePlayState::Playing,
            steps_taken: 0,
        }
    }
}

#[derive(Default)]
pub struct GamePlayEventQueue {
    pub queue: Vec<GamePlayEvent>,
}

#[derive(Default)]
pub struct KeyPressedEventQueue {
    pub queue: Vec<keyboard::KeyCode>,
}

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

    pub fn play_sound(&mut self, sound_path: &str) {
        if let Some(sound) = self.sounds.get_mut(sound_path) {
            if sound.play_detached().is_err() { /* ignore */ };
        }
    }
}

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

    pub fn get_image(&self, image_path: &str) -> Option<&graphics::Image> {
        self.images.get(image_path)
    }
}
