use crate::positive::Positive;

use super::player::Player;
use gilrs::{Axis, Gamepad};
use std::time::{Duration, Instant};

const IGNORE_THRESHOLD: f32 = 0.15;

const MIN_DELAY: Duration = Duration::from_millis(50);
const MAX_DELAY: Duration = Duration::from_millis(300);

fn compute_delay(controller_axis_value: f32, sensitivity: &Positive<f32>) -> Duration {
    let normalized_axis =
        (controller_axis_value.abs() - IGNORE_THRESHOLD) / (1.0 - IGNORE_THRESHOLD);
    let ms_range = (MAX_DELAY - MIN_DELAY).as_millis() as f32;
    let extra_ms = (ms_range * (1.0 - normalized_axis.powf(sensitivity.value()))).round() as u64;

    MIN_DELAY + Duration::from_millis(extra_ms)
}

fn should_move(axis_value: f32, last_move: Instant, sensitivity: &Positive<f32>) -> bool {
    let now = Instant::now();

    let input_is_strong = axis_value.abs() > IGNORE_THRESHOLD;
    let delay_is_ok = now - last_move >= compute_delay(axis_value, sensitivity);

    input_is_strong && delay_is_ok
}

fn handle_player_axis(
    gp: &Gamepad,
    axis: Axis,
    last_moved: Instant,
    player: &mut Player,
    sensitivity: &Positive<f32>,
    reverse: bool,
) -> Option<Instant> {
    let axis_value = gp.value(axis);

    if !should_move(axis_value, last_moved, sensitivity) {
        return None;
    }

    let movement_size = axis_value.signum() as i16;
    let movement_size = if reverse {
        movement_size
    } else {
        -movement_size
    };

    match axis {
        Axis::LeftStickX | Axis::RightStickX => {
            player.inc_x(movement_size);
            Some(Instant::now())
        }
        Axis::LeftStickY | Axis::RightStickY => {
            player.inc_y(movement_size);
            Some(Instant::now())
        }
        _ => None,
    }
}

pub struct PlayerMovementTimestamps {
    x: Instant,
    y: Instant,
}

impl Default for PlayerMovementTimestamps {
    fn default() -> Self {
        Self {
            x: Instant::now(),
            y: Instant::now(),
        }
    }
}

pub fn update_player_movement(
    gamepad: &gilrs::Gamepad,
    player: (&mut Player, &Positive<f32>),
    movement: &mut PlayerMovementTimestamps,
    additional_information: (Axis, bool, Axis),
) {
    movement.x = handle_player_axis(
        gamepad,
        additional_information.0,
        movement.x,
        player.0,
        player.1,
        additional_information.1,
    )
    .unwrap_or(movement.x);
    movement.y = handle_player_axis(
        gamepad,
        additional_information.2,
        movement.y,
        player.0,
        player.1,
        false,
    )
    .unwrap_or(movement.y);
}

pub const fn player_2_sticks(own_gamepad: bool) -> (Axis, Axis) {
    if own_gamepad {
        (Axis::LeftStickX, Axis::LeftStickY)
    } else {
        (Axis::RightStickX, Axis::RightStickY)
    }
}
