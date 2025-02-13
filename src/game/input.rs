use gilrs::{Axis, Gamepad};
use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use super::player::Player;

const IGNORE_THRESHOLD: f32 = 0.2;

const MIN_DELAY: Duration = Duration::from_millis(50);
const MAX_DELAY: Duration = Duration::from_millis(300);

fn compute_delay(controller_axis_value: f32) -> Duration {
    let normalized_axis =
        (controller_axis_value.abs() - IGNORE_THRESHOLD) / (1.0 - IGNORE_THRESHOLD);
    let normalized = normalized_axis * normalized_axis;
    let ms_range = (MAX_DELAY - MIN_DELAY).as_millis() as f32;
    let extra_ms = (ms_range * (1.0 - normalized)).round() as u64;

    MIN_DELAY + Duration::from_millis(extra_ms)
}

fn should_move(axis_value: f32, last_move: Instant) -> bool {
    let now = Instant::now();

    let input_is_strong = axis_value.abs() > IGNORE_THRESHOLD;
    let delay_is_ok = now - last_move >= compute_delay(axis_value);

    input_is_strong && delay_is_ok
}

pub fn handle_player_axis(
    gp: &Gamepad,
    axis: Axis,
    last_moved: Instant,
    player: &Arc<Mutex<Player>>,
) -> Option<Instant> {
    let axis_value = gp.value(axis);

    if !should_move(axis_value, last_moved) {
        return None;
    }

    let movement_size = axis_value.signum() as i16;
    let movement_size = if axis == Axis::RightStickX {
        movement_size
    } else {
        -movement_size
    };

    match axis {
        Axis::LeftStickX | Axis::RightStickX => {
            player.lock().unwrap().inc_x(movement_size);
            Some(Instant::now())
        }
        Axis::LeftStickY | Axis::RightStickY => {
            player.lock().unwrap().inc_y(movement_size);
            Some(Instant::now())
        }
        _ => None,
    }
}
