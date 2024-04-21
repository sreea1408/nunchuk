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

pub struct Calibrator {
    pub pos: f32,
    pub neg: f32,
}

impl Calibrator {
    pub fn calibrate_value(&self, value: f32) -> f32 {
        let calibrated_value = if value > 0.0 {
            value / self.pos
        } else if value < 0.0 {
            value / self.neg
        } else {
            0.0
        };
        if calibrated_value > 1.0 {
            1.0
        } else if calibrated_value < -1.0 {
            -1.0
        } else {
            calibrated_value
        }
    }
}
