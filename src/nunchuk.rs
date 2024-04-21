/*
   nunchuk library
   Copyright (C) 2024  sreea1408

   This program is free software; you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation; either version 2 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License along
   with this program; if not, write to the Free Software Foundation, Inc.,
   51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
*/

use gilrs::{Axis, Event, EventType, GamepadId, Gilrs};

use crate::calibration::Calibrator;

const BUTTON_C: u32 = 65824;
const BUTTON_Z: u32 = 65825;

pub enum NunchukEvent {
    XY(f32, f32),
    ButtonC,
    ButtonZ,
    None,
}

pub struct Nunchuk {
    id: GamepadId,
    gilrs: Gilrs,
    x_calibration: Calibrator,
    y_calibration: Calibrator,
}

impl Nunchuk {
    pub fn new(id: GamepadId) -> Self {
        let gilrs = Gilrs::new().unwrap();
        Self {
            id,
            gilrs,
            x_calibration: Default::default(),
            y_calibration: Default::default(),
        }
    }

    fn is_calibration_valid(pos: f32, neg: f32) -> bool {
        return pos > 0.0 && pos <= 1.0 && neg > 0.0 && neg <= 1.0;
    }

    pub fn set_x_calibration(&mut self, pos: f32, neg: f32) {
        if Nunchuk::is_calibration_valid(pos, neg) {
            self.x_calibration = Calibrator::new(pos, neg);
        }
    }

    pub fn set_y_calibration(&mut self, pos: f32, neg: f32) {
        if Nunchuk::is_calibration_valid(pos, neg) {
            self.y_calibration = Calibrator::new(pos, neg);
        }
    }

    pub fn next_event(&mut self) -> NunchukEvent {
        if let Some(Event { event, id, .. }) = self.gilrs.next_event() {
            if id == self.id {
                match event {
                    EventType::ButtonPressed(_, code) => match code.into_u32() {
                        BUTTON_C => return NunchukEvent::ButtonC,
                        BUTTON_Z => return NunchukEvent::ButtonZ,
                        _ => return NunchukEvent::None,
                    },
                    EventType::AxisChanged(_, _, _) => {
                        let gamepad = self.gilrs.gamepad(id);
                        let raw_x = gamepad.value(Axis::LeftStickX);
                        let raw_y = gamepad.value(Axis::LeftStickY);
                        return NunchukEvent::XY(
                            self.x_calibration.calibrate_value(raw_x),
                            self.y_calibration.calibrate_value(raw_y),
                        );
                    }
                    _ => return NunchukEvent::None,
                }
            }
        }
        NunchukEvent::None
    }
}
