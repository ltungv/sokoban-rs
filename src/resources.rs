use ggez::input::keyboard;

/// Queue of keyboard events
#[derive(Default)]
pub struct KeyBoardEventQueue {
    pub keys_pressed: Vec<keyboard::KeyCode>,
}

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
