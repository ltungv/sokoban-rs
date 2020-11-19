use ggez::input::keyboard;

/// Queue of keyboard events
#[derive(Default)]
pub struct KeyBoardEventQueue {
    pub keys_pressed: Vec<keyboard::KeyCode>,
}
