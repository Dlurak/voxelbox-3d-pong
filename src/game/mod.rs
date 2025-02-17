use crate::{input::JoyStick, log::Severity, positive::Positive, prelude::*, voxelbox};
use ball_movement::{handle_ball_movement_and_score, update_game_state_and_reset};
use gilrs::Gilrs;
use input::{handle_player_input, PlayerMovementTimestamps};
use std::{
    num::NonZero,
    sync::LazyLock,
    time::{Duration, Instant},
};

pub mod ball;
pub mod ball_movement;
pub mod input;
pub mod pad;
pub mod player;
pub mod state;

const FPS: f32 = 10.0;
static RENDER_FRAME_DURATION: LazyLock<Duration> =
    LazyLock::new(|| Duration::from_secs_f32(1.0 / FPS));

struct MovementTimestamps {
    player_1: PlayerMovementTimestamps,
    player_2: PlayerMovementTimestamps,
    ball: Instant,
    render: Instant,
}

impl Default for MovementTimestamps {
    fn default() -> Self {
        Self {
            player_1: PlayerMovementTimestamps::default(),
            player_2: PlayerMovementTimestamps::default(),
            ball: Instant::now(),
            render: Instant::now(),
        }
    }
}

pub fn game_loop(
    player_1_sensitivity: Positive<f32>,
    player_2_sensitivity: Positive<f32>,
    connectivity: (String, u16),
    gilrs: &mut Gilrs,
    gamepad_id: (gilrs::GamepadId, Option<gilrs::GamepadId>),
    winning_points: NonZero<u8>,
) {
    let mut last_movements = MovementTimestamps::default();
    let mut state = state::GameState::default();

    let mut voxelbox = voxelbox::Voxelbox::new(connectivity.0, connectivity.1);
    let mut player_1 = player::Player::player_1();
    let mut player_2 = player::Player::player_2();
    let mut ball = ball::Ball::default();

    let mut player_1_joystick = JoyStick::new_player_1(gamepad_id.0, player_1_sensitivity);
    let mut player_2_joystick = JoyStick::new_player_2(
        gamepad_id.1.unwrap_or(gamepad_id.0),
        player_2_sensitivity,
        gamepad_id.1.is_some(),
    );

    loop {
        if let Some(event) = gilrs.next_event() {
            player_1_joystick.add_event(&event);
            player_2_joystick.add_event(&event);
        }

        handle_player_input(
            &player_1_joystick,
            &mut player_1,
            &mut last_movements.player_1,
        );
        handle_player_input(
            &player_2_joystick,
            &mut player_2,
            &mut last_movements.player_2,
        );
        let scoring_player = handle_ball_movement_and_score(
            &mut ball,
            &player_1,
            &player_2,
            &mut last_movements.ball,
        );
        if let Some(p) = scoring_player {
            let new_structs = update_game_state_and_reset(&p, &mut state, winning_points);

            player_1 = new_structs.0;
            player_2 = new_structs.1;
            ball = new_structs.2;
        }

        let now = Instant::now();
        let duration_since_last_render = now - last_movements.render;
        if duration_since_last_render >= *RENDER_FRAME_DURATION {
            render(&mut voxelbox, &player_1, &player_2, &ball);
            last_movements.render = now;
        }
    }
}

fn render(
    vbox: &mut voxelbox::Voxelbox,
    player_1: &player::Player,
    player_2: &player::Player,
    ball: &ball::Ball,
) {
    vbox.reset_leds();
    player_1
        .draw_pad(vbox)
        .log(Severity::Warning, "Unable to draw player 1");
    player_2
        .draw_pad(vbox)
        .log(Severity::Warning, "Unable to draw player 2");
    ball.draw(vbox);
    vbox.send()
        .log(Severity::Warning, "Could not send pixel-data to Voxelbox");
}
