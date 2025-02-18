use std::time::Duration;

use crate::positive::Positive;
use gilrs::{Axis, Event, EventType, GamepadId};

use super::{ActivationTimes, GameInput, Movement};

#[derive(Debug)]
pub struct JoyStick {
    gamepad_id: GamepadId,
    deadzone: f32,
    is_left_stick: bool,
    invert_x: bool,
    sensitivity: Positive<f32>,
    latest_x: Option<f32>,
    latest_y: Option<f32>,
}

impl JoyStick {
    const DEFAULT_DEADZONE: f32 = 0.15;

    const fn new(
        gamepad_id: GamepadId,
        sensitivity: Positive<f32>,
        is_left_stick: bool,
        invert_x: bool,
    ) -> Self {
        Self {
            gamepad_id,
            deadzone: Self::DEFAULT_DEADZONE,
            is_left_stick,
            invert_x,
            sensitivity,
            latest_x: None,
            latest_y: None,
        }
    }

    pub const fn new_player_1(gamepad_id: GamepadId, sensitivity: Positive<f32>) -> Self {
        Self::new(gamepad_id, sensitivity, true, false)
    }

    pub const fn new_player_2(
        gamepad_id: GamepadId,
        sensitivity: Positive<f32>,
        own_controller: bool,
    ) -> Self {
        Self::new(gamepad_id, sensitivity, own_controller, true)
    }

    pub fn add_event(&mut self, event: &Event) -> Option<(Axis, f32)> {
        if event.id != self.gamepad_id {
            return None;
        }

        if let EventType::AxisChanged(axis, strength, _) = event.event {
            match (self.is_left_stick, axis) {
                (true, Axis::LeftStickX) => self.latest_x = Some(strength),
                (true, Axis::LeftStickY) => self.latest_y = Some(strength),
                (false, Axis::RightStickX) => self.latest_x = Some(strength),
                (false, Axis::RightStickY) => self.latest_y = Some(strength),
                _ => return None,
            }
            Some((axis, strength))
        } else {
            None
        }
    }

    const fn x_value(&self) -> Option<f32> {
        self.latest_x
    }

    const fn y_value(&self) -> Option<f32> {
        self.latest_y
    }
}

impl GameInput for JoyStick {
    const MIN_TIME: Duration = Duration::from_millis(110);
    const MAX_TIME: Duration = Duration::from_millis(350);

    fn activation_times(&self) -> ActivationTimes {
        let sensitivity = self.sensitivity.value();
        let ms_range = (Self::MAX_TIME - Self::MIN_TIME).as_millis() as f32;

        let compute_time = |normalized: Option<f32>| {
            normalized.map(|n| {
                let extra_ms = ms_range * (1.0 - n.abs().powf(sensitivity));
                let extra_ms = extra_ms.round() as u64;
                Self::MIN_TIME + Duration::from_millis(extra_ms)
            })
        };

        let normalized = self.normalized();
        ActivationTimes {
            x: compute_time(normalized.x),
            y: compute_time(normalized.y),
        }
    }

    fn normalized(&self) -> super::Normalized {
        let deadzone = self.deadzone;

        let normalize = |value: Option<f32>| {
            value.filter(|&v| v.abs() > deadzone).map(|v| {
                let n = (v.abs() - deadzone) / (1.0 - deadzone);
                if v < 0.0 {
                    -n
                } else {
                    n
                }
            })
        };

        super::Normalized {
            x: normalize(self.x_value()),
            y: normalize(self.y_value()),
        }
    }

    fn movement(&self) -> Movement {
        let normalized = self.normalized();

        let x = -normalized.x.unwrap_or(0.0).signum() as i16;
        let y = -normalized.y.unwrap_or(0.0).signum() as i16;

        if self.invert_x {
            Movement { x: -x, y }
        } else {
            Movement { x, y }
        }
    }
}
