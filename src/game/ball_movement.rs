use crate::log;

use super::{
    ball::{Ball, CollisionSide},
    player::Player,
    state,
};
use std::{num::NonZero, time::Instant};

pub fn handle_ball_movement_and_score(
    ball: &mut Ball,
    player_1: &Player,
    player_2: &Player,
    last_move: &mut Instant,
) -> Option<state::Player> {
    let now = Instant::now();

    if now.duration_since(*last_move) >= ball.movement_intervall {
        *last_move = now;

        let colliding_sides = ball.colliding_sides(player_1, player_2);

        ball.change_direction(&colliding_sides);
        ball.apply_movement();

        if colliding_sides.contains(&CollisionSide::Right) {
            Some(state::Player::Player1)
        } else if colliding_sides.contains(&CollisionSide::Left) {
            Some(state::Player::Player2)
        } else {
            None
        }
    } else {
        None
    }
}

pub fn update_game_state_and_reset(
    player: &state::Player,
    state: &mut state::GameState,
    winning_points: NonZero<u8>,
) -> (Player, Player, Ball) {
    state.score(player);
    log!(Log, "{} Scored ({})", player, state.fmt_score());

    if let Some(winner) = state.winner(winning_points) {
        log!(Success, "{} won ({})", winner, state.fmt_score());
        std::process::exit(0);
    }

    let new_x = match player {
        state::Player::Player1 => NonZero::new(1).unwrap(),
        state::Player::Player2 => NonZero::new(-1).unwrap(),
    };
    (
        Player::player_1(),
        Player::player_2(),
        Ball::new_with_x(new_x),
    )
}
