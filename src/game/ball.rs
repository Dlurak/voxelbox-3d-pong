// TODO: This file includes many disabled warnings

use crate::{color::Rgb, game::player::Player, odd::Odd, voxelbox};

const SIZE: Odd<u8> = Odd::<u8>::new_panics(3);
const PADDING: u8 = (SIZE.value() - 1) / 2;

#[allow(dead_code)]
pub struct Ball {
    position: (u8, u8, u8),
    color: Rgb,
    direction: (i8, i8, i8),
}

#[allow(dead_code)]
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

    fn collides_with_voxelbox(&self) -> bool {
        let (x, y, z) = self.position;

        (x == PADDING)
            || (x == voxelbox::WIDTH - 1 - PADDING)
            || (y == PADDING)
            || (y == voxelbox::HEIGHT - 1 - PADDING)
            || (z == PADDING)
            || (z == voxelbox::DEEPTH - 1 - PADDING)
    }

    fn collides_with_player(&self, _player: &Player) -> bool {
        //let (x, y, z) = self.position;
        //let ((player_x1, player_x2), player_y, player_z) = player.full_position();

        todo!()
        //pad::DRAWING_DELTAS.iter().any(|(y_offset, z_offset)| {
        // TODO: Implement this correctly
        //false
        //})
    }

    pub fn collides(&self, player_1: &Player, player_2: &Player) -> bool {
        self.collides_with_voxelbox()
            || self.collides_with_player(player_1)
            || self.collides_with_player(player_2)
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
            color: Rgb::purple(),
            //direction: (1, 1, 1),
            direction: (1, 0, 0),
        }
    }
}
