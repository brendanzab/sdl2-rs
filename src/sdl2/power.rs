// Copyright 2014 The sdl2-rs Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use ffi;

/// An enum describing the power state.
///
/// # Example
///
/// ~~~
/// match sdl2::power::info() {
///     (sdl2::power::Unknown,   _, _) => { /* ... */ },
///     (sdl2::power::OnBattery, _, _) => { /* ... */ },
///     (sdl2::power::NoBattery, _, _) => { /* ... */ },
///     (sdl2::power::Charging,  _, _) => { /* ... */ },
///     (sdl2::power::Charged,   _, _) => { /* ... */ },
/// }
/// ~~~
pub type State = ffi::power::SDL_PowerState;

/// Cannot determine the power status.
pub static Unknown: State = ffi::power::SDL_POWERSTATE_UNKNOWN;

/// Not plugged in and running on the battery.
pub static OnBattery: State = ffi::power::SDL_POWERSTATE_ON_BATTERY;

/// Plugged in but no battery is available.
pub static NoBattery: State = ffi::power::SDL_POWERSTATE_NO_BATTERY;

/// Plugged in and charging the battery.
pub static Charging: State = ffi::power::SDL_POWERSTATE_CHARGING;

/// Plugged in and the battery is charged.
pub static Charged: State = ffi::power::SDL_POWERSTATE_CHARGED;

/// Returns the power supply state.
pub fn info() -> (State, Option<uint>, Option<uint>) {
    let (mut secs, mut percent) = (0, 0);
    let state = unsafe {
        ffi::power::SDL_GetPowerInfo(&mut secs, &mut percent)
    };

    let secs    = if secs    < 0 { None } else { Some(secs as uint)    };
    let percent = if percent < 0 { None } else { Some(percent as uint) };

    (state, secs, percent)
}
