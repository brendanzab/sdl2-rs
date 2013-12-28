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

use ffi::stdinc::{Uint8, Uint32};
use ffi::video::SDL_Window;
use std::libc::{c_char, c_int};

// SDL_messagebox.h

pub type SDL_MessageBoxFlags = Uint32;
pub static SDL_MESSAGEBOX_ERROR        : SDL_MessageBoxFlags = 0x00000010;
pub static SDL_MESSAGEBOX_WARNING      : SDL_MessageBoxFlags = 0x00000020;
pub static SDL_MESSAGEBOX_INFORMATION  : SDL_MessageBoxFlags = 0x00000040;

pub type SDL_MessageBoxButtonFlags = Uint32;
pub static SDL_MESSAGEBOX_BUTTON_RETURNKEY_DEFAULT : SDL_MessageBoxButtonFlags = 0x00000001;
pub static SDL_MESSAGEBOX_BUTTON_ESCAPEKEY_DEFAULT : SDL_MessageBoxButtonFlags = 0x00000002;

pub struct SDL_MessageBoxButtonData {
    flags:      SDL_MessageBoxButtonFlags, // Uint32,
    buttonid:   c_int,
    text:       *c_char,
}

pub struct SDL_MessageBoxColor {
    r: Uint8,
    g: Uint8,
    b: Uint8,
}

pub type SDL_MessageBoxColorType = uint;
pub static SDL_MESSAGEBOX_COLOR_BACKGROUND:         SDL_MessageBoxColorType = 0;
pub static SDL_MESSAGEBOX_COLOR_TEXT:               SDL_MessageBoxColorType = 1;
pub static SDL_MESSAGEBOX_COLOR_BUTTON_BORDER:      SDL_MessageBoxColorType = 2;
pub static SDL_MESSAGEBOX_COLOR_BUTTON_BACKGROUND:  SDL_MessageBoxColorType = 3;
pub static SDL_MESSAGEBOX_COLOR_BUTTON_SELECTED:    SDL_MessageBoxColorType = 4;
pub static SDL_MESSAGEBOX_COLOR_MAX:                SDL_MessageBoxColorType = 5;

pub struct SDL_MessageBoxColorScheme {
    colors: [SDL_MessageBoxColor, ..SDL_MESSAGEBOX_COLOR_MAX],
}

pub struct SDL_MessageBoxData {
    flags:          SDL_MessageBoxFlags, // Uint32,
    window:         Option<*SDL_Window>,
    title:          *c_char,
    message:        *c_char,
    numbuttons:     c_int,
    buttons:        *SDL_MessageBoxButtonData,
    colorScheme:    Option<*SDL_MessageBoxColorScheme>,
}

extern "C" {
    pub fn SDL_ShowMessageBox(messageboxdata: *SDL_MessageBoxData, buttonid: *mut c_int) -> c_int;
    pub fn SDL_ShowSimpleMessageBox(flags: SDL_MessageBoxFlags/*Uint32*/, title: *c_char, message: *c_char, window: Option<*SDL_Window>) -> c_int;
}
