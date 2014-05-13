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

use libc::c_int;

// SDL_power.h

#[repr(C)]
pub enum SDL_PowerState {
    SDL_POWERSTATE_UNKNOWN,      // cannot determine power status
    SDL_POWERSTATE_ON_BATTERY,   // Not plugged in, running on the battery
    SDL_POWERSTATE_NO_BATTERY,   // Plugged in, no battery available
    SDL_POWERSTATE_CHARGING,     // Plugged in, charging battery
    SDL_POWERSTATE_CHARGED,      // Plugged in, battery charged
}

extern "C" {
    pub fn SDL_GetPowerInfo(secs: *mut c_int, pct: *mut c_int) -> SDL_PowerState;
}
