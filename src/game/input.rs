use super::{
    ball::Ball,
    ball_movement::{handle_ball_movement_and_score, update_game_state_and_reset},
    player::Player,
    state::GameState,
};
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

struct MovementTimestamps {
    player_1: PlayerMovementTimestamps,
    player_2: PlayerMovementTimestamps,
    ball: Instant,
}

impl Default for MovementTimestamps {
    fn default() -> Self {
        Self {
            player_1: PlayerMovementTimestamps::default(),
            player_2: PlayerMovementTimestamps::default(),
            ball: Instant::now(),
        }
    }
}

struct PlayerMovementTimestamps {
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

fn update_player_movement(
    gamepad: &gilrs::Gamepad,
    player: &(Arc<Mutex<Player>>, f32),
    movement: &mut PlayerMovementTimestamps,
    additional_information: (Axis, bool, Axis),
) {
    movement.x = handle_player_axis(
        gamepad,
        additional_information.0,
        movement.x,
        &player.0,
        player.1,
        additional_information.1,
    )
    .unwrap_or(movement.x);
    movement.y = handle_player_axis(
        gamepad,
        additional_information.2,
        movement.y,
        &player.0,
        player.1,
        false,
    )
    .unwrap_or(movement.y);
}

pub fn handle_input(
    player_1: (Arc<Mutex<Player>>, f32),
    player_2: (Arc<Mutex<Player>>, f32),
    ball: Arc<Mutex<Ball>>,
    state: &mut GameState,
    gilrs: &mut Gilrs,
    gamepad_id: (gilrs::GamepadId, Option<gilrs::GamepadId>),
) {
    let mut last_movements = MovementTimestamps::default();

    loop {
        gilrs.next_event();
        let gp = gilrs.gamepad(gamepad_id.0);
        let gp_2 = gamepad_id.1.map(|id| gilrs.gamepad(id));

        update_player_movement(
            &gp,
            &player_1,
            &mut last_movements.player_1,
            (Axis::LeftStickX, false, Axis::LeftStickY),
        );

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
        update_player_movement(
            &gp_2.unwrap_or(gp),
            &player_2,
            &mut last_movements.player_2,
            (p2_x_stick, true, p2_y_stick),
        );

        let scoring_player = handle_ball_movement_and_score(
            &ball,
            &player_1.0,
            &player_2.0,
            &mut last_movements.ball,
        );
        if let Some(p) = scoring_player {
            let new_structs = update_game_state_and_reset(&p, state);

            *player_1.0.lock().unwrap() = new_structs.0;
            *player_2.0.lock().unwrap() = new_structs.1;
            *ball.lock().unwrap() = new_structs.2;
        }
    }
}
