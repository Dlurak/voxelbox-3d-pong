use std::sync::LazyLock;

use crate::color::Rgb;
use crate::odd::Odd;
use crate::voxelbox;

pub const SIZE: Odd<u8> = Odd::<u8>::new_panics(5);

pub static DRAWING_DELTAS: LazyLock<Vec<(i8, i8)>> = LazyLock::new(|| deltas(SIZE));

fn deltas(size: Odd<u8>) -> Vec<(i8, i8)> {
    let size = size.value() as i8;
    let padding = (size - 1) / 2;

    (-padding..=padding)
        .flat_map(|y| {
            let width = size - (y.abs() * 2);
            let half = width / 2;
            (-half..=half).map(move |x| (x, y))
        })
        .collect()
}

#[derive(Debug)]
pub enum OutOfBounds {
    X,
    Y,
    Z,
}

pub fn draw_pad(
    voxelbox: &mut voxelbox::Voxelbox,
    color: Rgb,
    x: u8,
    y: u8,
    z: u8,
) -> Result<(), OutOfBounds> {
    if !(0..voxelbox::WIDTH).contains(&x) {
        return Err(OutOfBounds::X);
    }

    let width_offset = (SIZE - 1) / 2;
    let z_max = voxelbox::DEEPTH - width_offset;
    let in_z_bounds = (width_offset..z_max).contains(&z);
    if !in_z_bounds {
        return Err(OutOfBounds::Y);
    }

    let height_offset = (SIZE) / 2;
    let y_max = voxelbox::HEIGHT - height_offset;
    let in_y_bounds = (width_offset..y_max).contains(&y);
    if !in_y_bounds {
        return Err(OutOfBounds::Z);
    }

    for (delta_y, delta_z) in (*DRAWING_DELTAS).iter() {
        voxelbox.set_led(
            x,
            u8::try_from(delta_y + y as i8).unwrap(),
            u8::try_from(delta_z + z as i8).unwrap(),
            color,
        );
    }

    Ok(())
}
