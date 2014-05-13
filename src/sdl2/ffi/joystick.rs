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

use ffi::stdinc::{Sint16, Sint32, Uint8, SDL_bool};
use libc::{c_char, c_int};

// SDL_joystick.h

pub enum SDL_Joystick {}

pub struct SDL_JoystickGUID {
    pub data: [Uint8, ..16],
}

pub type SDL_JoystickID = Sint32;

bitflags!(flags SDL_Hat: Uint8 {
    static SDL_HAT_CENTERED    = 0x00,
    static SDL_HAT_UP          = 0x01,
    static SDL_HAT_RIGHT       = 0x02,
    static SDL_HAT_DOWN        = 0x04,
    static SDL_HAT_LEFT        = 0x08,
    static SDL_HAT_RIGHTUP     = SDL_HAT_RIGHT.bits | SDL_HAT_UP.bits,
    static SDL_HAT_RIGHTDOWN   = SDL_HAT_RIGHT.bits | SDL_HAT_DOWN.bits,
    static SDL_HAT_LEFTUP      = SDL_HAT_LEFT.bits | SDL_HAT_UP.bits,
    static SDL_HAT_LEFTDOWN    = SDL_HAT_LEFT.bits | SDL_HAT_DOWN.bits
})

extern "C" {
    pub fn SDL_NumJoysticks() -> c_int;
    pub fn SDL_JoystickNameForIndex(device_index: c_int) -> Option<*c_char>;
    pub fn SDL_JoystickOpen(device_index: c_int) -> Option<*SDL_Joystick>;
    pub fn SDL_JoystickName(joystick: *SDL_Joystick) -> Option<*c_char>;
    pub fn SDL_JoystickGetDeviceGUID(device_index: c_int) -> SDL_JoystickGUID;
    pub fn SDL_JoystickGetGUID(joystick: *SDL_Joystick) -> SDL_JoystickGUID;
    pub fn SDL_JoystickGetGUIDString(guid: SDL_JoystickGUID, pszGUID: *mut c_char, cbGUID: c_int);
    pub fn SDL_JoystickGetGUIDFromString(pchGUID: *c_char) -> SDL_JoystickGUID;
    pub fn SDL_JoystickGetAttached(joystick: *SDL_Joystick) -> SDL_bool;
    pub fn SDL_JoystickInstanceID(joystick: *SDL_Joystick) -> SDL_JoystickID;
    pub fn SDL_JoystickNumAxes(joystick: *SDL_Joystick) -> c_int;
    pub fn SDL_JoystickNumBalls(joystick: *SDL_Joystick) -> c_int;
    pub fn SDL_JoystickNumHats(joystick: *SDL_Joystick) -> c_int;
    pub fn SDL_JoystickNumButtons(joystick: *SDL_Joystick) -> c_int;
    pub fn SDL_JoystickEventState(state: c_int) -> c_int;
    pub fn SDL_JoystickGetAxis(joystick: *SDL_Joystick, axis: c_int) -> Sint16;
    pub fn SDL_JoystickGetHat(joystick: *SDL_Joystick, hat: c_int) -> SDL_Hat;
    pub fn SDL_JoystickGetBall(joystick: *SDL_Joystick, ball: c_int, dx: *mut c_int, dy: *mut c_int) -> c_int;
    pub fn SDL_JoystickGetButton(joystick: *SDL_Joystick, button: c_int) -> Uint8;
    pub fn SDL_JoystickClose(joystick: *SDL_Joystick);
}
