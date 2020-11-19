use ggez::input::keyboard;
use legion::system;

use crate::components;
use crate::resources;

/// Drain the key pressed events queue and modify the player's sprite position
/// based on the received keycode
#[system(for_each)]
pub fn player_control(
    _player: &components::Player,
    position: &mut components::Position,
    #[resource] keyboard_events: &mut resources::KeyBoardEventQueue,
) {
    while let Some(keycode) = keyboard_events.keys_pressed.pop() {
        match keycode {
            keyboard::KeyCode::Right => position.x = position.x.wrapping_add(1),
            keyboard::KeyCode::Down => position.y = position.y.wrapping_add(1),
            keyboard::KeyCode::Left => position.x = position.x.saturating_sub(1),
            keyboard::KeyCode::Up => position.y = position.y.saturating_sub(1),
            _ => {}
        }
    }
}
