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

use ffi::stdinc::{Uint32, Sint64};
use libc::{c_float, c_int};

// SDL_touch.h

pub type SDL_TouchID = Sint64;
pub type SDL_FingerID = Sint64;

pub struct SDL_Finger {
    pub id: SDL_FingerID,
    pub x: c_float,
    pub y: c_float,
    pub pressure: c_float,
}

pub static SDL_TOUCH_MOUSEID: Uint32 = -1;

extern "C" {
    pub fn SDL_GetNumTouchDevices() -> c_int;
    pub fn SDL_GetTouchDevice(index: c_int) -> SDL_TouchID;
    pub fn SDL_GetNumTouchFingers(touchID: SDL_TouchID) -> c_int;
    pub fn SDL_GetTouchFinger(touchID: SDL_TouchID, index: c_int) -> *SDL_Finger;
}
