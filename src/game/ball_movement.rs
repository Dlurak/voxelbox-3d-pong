use crate::log;

use super::{
    ball::{Ball, CollisionSide},
    player::Player,
    state,
};
use std::{
    num::NonZero,
    sync::{Arc, Mutex},
    time::Instant,
};

pub fn handle_ball_movement_and_score(
    ball: &Arc<Mutex<Ball>>,
    player_1: &Arc<Mutex<Player>>,
    player_2: &Arc<Mutex<Player>>,
    last_move: &mut Instant,
) -> Option<state::Player> {
    let now = Instant::now();
    let mut ball = ball.lock().unwrap();

    if now.duration_since(*last_move) >= ball.movement_intervall {
        *last_move = now;

        let colliding_sides =
            ball.colliding_sides(&player_1.lock().unwrap(), &player_2.lock().unwrap());

        ball.change_direction(&colliding_sides);
        ball.apply_movement();
        drop(ball);

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
) -> (Player, Player, Ball) {
    state.score(player);
    log!(Log, "{} Scored ({})", player, state.fmt_score());

    if let Some(winner) = state.winner() {
        log!(Success, "{} won ({})", winner, state.fmt_score());
        std::process::exit(0);
    }

    let new_x = match player {
        super::state::Player::Player1 => NonZero::new(1).unwrap(),
        super::state::Player::Player2 => NonZero::new(-1).unwrap(),
    };
    (
        Player::player_1(),
        Player::player_2(),
        Ball::new_with_x(new_x),
    )
}
