use crate::log::Severity;
use crate::prelude::*;
use crate::voxelbox;
use std::{
    sync::{Arc, LazyLock, Mutex},
    thread,
    time::Duration,
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

pub fn render_loop(
    voxelbox: &Arc<Mutex<voxelbox::Voxelbox>>,
    player_1: &Arc<Mutex<player::Player>>,
    player_2: &Arc<Mutex<player::Player>>,
    ball: &Arc<Mutex<ball::Ball>>,
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
