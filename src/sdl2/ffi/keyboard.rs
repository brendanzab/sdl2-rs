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

use ffi::keycode::{SDL_Keycode, SDL_Keymod};
use ffi::rect::SDL_Rect;
use ffi::scancode::SDL_Scancode;
use ffi::video::SDL_Window;
use ffi::stdinc::{SDL_bool, Uint8, Uint16, Uint32};
use libc::{c_char, c_int};

// SDL_keyboard.h

pub struct SDL_Keysym {
    pub scancode:   SDL_Scancode,
    pub sym:        SDL_Keycode,
    pub key_mod:    Uint16,
    unused:         Uint32,
}

extern "C" {
    pub fn SDL_GetKeyboardFocus() -> *SDL_Window;
    pub fn SDL_GetKeyboardState(numkeys: *mut c_int) -> *Uint8;
    pub fn SDL_GetModState() -> SDL_Keymod;
    pub fn SDL_SetModState(modstate: SDL_Keymod);
    pub fn SDL_GetKeyFromScancode(scancode: SDL_Scancode) -> SDL_Keycode;
    pub fn SDL_GetScancodeFromKey(key: SDL_Keycode) -> SDL_Scancode;
    pub fn SDL_GetScancodeName(scancode: SDL_Scancode) -> *c_char;
    pub fn SDL_GetScancodeFromName(name: *c_char) -> SDL_Scancode;
    pub fn SDL_GetKeyName(key: SDL_Keycode ) -> *c_char;
    pub fn SDL_GetKeyFromName(name: *c_char) -> SDL_Keycode;
    pub fn SDL_StartTextInput();
    pub fn SDL_IsTextInputActive() -> SDL_bool;
    pub fn SDL_StopTextInput();
    pub fn SDL_SetTextInputRect(rect: *SDL_Rect);
    pub fn SDL_HasScreenKeyboardSupport() -> SDL_bool;
    pub fn SDL_IsScreenKeyboardShown(window: *SDL_Window) -> SDL_bool;
}
