use ggez::input::keyboard;

#[derive(Default)]
pub struct KeyBoardEventQueue {
    pub keys_pressed: Vec<keyboard::KeyCode>,
}
