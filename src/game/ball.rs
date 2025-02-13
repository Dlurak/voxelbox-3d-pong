use crate::{color::Rgb, dynamic_vec, game::player::Player, odd::Odd, plus_minus, voxelbox};

use super::pad;

const SIZE: Odd<u8> = Odd::<u8>::new_panics(3);
const PADDING: u8 = (SIZE.value() - 1) / 2;

#[derive(PartialEq, Eq, Debug)]
pub enum VoxelboxSide {
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
    direction: (i8, i8, i8),
}

impl Ball {
    pub fn apply_movement(&mut self) {
        let x = (((self.position.0 as i8) + self.direction.0) as u8)
            .clamp(PADDING, voxelbox::WIDTH - 1 - PADDING);
        let y = (((self.position.1 as i8) + self.direction.1) as u8)
            .clamp(PADDING, voxelbox::HEIGHT - 1 - PADDING);
        let z = (((self.position.2 as i8) + self.direction.2) as u8)
            .clamp(PADDING, voxelbox::DEEPTH - 1 - PADDING);
        self.position = (x, y, z);
    }

    fn colliding_voxelbox_sides(&self) -> Vec<VoxelboxSide> {
        let (x, y, z) = self.position;

        dynamic_vec! {
            x == PADDING => VoxelboxSide::Left,
            x == voxelbox::WIDTH - 1 - PADDING => VoxelboxSide::Right,
            y == PADDING => VoxelboxSide::Top,
            y == voxelbox::HEIGHT - 1 - PADDING => VoxelboxSide::Bottom,
            z == PADDING => VoxelboxSide::Front,
            z == voxelbox::DEEPTH - 1 - PADDING => VoxelboxSide::Back,
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

    pub fn colliding_sides(&self, player_1: &Player, player_2: &Player) -> Vec<VoxelboxSide> {
        dynamic_vec! {self.colliding_voxelbox_sides(),
            self.collides_with_player(player_1) => VoxelboxSide::Left,
            self.collides_with_player(player_2) => VoxelboxSide::Right,
        }
    }

    pub fn change_direction(&mut self, colliding_sides: &[VoxelboxSide]) {
        if colliding_sides.contains(&VoxelboxSide::Left)
            || colliding_sides.contains(&VoxelboxSide::Right)
        {
            self.direction.0 = -(self.direction.0)
        }

        if colliding_sides.contains(&VoxelboxSide::Top)
            || colliding_sides.contains(&VoxelboxSide::Bottom)
        {
            self.direction.1 = -(self.direction.1)
        }

        if colliding_sides.contains(&VoxelboxSide::Front)
            || colliding_sides.contains(&VoxelboxSide::Back)
        {
            self.direction.2 = -(self.direction.2)
        }
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
            //direction: (0, 2, 1),
            direction: (1, 2, 0),
        }
    }
}
