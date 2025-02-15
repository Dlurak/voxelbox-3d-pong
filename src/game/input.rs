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
    gamepad_id: (gilrs::GamepadId, Option<gilrs::GamepadId>),
) {
    let mut last_moved_player1_x = Instant::now();
    let mut last_moved_player1_y = Instant::now();
    let mut last_moved_player2_x = Instant::now();
    let mut last_moved_player2_y = Instant::now();
    let mut last_moved_ball = Instant::now();

    loop {
        gilrs.next_event();
        let gp = gilrs.gamepad(gamepad_id.0);
        let gp_2 = gamepad_id.1.map(|id| gilrs.gamepad(id));

        last_moved_player1_x = handle_player_axis(
            &gp,
            Axis::LeftStickX,
            last_moved_player1_x,
            &player_1.0,
            player_1.1,
            false,
        )
        .unwrap_or(last_moved_player1_x);
        last_moved_player1_y = handle_player_axis(
            &gp,
            Axis::LeftStickY,
            last_moved_player1_y,
            &player_1.0,
            player_1.1,
            false,
        )
        .unwrap_or(last_moved_player1_y);

        let p2_own_gamepad = gp_2.is_some();
        let p2_x_stick = if p2_own_gamepad {
            Axis::LeftStickX
        } else {
            Axis::RightStickX
        };
        let p2_y_stick = if p2_own_gamepad {
            Axis::LeftStickY
        } else {
            Axis::RightStickY
        };
        last_moved_player2_x = handle_player_axis(
            &gp_2.unwrap_or(gp),
            p2_x_stick,
            last_moved_player2_x,
            &player_2.0,
            player_2.1,
            true,
        )
        .unwrap_or(last_moved_player2_x);
        last_moved_player2_y = handle_player_axis(
            &gp_2.unwrap_or(gp),
            p2_y_stick,
            last_moved_player2_y,
            &player_2.0,
            player_2.1,
            false,
        )
        .unwrap_or(last_moved_player2_y);

        let now = Instant::now();
        let mut ball = ball.lock().unwrap();
        if now - last_moved_ball >= ball.movement_intervall {
            last_moved_ball = now;

            let colliding_sides =
                ball.colliding_sides(&player_1.0.lock().unwrap(), &player_2.0.lock().unwrap());
            ball.change_direction(&colliding_sides);
            ball.apply_movement();
        }
    }
}
