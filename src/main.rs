mod cli;
mod color;
mod input;
mod game;
mod log;
mod macros;
mod odd;
mod positive;
mod prelude;
mod voxelbox;

use clap::Parser;
use game::game_loop;
use gilrs::Gilrs;
use log::Severity;

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

    game_loop(
        args.sensitivity_p1,
        args.sensitivity_p2,
        (args.ip, args.port),
        &mut gilrs,
        (gp_id, gp_id_2),
        args.winning_points,
    );
}
