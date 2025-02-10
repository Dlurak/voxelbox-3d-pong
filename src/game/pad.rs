use crate::color::Rgb;
use crate::odd::Odd;
use crate::voxelbox;

pub const DRAWING_DELTAS: [(i8, i8); 13] = [
    (0, 0),
    (-1, -1),
    (-1, 1),
    (1, 1),
    (1, -1),
    (0, -2),
    (0, -1),
    (0, 1),
    (0, 2),
    (-2, 0),
    (-1, 0),
    (1, 0),
    (2, 0),
];

// TODO: Rendered size only is applied using Deltas not using width/height
pub const WIDTH: Odd<u8> = Odd::<u8>::new_panics(5);
pub const HEIGHT: Odd<u8> = Odd::<u8>::new_panics(5);

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

    let width_offset = (WIDTH - 1) / 2;
    let z_max = voxelbox::DEEPTH - width_offset;
    let in_z_bounds = (width_offset..z_max).contains(&z);
    if !in_z_bounds {
        return Err(OutOfBounds::Y);
    }

    let height_offset = (HEIGHT) / 2;
    let y_max = voxelbox::HEIGHT - height_offset;
    let in_y_bounds = (width_offset..y_max).contains(&y);
    if !in_y_bounds {
        return Err(OutOfBounds::Z);
    }

    for (delta_y, delta_z) in DRAWING_DELTAS {
        voxelbox.set_led(
            x,
            u8::try_from(delta_y + y as i8).unwrap(),
            u8::try_from(delta_z + z as i8).unwrap(),
            color,
        );
    }

    Ok(())
}
