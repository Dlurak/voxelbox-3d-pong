use rand::Rng;
use std::{num::NonZero, time::Duration};

use crate::{
    color::Rgb,
    odd::Odd,
    voxelbox::{self, Draw},
};

const COLLISIONS_UNTIL_SPEED_INC: u8 = 2;
const MAX_BALL_SLEEP_TIME: f64 = 600.0;
const MIN_BALL_SLEEP_TIME: f64 = 300.0;

pub struct Ball {
    pub position: (u8, u8, u8),
    color: Rgb,
    direction: (NonZero<i8>, i8, i8),
    collisions_since_speed_inc: u8,
    pub movement_intervall: Duration,
}

impl Ball {
    const SIZE: Odd<u8> = Odd::<u8>::new_panics(3);
    pub const PADDING: u8 = (Self::SIZE.value() - 1) / 2;

    pub fn new_with_x(x: NonZero<i8>) -> Self {
        Self {
            direction: (x, 0, 0),
            ..Self::default()
        }
    }

    pub fn apply_movement(&mut self) {
        let x = (((self.position.0 as i8) + self.direction.0.get()) as u8)
            .clamp(Self::PADDING, voxelbox::WIDTH - 1 - Self::PADDING);
        let y = (((self.position.1 as i8) + self.direction.1) as u8)
            .clamp(Self::PADDING, voxelbox::HEIGHT - 1 - Self::PADDING);
        let z = (((self.position.2 as i8) + self.direction.2) as u8)
            .clamp(Self::PADDING, voxelbox::DEEPTH - 1 - Self::PADDING);
        self.position = (x, y, z);
    }

    pub fn handle_collision(&mut self) {
        self.collisions_since_speed_inc += 1;
        if self.collisions_since_speed_inc >= COLLISIONS_UNTIL_SPEED_INC {
            self.collisions_since_speed_inc = 0;
            let current_ms = self.movement_intervall.as_millis() as f64;
            let decrease = (current_ms - MIN_BALL_SLEEP_TIME) / 3.0;
            let new_ms = (current_ms - decrease) as u64;
            self.movement_intervall = Duration::from_millis(new_ms);
        }
    }

    pub fn change_direction(&mut self, (x_collides, y_collides, z_collides): (bool, bool, bool)) {
        let mut direction = self.direction;
        let mut rng = rand::rng();

        if x_collides {
            direction.0 = -(direction.0);
        }

        if y_collides {
            direction.1 = -(direction.1);
        }

        if z_collides {
            direction.2 = -(direction.2);
        }

        if x_collides || z_collides || y_collides {
            let both_are_straight = direction.1 == 0 && direction.2 == 0;
            let probability = if both_are_straight { 0.7 } else { 0.4 };
            if rng.random_bool(probability) && direction.1 == 0 {
                direction.1 += if rng.random_bool(0.5) { 1 } else { -1 };
            }
            if rng.random_bool(probability) && direction.2 == 0 {
                direction.2 += if rng.random_bool(0.5) { 1 } else { -1 };
            }
        }

        self.direction = direction;
    }
}

impl Draw for Ball {
    fn color(&self) -> Rgb {
        self.color
    }

    fn draw(&self) -> Vec<(usize, usize, usize)> {
        let (x, y, z) = self.position;

        let mut result = Vec::with_capacity(Self::PADDING.pow(3).into());
        result.push((x.into(), y.into(), z.into()));

        for dx in -(Self::PADDING as i8)..=(Self::PADDING as i8) {
            for dy in -(Self::PADDING as i8)..=(Self::PADDING as i8) {
                for dz in -(Self::PADDING as i8)..=(Self::PADDING as i8) {
                    let x = ((x as i8) + dx) as usize;
                    let y = ((y as i8) + dy) as usize;
                    let z = ((z as i8) + dz) as usize;
                    result.push((x, y, z));
                }
            }
        }

        result
    }
}

impl Default for Ball {
    fn default() -> Self {
        Self {
            position: (
                voxelbox::WIDTH / 2,
                voxelbox::HEIGHT / 2,
                voxelbox::DEEPTH / 2,
            ),
            color: Rgb::pink(),
            direction: (NonZero::new(1).unwrap(), 0, 0),
            collisions_since_speed_inc: 0,
            movement_intervall: Duration::from_millis(MAX_BALL_SLEEP_TIME.round() as u64),
        }
    }
}
