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

use ffi::stdinc::Uint8;
use ffi::video::SDL_Window;
use libc::{c_char, c_int};

// SDL_messagebox.h

#[repr(u32)]
pub enum SDL_MessageBoxFlags {
    SDL_MESSAGEBOX_ERROR        = 0x00000010,
    SDL_MESSAGEBOX_WARNING      = 0x00000020,
    SDL_MESSAGEBOX_INFORMATION  = 0x00000040,
}

#[repr(u32)]
pub enum SDL_MessageBoxButtonFlags {
    SDL_MESSAGEBOX_BUTTON_RETURNKEY_DEFAULT = 0x00000001,
    SDL_MESSAGEBOX_BUTTON_ESCAPEKEY_DEFAULT = 0x00000002,
}

pub struct SDL_MessageBoxButtonData {
    pub flags:      SDL_MessageBoxButtonFlags,
    pub buttonid:   c_int,
    pub text:       *c_char,
}

pub struct SDL_MessageBoxColor {
    pub r: Uint8,
    pub g: Uint8,
    pub b: Uint8,
}

pub type SDL_MessageBoxColorType = uint;
pub static SDL_MESSAGEBOX_COLOR_BACKGROUND:         SDL_MessageBoxColorType = 0;
pub static SDL_MESSAGEBOX_COLOR_TEXT:               SDL_MessageBoxColorType = 1;
pub static SDL_MESSAGEBOX_COLOR_BUTTON_BORDER:      SDL_MessageBoxColorType = 2;
pub static SDL_MESSAGEBOX_COLOR_BUTTON_BACKGROUND:  SDL_MessageBoxColorType = 3;
pub static SDL_MESSAGEBOX_COLOR_BUTTON_SELECTED:    SDL_MessageBoxColorType = 4;
pub static SDL_MESSAGEBOX_COLOR_MAX:                SDL_MessageBoxColorType = 5;

pub struct SDL_MessageBoxColorScheme {
    pub colors: [SDL_MessageBoxColor, ..SDL_MESSAGEBOX_COLOR_MAX],
}

pub struct SDL_MessageBoxData {
    pub flags:          SDL_MessageBoxFlags,
    pub window:         Option<*SDL_Window>,
    pub title:          *c_char,
    pub message:        *c_char,
    pub numbuttons:     c_int,
    pub buttons:        *SDL_MessageBoxButtonData,
    pub colorScheme:    Option<*SDL_MessageBoxColorScheme>,
}

extern "C" {
    pub fn SDL_ShowMessageBox(messageboxdata: *SDL_MessageBoxData, buttonid: *mut c_int) -> c_int;
    pub fn SDL_ShowSimpleMessageBox(flags: SDL_MessageBoxFlags, title: *c_char, message: *c_char, window: Option<*SDL_Window>) -> c_int;
}
