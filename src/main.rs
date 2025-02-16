mod color;
mod game;
mod log;
mod macros;
mod odd;
mod positive;
mod prelude;
mod voxelbox;

use clap::Parser;
use game::input::handle_input;
use gilrs::Gilrs;
use log::Severity;
use positive::Positive;
use prelude::*;
use std::{
    sync::{Arc, LazyLock, Mutex},
    thread,
    time::Duration,
};

const FPS: f32 = 10.0;
static RENDER_FRAME_DURATION: LazyLock<Duration> =
    LazyLock::new(|| Duration::from_secs_f32(1.0 / FPS));

fn render_loop(
    voxelbox: &Arc<Mutex<voxelbox::Voxelbox>>,
    player_1: &Arc<Mutex<game::player::Player>>,
    player_2: &Arc<Mutex<game::player::Player>>,
    ball: &Arc<Mutex<game::ball::Ball>>,
) {
    loop {
        thread::sleep(*RENDER_FRAME_DURATION);

        let mut vbox = voxelbox.lock().unwrap();
        vbox.reset_leds();
        player_1
            .lock()
            .unwrap()
            .draw_pad(&mut vbox)
            .log(Severity::Warning, "Unable to draw player 1");
        player_2
            .lock()
            .unwrap()
            .draw_pad(&mut vbox)
            .log(Severity::Warning, "Unable to draw player 2");
        ball.lock().unwrap().draw(&mut vbox);
        vbox.send()
            .log(Severity::Warning, "Could not send pixel-data to Voxelbox");
    }
}

fn sensitivity_parser(s: &str) -> Result<Positive<f32>, String> {
    s.parse()
        .map_err(|_| format!("{s} isn't a number"))
        .and_then(|n| Positive::new(n).ok_or_else(|| format!("{s} is bigger than 0")))
}

#[derive(Parser)]
#[command(version, about = "3d Pong on the Voxelbox", long_about = None)]
struct Args {
    /// Sensitivity of Player 1 (Green), controls paddle speed
    #[arg(
        long,
        visible_alias = "sens-p1",
        default_value_t = Positive::new(1.5).unwrap(),
        value_parser = sensitivity_parser
    )]
    sensitivity_p1: Positive<f32>,
    #[arg(
        long,
        visible_alias = "sens-p2",
        default_value_t = Positive::new(1.5).unwrap(),
        value_parser = sensitivity_parser
    )]
    /// Sensitivity of Player 2 (Yellow), controls paddle speed
    sensitivity_p2: Positive<f32>,
    /// IP-Address of the Voxelbox
    #[arg(
        long,
        default_value_t = String::from("127.0.0.1"),
    )]
    ip: String,
    /// Port of the Voxelbox
    #[arg(
        long,
        default_value_t = 5005,
        value_parser = clap::value_parser!(u16).range(1..)
    )]
    port: u16,
}

fn main() {
    let args = Args::parse();

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
        );
    });
    let render_thread =
        thread::spawn(move || render_loop(&voxelbox_clone, &player_1, &player_2, &ball));

    input_thread
        .join()
        .expect("The input handling thread paniced");
    render_thread.join().expect("The rendering thread paniced");
}
