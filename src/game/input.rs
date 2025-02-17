use crate::input::{GameInput, Movement};

use super::player::Player;
use std::time::Instant;

pub struct PlayerMovementTimestamps {
    x: Instant,
    y: Instant,
}

impl Default for PlayerMovementTimestamps {
    fn default() -> Self {
        let now = Instant::now();
        Self { x: now, y: now }
    }
}

pub fn handle_player_input<T: GameInput>(
    input: &T,
    player: &mut Player,
    last_moved: &mut PlayerMovementTimestamps,
) {
    let activation_durations = input.activation_times();
    let Movement { x, y } = input.movement();
    let now = Instant::now();

    if activation_durations
        .x
        .is_some_and(|x| now.duration_since(last_moved.x) >= x)
    {
        last_moved.x = now;
        player.inc_x(x);
    }

    if activation_durations
        .y
        .is_some_and(|y| now.duration_since(last_moved.y) >= y)
    {
        last_moved.y = now;
        player.inc_y(y);
    }
}
