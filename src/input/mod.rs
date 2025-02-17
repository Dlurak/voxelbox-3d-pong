mod joystick;

pub use joystick::*;

use std::time::Duration;

#[derive(Debug)]
pub struct TwoDimensional<T> {
    pub x: T,
    pub y: T,
}

pub type ActivationTimes = TwoDimensional<Option<Duration>>;
pub type Normalized = TwoDimensional<Option<f32>>;
pub type Movement = TwoDimensional<i16>;

pub trait GameInput {
    const MIN_TIME: Duration;
    const MAX_TIME: Duration = Self::MIN_TIME;
    fn normalized(&self) -> Normalized;
    fn activation_times(&self) -> ActivationTimes {
        let ms_range = (Self::MAX_TIME - Self::MIN_TIME).as_millis() as f32;

        let compute_time = |normalized: Option<f32>| {
            normalized.map(|n| {
                let extra_ms = (ms_range * (1.0 - n)).round() as u64;
                Self::MIN_TIME + Duration::from_millis(extra_ms)
            })
        };

        let normalized = self.normalized();
        ActivationTimes {
            x: compute_time(normalized.x),
            y: compute_time(normalized.x),
        }
    }
    fn movement(&self) -> Movement {
        let normalized = self.normalized();

        let x = -normalized.x.unwrap_or(0.0).signum() as i16;
        let y = -normalized.x.unwrap_or(0.0).signum() as i16;

        Movement { x, y }
    }
}
