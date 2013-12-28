// Copyright 2013 The sdl2-rs Developers.
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

#[feature(globs)];
#[feature(macro_rules)];

#[comment = "Bindings and wrapper for SDL2."];
#[crate_id = "github.com/bjz/sdl2-rs#sdl2:0.1"];
#[crate_type = "lib"];

use ffi::stdinc::Uint32;

pub mod ffi;

#[repr(C)]
pub enum InitFlag {
    InitTimer           = ffi::SDL_INIT_TIMER,
    InitAudio           = ffi::SDL_INIT_AUDIO,
    InitVideo           = ffi::SDL_INIT_VIDEO,
    InitJoystick        = ffi::SDL_INIT_JOYSTICK,
    InitHaptic          = ffi::SDL_INIT_HAPTIC,
    InitGamecontroller  = ffi::SDL_INIT_GAMECONTROLLER,
    InitEvents          = ffi::SDL_INIT_EVENTS,
    InitNoparachute     = ffi::SDL_INIT_NOPARACHUTE,
    InitEverything      = ffi::SDL_INIT_EVERYTHING,
}

impl InitFlag {
    pub fn fold_bits(flags: &[InitFlag]) -> Uint32 {
        flags.iter().fold(0, |acc, &flag| acc | flag as Uint32)
    }
}

#[inline]
pub fn init(flags: &[InitFlag]) -> bool {
    unsafe { ffi::SDL_Init(InitFlag::fold_bits(flags)) == 0 }
}

#[inline]
pub fn init_subsystem(flags: &[InitFlag]) -> bool {
    unsafe { ffi::SDL_InitSubSystem(InitFlag::fold_bits(flags)) == 0 }
}

#[inline]
pub fn quit_subsystem(flags: &[InitFlag]) {
    unsafe { ffi::SDL_QuitSubSystem(InitFlag::fold_bits(flags)) }
}

#[inline]
pub fn quit() {
    unsafe { ffi::SDL_Quit() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_flags_fold_bits() {
        assert_eq!(
            InitFlag::fold_bits([
                InitTimer, InitAudio, InitVideo, InitEvents,
                InitJoystick, InitHaptic, InitGamecontroller
            ]),
            ffi::SDL_INIT_EVERYTHING
        )
    }
}
