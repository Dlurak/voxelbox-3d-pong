use super::{ball::Ball, player::Player};
use gilrs::{Axis, Gamepad, Gilrs};
use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

const IGNORE_THRESHOLD: f32 = 0.15;

const MIN_DELAY: Duration = Duration::from_millis(50);
const MAX_DELAY: Duration = Duration::from_millis(300);

fn compute_delay(controller_axis_value: f32, sensitivity: f32) -> Duration {
    let normalized_axis =
        (controller_axis_value.abs() - IGNORE_THRESHOLD) / (1.0 - IGNORE_THRESHOLD);
    let ms_range = (MAX_DELAY - MIN_DELAY).as_millis() as f32;
    let extra_ms = (ms_range * (1.0 - normalized_axis.powf(sensitivity))).round() as u64;

    MIN_DELAY + Duration::from_millis(extra_ms)
}

fn should_move(axis_value: f32, last_move: Instant, sensitivity: f32) -> bool {
    let now = Instant::now();

    let input_is_strong = axis_value.abs() > IGNORE_THRESHOLD;
    let delay_is_ok = now - last_move >= compute_delay(axis_value, sensitivity);

    input_is_strong && delay_is_ok
}

fn handle_player_axis(
    gp: &Gamepad,
    axis: Axis,
    last_moved: Instant,
    player: &Arc<Mutex<Player>>,
    sensitivity: f32,
) -> Option<Instant> {
    let axis_value = gp.value(axis);

    if !should_move(axis_value, last_moved, sensitivity) {
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

pub fn handle_input(
    player_1: (Arc<Mutex<Player>>, f32),
    player_2: (Arc<Mutex<Player>>, f32),
    ball: Arc<Mutex<Ball>>,
    gilrs: &mut Gilrs,
    gamepad_id: gilrs::GamepadId,
) {
    let mut last_moved_player1_x = Instant::now();
    let mut last_moved_player1_y = Instant::now();
    let mut last_moved_player2_x = Instant::now();
    let mut last_moved_player2_y = Instant::now();
    let mut last_moved_ball = Instant::now();

    loop {
        gilrs.next_event();
        let gp = gilrs.gamepad(gamepad_id);

        last_moved_player1_x = handle_player_axis(
            &gp,
            Axis::LeftStickX,
            last_moved_player1_x,
            &player_1.0,
            player_1.1,
        )
        .unwrap_or(last_moved_player1_x);
        last_moved_player1_y = handle_player_axis(
            &gp,
            Axis::LeftStickY,
            last_moved_player1_y,
            &player_1.0,
            player_1.1,
        )
        .unwrap_or(last_moved_player1_y);

        last_moved_player2_x = handle_player_axis(
            &gp,
            Axis::RightStickX,
            last_moved_player2_x,
            &player_2.0,
            player_2.1,
        )
        .unwrap_or(last_moved_player2_x);
        last_moved_player2_y = handle_player_axis(
            &gp,
            Axis::RightStickY,
            last_moved_player2_y,
            &player_2.0,
            player_2.1,
        )
        .unwrap_or(last_moved_player2_y);

        let now = Instant::now();
        if (now - last_moved_ball).as_millis() >= 1_000 {
            last_moved_ball = now;
            let mut ball = ball.lock().unwrap();

            let colliding_sides =
                ball.colliding_sides(&player_1.0.lock().unwrap(), &player_2.0.lock().unwrap());
            ball.change_direction(&colliding_sides);
            ball.apply_movement();
        }
    }
}
