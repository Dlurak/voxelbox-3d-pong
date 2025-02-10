mod color;
mod game;
mod odd;
mod prelude;
mod voxelbox;

use gilrs::{ev::Axis, Gilrs};
use prelude::*;
use std::sync::{Arc, LazyLock, Mutex};
use std::thread;
use std::time::Duration;

const IGNORE_THRESHOLD: f32 = 0.2;
const FPS: f32 = 10.0;
static FRAME_DURATION: LazyLock<Duration> = LazyLock::new(|| Duration::from_secs_f32(1.0 / FPS));
const IP: &str = "127.0.0.1";
const PORT: u16 = 5005;

fn delta(controller_axis_value: f32) -> f32 {
    if controller_axis_value.abs() <= IGNORE_THRESHOLD {
        0.0
    } else if controller_axis_value > 0.0 {
        -1.0
    } else {
        1.0
    }
}

fn main() {
    let mut gilrs = Gilrs::new().expect("Failed to initialize gilrs, needed to get controllers");
    let gp_id = gilrs
        .gamepads()
        .next()
        .map(|(id, _)| id)
        .expect("Please connect a gamepad");

    let voxelbox = Arc::new(Mutex::new(voxelbox::Voxelbox::new(IP, PORT)));
    let player_1 = Arc::new(Mutex::new(game::player::Player::player_1()));
    let player_2 = Arc::new(Mutex::new(game::player::Player::player_2()));

    let voxelbox_clone = Arc::clone(&voxelbox);
    let player_1_clone = Arc::clone(&player_1);
    let player_2_clone = Arc::clone(&player_2);

    let input_thread = thread::spawn(move || loop {
        gilrs.next_event();
        let gp = gilrs.gamepad(gp_id);

        let mut p1 = player_1_clone.lock().unwrap();
        let mut p2 = player_2_clone.lock().unwrap();

        p1.inc_x(delta(gp.value(Axis::LeftStickX)).round() as i16);
        p1.inc_y(delta(gp.value(Axis::LeftStickY)).round() as i16);
        p2.inc_x(-(delta(gp.value(Axis::RightStickX)).round() as i16));
        p2.inc_y(delta(gp.value(Axis::RightStickY)).round() as i16);
    });

    let render_thread = thread::spawn(move || loop {
        thread::sleep(*FRAME_DURATION);

        let mut vbox = voxelbox_clone.lock().unwrap();
        let p1 = player_1.lock().unwrap();
        let p2 = player_2.lock().unwrap();

        vbox.reset_leds();
        p1.draw_pad(&mut vbox).log("Unable to draw player 1");
        p2.draw_pad(&mut vbox).log("Unable to draw player 2");
        vbox.send().log("Could not send data");
    });

    input_thread
        .join()
        .expect("The input handling thread paniced");
    render_thread.join().expect("The rendering thread paniced");
}
