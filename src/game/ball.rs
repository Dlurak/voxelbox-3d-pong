use rand::Rng;
use std::{num::NonZero, time::Duration};

use crate::{color::Rgb, dynamic_vec, game::player::Player, odd::Odd, plus_minus, voxelbox};

use super::pad;

const SIZE: Odd<u8> = Odd::<u8>::new_panics(3);
const PADDING: u8 = (SIZE.value() - 1) / 2;
const COLLISIONS_UNTIL_SPEED_INC: u8 = 1;
const MAX_BALL_SLEEP_TIME: f64 = 800.0;
const MIN_BALL_SLEEP_TIME: f64 = 300.0;

#[derive(PartialEq, Eq, Debug)]
pub enum CollisionSide {
    Left,
    Right,
    Top,
    Bottom,
    Front,
    Back,
}

pub struct Ball {
    position: (u8, u8, u8),
    color: Rgb,
    direction: (NonZero<i8>, i8, i8),
    collisions_since_speed_inc: u8,
    pub movement_intervall: Duration,
}

impl Ball {
    pub fn apply_movement(&mut self) {
        let x = (((self.position.0 as i8) + self.direction.0.get()) as u8)
            .clamp(PADDING, voxelbox::WIDTH - 1 - PADDING);
        let y = (((self.position.1 as i8) + self.direction.1) as u8)
            .clamp(PADDING, voxelbox::HEIGHT - 1 - PADDING);
        let z = (((self.position.2 as i8) + self.direction.2) as u8)
            .clamp(PADDING, voxelbox::DEEPTH - 1 - PADDING);
        self.position = (x, y, z);
    }

    fn colliding_voxelbox_sides(&self) -> Vec<CollisionSide> {
        let (x, y, z) = self.position;

        dynamic_vec! {
            x == PADDING => CollisionSide::Left,
            x == voxelbox::WIDTH - 1 - PADDING => CollisionSide::Right,
            y == PADDING => CollisionSide::Top,
            y == voxelbox::HEIGHT - 1 - PADDING => CollisionSide::Bottom,
            z == PADDING => CollisionSide::Front,
            z == voxelbox::DEEPTH - 1 - PADDING => CollisionSide::Back,
        }
    }

    fn collides_with_player(&self, player: &Player) -> bool {
        let (x, y, z) = self.position;
        let ((player_x1, player_x2), player_y, player_z) = player.full_position();
        let x1 = x + PADDING;
        let x2 = x - PADDING;

        let values = plus_minus!(player_x1 as i8, player_x2 as i8);
        let x_is_matching = values.contains(&(x1 as i8)) || values.contains(&(x2 as i8));
        if !x_is_matching {
            return false;
        }

        pad::DRAWING_DELTAS.iter().any(|(y_offset, z_offset)| {
            let pad_y = player_y as i8 + y_offset;
            let pad_z = player_z as i8 + z_offset;

            plus_minus!(pad_y).contains(&(y as i8)) && plus_minus!(pad_z).contains(&(z as i8))
        })
    }

    pub fn colliding_sides(&mut self, player_1: &Player, player_2: &Player) -> Vec<CollisionSide> {
        let sides = dynamic_vec! {self.colliding_voxelbox_sides(),
            self.collides_with_player(player_1) => CollisionSide::Left,
            self.collides_with_player(player_2) => CollisionSide::Right,
        };

        if !sides.is_empty() {
            self.collisions_since_speed_inc += 1;
            if self.collisions_since_speed_inc >= COLLISIONS_UNTIL_SPEED_INC {
                self.collisions_since_speed_inc = 0;
                let current_ms = self.movement_intervall.as_millis() as f64;
                let decrease = (current_ms - MIN_BALL_SLEEP_TIME) / 3.0;
                let new_ms = (current_ms - decrease) as u64;
                self.movement_intervall = Duration::from_millis(new_ms);
            }
        }

        sides
    }

    pub fn change_direction(&mut self, colliding_sides: &[CollisionSide]) {
        let mut direction = self.direction;
        let mut rng = rand::rng();
        let x_collides = colliding_sides.contains(&CollisionSide::Left)
            || colliding_sides.contains(&CollisionSide::Right);
        let y_collides = colliding_sides.contains(&CollisionSide::Top)
            || colliding_sides.contains(&CollisionSide::Bottom);
        let z_collides = colliding_sides.contains(&CollisionSide::Front)
            || colliding_sides.contains(&CollisionSide::Back);

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
            if rng.random_bool(0.4) && direction.1 == 0 {
                direction.1 += if rng.random_bool(0.5) { 1 } else { -1 };
            }
            if rng.random_bool(0.4) && direction.2 == 0 {
                direction.2 += if rng.random_bool(0.5) { 1 } else { -1 };
            }
        }

        self.direction = direction;
    }

    pub fn draw(&self, voxelbox: &mut voxelbox::Voxelbox) {
        let (x, y, z) = self.position;

        voxelbox.set_led(x, y, z, self.color);

        for dx in -(PADDING as i8)..=(PADDING as i8) {
            for dy in -(PADDING as i8)..=(PADDING as i8) {
                for dz in -(PADDING as i8)..=(PADDING as i8) {
                    let x = ((x as i8) + dx) as usize;
                    let y = ((y as i8) + dy) as usize;
                    let z = ((z as i8) + dz) as usize;
                    voxelbox.set_led(x, y, z, self.color);
                }
            }
        }
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
            color: Rgb::red(),
            //direction: (NonZero::new(1).unwrap(), 2, 0),
            direction: (NonZero::new(1).unwrap(), 0, 0),
            collisions_since_speed_inc: 0,
            movement_intervall: Duration::from_millis(MAX_BALL_SLEEP_TIME.round() as u64),
        }
    }
}
