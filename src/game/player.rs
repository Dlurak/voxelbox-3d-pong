use super::pad;
use crate::color::Rgb;
use crate::voxelbox;

enum PlayerSite {
    Left,
    Right,
}

impl PlayerSite {
    fn get_x(&self) -> (u8, u8) {
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

    pub fn draw_pad(&self, voxelbox: &mut voxelbox::Voxelbox) -> Result<(), pad::OutOfBounds> {
        let (x1, x2) = self.site.get_x();

        pad::draw_pad(voxelbox, self.color, x1, self.position.y, self.position.x)?;
        pad::draw_pad(voxelbox, self.color, x2, self.position.y, self.position.x)?;

        Ok(())
    }

    //pub fn full_position(&self) -> ((u8, u8), u8, u8) {
        //(self.site.get_x(), self.position.y, self.position.x)
    //}

    pub fn inc_x(&mut self, x: i16) {
        let padding = (super::pad::WIDTH - 1) / 2;
        let lower_limit: i16 = padding.into();
        let upper_limit: i16 = (voxelbox::DEEPTH - 1 - padding).into();

        self.position.x = (self.position.x as i16 + x).clamp(lower_limit, upper_limit) as u8;
    }

    pub fn inc_y(&mut self, y: i16) {
        let padding = (super::pad::HEIGHT - 1) / 2;
        let lower_limit: i16 = padding.into();
        let upper_limit: i16 = (voxelbox::HEIGHT - 1 - padding).into();

        self.position.y = (self.position.y as i16 + y).clamp(lower_limit, upper_limit) as u8;
    }
}
