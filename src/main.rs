mod color;
mod game;
mod odd;
mod prelude;
mod voxelbox;

use game::input::handle_player_axis;
use gilrs::{ev::Axis, Gilrs};
use prelude::*;
use std::sync::{Arc, LazyLock, Mutex};
use std::thread;
use std::time::{Duration, Instant};

const FPS: f32 = 10.0;
const IP: &str = "127.0.0.1";
const PORT: u16 = 5005;
static RENDER_FRAME_DURATION: LazyLock<Duration> =
    LazyLock::new(|| Duration::from_secs_f32(1.0 / FPS));

fn handle_input(
    player_1: Arc<Mutex<game::player::Player>>,
    player_2: Arc<Mutex<game::player::Player>>,
    gilrs: &mut Gilrs,
    gamepad_id: gilrs::GamepadId,
) {
    let mut last_moved_player1_x = Instant::now();
    let mut last_moved_player1_y = Instant::now();
    let mut last_moved_player2_x = Instant::now();
    let mut last_moved_player2_y = Instant::now();

    loop {
        gilrs.next_event();
        let gp = gilrs.gamepad(gamepad_id);

        last_moved_player1_x =
            handle_player_axis(&gp, Axis::LeftStickX, last_moved_player1_x, &player_1)
                .unwrap_or(last_moved_player1_x);
        last_moved_player1_y =
            handle_player_axis(&gp, Axis::LeftStickY, last_moved_player1_y, &player_1)
                .unwrap_or(last_moved_player1_y);

        last_moved_player2_x =
            handle_player_axis(&gp, Axis::RightStickX, last_moved_player2_x, &player_2)
                .unwrap_or(last_moved_player2_x);
        last_moved_player2_y =
            handle_player_axis(&gp, Axis::RightStickY, last_moved_player2_y, &player_2)
                .unwrap_or(last_moved_player2_y);
    }
}

fn render_loop(
    voxelbox: Arc<Mutex<voxelbox::Voxelbox>>,
    player_1: Arc<Mutex<game::player::Player>>,
    player_2: Arc<Mutex<game::player::Player>>,
) {
    loop {
        thread::sleep(*RENDER_FRAME_DURATION);

        let mut vbox = voxelbox.lock().unwrap();
        let p1 = player_1.lock().unwrap();
        let p2 = player_2.lock().unwrap();

        vbox.reset_leds();
        p1.draw_pad(&mut vbox).log("Unable to draw player 1");
        p2.draw_pad(&mut vbox).log("Unable to draw player 2");
        vbox.send().log("Could not send data");
    }
}

fn main() {
    let mut gilrs = Gilrs::new().expect("Failed to initialize gilrs, needed to get controllers");
    let gp_id = gilrs.gamepads().next().expect("Please connect a gamepad").0;

    let voxelbox = Arc::new(Mutex::new(voxelbox::Voxelbox::new(IP, PORT)));
    let player_1 = Arc::new(Mutex::new(game::player::Player::player_1()));
    let player_2 = Arc::new(Mutex::new(game::player::Player::player_2()));

    let voxelbox_clone = Arc::clone(&voxelbox);
    let player_1_clone = Arc::clone(&player_1);
    let player_2_clone = Arc::clone(&player_2);

    let input_thread =
        thread::spawn(move || handle_input(player_1_clone, player_2_clone, &mut gilrs, gp_id));
    let render_thread = thread::spawn(move || render_loop(voxelbox_clone, player_1, player_2));

    input_thread
        .join()
        .expect("The input handling thread paniced");
    render_thread.join().expect("The rendering thread paniced");
}
