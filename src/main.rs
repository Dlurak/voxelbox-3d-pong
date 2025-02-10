mod color;
mod game;
mod odd;
mod voxelbox;

use gilrs::ev::Axis;
use gilrs::Gilrs;
use std::sync::LazyLock;
use std::time::Duration;

const IGNORE_THRESHOLD: f32 = 0.2;
const FPS: f32 = 10.0;
static FRAME_DURATION: LazyLock<Duration> = LazyLock::new(|| Duration::from_secs_f32(1.0 / FPS));
const IP: &str = "127.0.0.1";
const PORT: u16 = 5005;

fn delta(value: f32) -> f32 {
    if value.abs() <= IGNORE_THRESHOLD {
        0.0
    } else if value > 0.0 {
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

    let mut voxelbox = voxelbox::Voxelbox::new(IP, PORT);

    let mut player_1 = game::player::Player::player_1();
    let mut player_2 = game::player::Player::player_2();

    loop {
        gilrs.next_event();
        let gp = gilrs.gamepad(gp_id);

        player_1.inc_x(delta(gp.value(Axis::LeftStickX)).round() as i16);
        player_1.inc_y(delta(gp.value(Axis::LeftStickY)).round() as i16);
        player_2.inc_x(-(delta(gp.value(Axis::RightStickX)).round() as i16));
        player_2.inc_y(delta(gp.value(Axis::RightStickY)).round() as i16);

        voxelbox.reset_leds();
        let _ = player_1.draw_pad(&mut voxelbox);
        let _ = player_2.draw_pad(&mut voxelbox);

        if voxelbox.send().is_err() {
            eprintln!("Could not send data");
        }

        std::thread::sleep(*FRAME_DURATION);
    }
}
