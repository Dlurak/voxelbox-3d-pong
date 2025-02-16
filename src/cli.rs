use std::num::NonZero;

use clap::Parser;

use crate::positive::Positive;

#[derive(Parser)]
#[command(version, about = "3d Pong on the Voxelbox", long_about = None)]
pub struct Args {
    /// Sensitivity of Player 1 (Green), controls paddle speed
    #[arg(
        long,
        visible_alias = "sens-p1",
        default_value_t = Positive::new(1.5).unwrap(),
        value_parser = sensitivity_parser
    )]
    pub sensitivity_p1: Positive<f32>,
    #[arg(
        long,
        visible_alias = "sens-p2",
        default_value_t = Positive::new(1.5).unwrap(),
        value_parser = sensitivity_parser
    )]
    /// Sensitivity of Player 2 (Yellow), controls paddle speed
    pub sensitivity_p2: Positive<f32>,
    /// The number of points needed to win
    #[arg(
        long,
        default_value_t = NonZero::new(5).unwrap()
    )]
    pub winning_points: NonZero<u8>,
    /// IP-Address of the Voxelbox
    #[arg(
        long,
        default_value_t = String::from("127.0.0.1"),
    )]
    pub ip: String,
    /// Port of the Voxelbox
    #[arg(
        long,
        default_value_t = 5005,
        value_parser = clap::value_parser!(u16).range(1..)
    )]
    pub port: u16,
}


fn sensitivity_parser(s: &str) -> Result<Positive<f32>, String> {
    s.parse()
        .map_err(|_| format!("{s} isn't a number"))
        .and_then(|n| Positive::new(n).ok_or_else(|| format!("{s} is bigger than 0")))
}
