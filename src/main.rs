mod cli;
mod color;
mod game;
mod log;
mod macros;
mod odd;
mod positive;
mod prelude;
mod voxelbox;

use clap::Parser;
use game::{input::handle_input, render_loop};
use gilrs::Gilrs;
use log::Severity;
use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let args = cli::Args::parse();

    let mut gilrs = Gilrs::new().expect("Failed to initialize gilrs, needed to get controllers");
    let mut gamepads = gilrs.gamepads();
    let gp_id = gamepads
        .next()
        .unwrap_or_else(|| {
            log!(Critical, "Plese connect a gampepad");
            std::process::exit(1);
        })
        .0;
    let gp_id_2 = gamepads.next().map(|v| v.0);
    let log_msg = match gp_id_2 {
        Some(_) => "Both player have a own gamepad",
        None => "Both player share one gamepad",
    };
    log!(Log, "{}", log_msg);

    let mut game_state = game::state::GameState::default();

    let voxelbox = Arc::new(Mutex::new(voxelbox::Voxelbox::new(args.ip, args.port)));
    let player_1 = Arc::new(Mutex::new(game::player::Player::player_1()));
    let player_2 = Arc::new(Mutex::new(game::player::Player::player_2()));
    let ball = Arc::new(Mutex::new(game::ball::Ball::default()));

    let voxelbox_clone = Arc::clone(&voxelbox);
    let player_1_clone = Arc::clone(&player_1);
    let player_2_clone = Arc::clone(&player_2);
    let ball_clone = Arc::clone(&ball);

    let input_thread = thread::spawn(move || {
        handle_input(
            (player_1_clone, args.sensitivity_p1),
            (player_2_clone, args.sensitivity_p2),
            ball_clone,
            &mut game_state,
            &mut gilrs,
            (gp_id, gp_id_2),
            args.winning_points,
        );
    });
    let render_thread =
        thread::spawn(move || render_loop(&voxelbox_clone, &player_1, &player_2, &ball));

    input_thread
        .join()
        .expect("The input handling thread paniced");
    render_thread.join().expect("The rendering thread paniced");
}
