use std::sync::LazyLock;

use crate::color::Rgb;
use crate::odd::Odd;
use crate::voxelbox::{self, Draw};

pub const PAD_SIZE: Odd<u8> = Odd::<u8>::new_panics(5);

pub static DRAWING_DELTAS: LazyLock<Vec<(i8, i8)>> = LazyLock::new(|| {
    let size = PAD_SIZE.value() as i8;
    let padding = (size - 1) / 2;

    (-padding..=padding)
        .flat_map(|y| {
            let width = size - (y.abs() * 2);
            let half = width / 2;
            (-half..=half).map(move |x| (x, y))
        })
        .collect()
});

enum PlayerSite {
    Left,
    Right,
}

impl PlayerSite {
    const fn get_x(&self) -> (u8, u8) {
        match self {
            Self::Left => (0, 1),
            Self::Right => (voxelbox::WIDTH - 1, voxelbox::WIDTH - 2),
        }
    }
}

pub struct Player {
    color: Rgb,
    position: Position,
    site: PlayerSite,
}

struct Position {
    x: u8,
    y: u8,
}

impl Default for Position {
    fn default() -> Self {
        Self {
            x: voxelbox::DEEPTH / 2,
            y: voxelbox::HEIGHT / 2,
        }
    }
}

impl Player {
    pub fn player_1() -> Self {
        Self {
            color: Rgb::green(),
            position: Position::default(),
            site: PlayerSite::Left,
        }
    }
    pub fn player_2() -> Self {
        Self {
            color: Rgb::yellow(),
            position: Position::default(),
            site: PlayerSite::Right,
        }
    }

    pub const fn full_position(&self) -> ((u8, u8), u8, u8) {
        (self.site.get_x(), self.position.y, self.position.x)
    }

    pub fn inc_x(&mut self, x: i16) {
        let padding = (PAD_SIZE - 1) / 2;
        let lower_limit: i16 = padding.into();
        let upper_limit: i16 = (voxelbox::DEEPTH - 1 - padding).into();

        self.position.x = (self.position.x as i16 + x).clamp(lower_limit, upper_limit) as u8;
    }

    pub fn inc_y(&mut self, y: i16) {
        let padding = (PAD_SIZE - 1) / 2;
        let lower_limit: i16 = padding.into();
        let upper_limit: i16 = (voxelbox::HEIGHT - 1 - padding).into();

        self.position.y = (self.position.y as i16 + y).clamp(lower_limit, upper_limit) as u8;
    }
}

impl Draw for Player {
    fn color(&self) -> Rgb {
        self.color
    }

    fn draw(&self) -> Vec<(usize, usize, usize)> {
        let (x1, x2) = self.site.get_x();
        let x1 = x1 as usize;
        let x2 = x2 as usize;

        let y = self.position.y;
        let z = self.position.x;
        (*DRAWING_DELTAS)
            .iter()
            .flat_map(|(delta_y, delta_z)| {
                let y = (delta_y + y as i8) as usize;
                let z = (delta_z + z as i8) as usize;
                [(x1, y, z), (x2, y, z)]
            })
            .collect()
    }
}
