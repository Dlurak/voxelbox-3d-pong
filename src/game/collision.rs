use super::{
    ball::Ball,
    player::{Player, DRAWING_DELTAS},
};
use crate::{dynamic_vec, plus_minus, voxelbox};

pub trait Collision<T> {
    type Output;
    fn collides(&self, other: &T) -> Self::Output;
}

#[derive(PartialEq, Eq, Debug)]
pub enum CollisionSide {
    Left,
    Right,
    Top,
    Bottom,
    Front,
    Back,
}

pub struct Bounds;

impl Collision<Bounds> for Ball {
    type Output = Vec<CollisionSide>;
    fn collides(&self, _: &Bounds) -> Self::Output {
        let (x, y, z) = self.position;

        let padding = Self::PADDING;
        dynamic_vec! {
            x == padding=> CollisionSide::Left,
            x == voxelbox::WIDTH - 1 - padding => CollisionSide::Right,
            y == padding => CollisionSide::Top,
            y == voxelbox::HEIGHT - 1 - padding => CollisionSide::Bottom,
            z == padding => CollisionSide::Front,
            z == voxelbox::DEEPTH - 1 - padding => CollisionSide::Back,
        }
    }
}

impl Collision<Player> for Ball {
    type Output = bool;
    fn collides(&self, other: &Player) -> Self::Output {
        let (x, y, z) = self.position;
        let padding = Self::PADDING;
        let ((player_x1, player_x2), player_y, player_z) = other.full_position();
        let x1 = x + padding;
        let x2 = x - padding;

        let values = plus_minus!(player_x1 as i8, player_x2 as i8);
        let x_is_matching = values.contains(&(x1 as i8)) || values.contains(&(x2 as i8));
        if !x_is_matching {
            return false;
        }

        (*DRAWING_DELTAS).iter().any(|(y_offset, z_offset)| {
            let pad_y = player_y as i8 + y_offset;
            let pad_z = player_z as i8 + z_offset;

            plus_minus!(pad_y).contains(&(y as i8)) && plus_minus!(pad_z).contains(&(z as i8))
        })
    }
}
