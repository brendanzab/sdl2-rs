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

use ffi::stdinc::{SDL_bool, Uint8, Uint32};
use ffi::surface::SDL_Surface;
use ffi::video::SDL_Window;
use libc::c_int;

// SDL_mouse.h

pub enum SDL_Cursor {}

#[repr(C)]
pub enum SDL_SystemCursor {
    SDL_SYSTEM_CURSOR_ARROW,
    SDL_SYSTEM_CURSOR_IBEAM,
    SDL_SYSTEM_CURSOR_WAIT,
    SDL_SYSTEM_CURSOR_CROSSHAIR,
    SDL_SYSTEM_CURSOR_WAITARROW,
    SDL_SYSTEM_CURSOR_SIZENWSE,
    SDL_SYSTEM_CURSOR_SIZENESW,
    SDL_SYSTEM_CURSOR_SIZEWE,
    SDL_SYSTEM_CURSOR_SIZENS,
    SDL_SYSTEM_CURSOR_SIZEALL,
    SDL_SYSTEM_CURSOR_NO,
    SDL_SYSTEM_CURSOR_HAND,
    SDL_NUM_SYSTEM_CURSORS,
}

extern "C" {
    pub fn SDL_GetMouseFocus() -> *SDL_Window;
    pub fn SDL_GetMouseState(x: *mut c_int, y: *mut c_int) -> Uint32;
    pub fn SDL_GetRelativeMouseState(x: *mut c_int, y: *mut c_int) -> Uint32;
    pub fn SDL_WarpMouseInWindow(window: *SDL_Window, x: c_int, y: c_int);
    pub fn SDL_SetRelativeMouseMode(enabled: SDL_bool) -> c_int;
    pub fn SDL_GetRelativeMouseMode() -> SDL_bool;
    pub fn SDL_CreateCursor(data: *Uint8, mask: *Uint8, w: c_int, h: c_int, hot_x: c_int, hot_y: c_int) -> *SDL_Cursor;
    pub fn SDL_CreateColorCursor(surface: *SDL_Surface, hot_x: c_int, hot_y: c_int) -> *SDL_Cursor;
    pub fn SDL_CreateSystemCursor(id: SDL_SystemCursor) -> *SDL_Cursor;
    pub fn SDL_SetCursor(cursor: *SDL_Cursor);
    pub fn SDL_GetCursor() -> *SDL_Cursor;
    pub fn SDL_GetDefaultCursor() -> *SDL_Cursor;
    pub fn SDL_FreeCursor(cursor: *SDL_Cursor);
    pub fn SDL_ShowCursor(toggle: c_int) -> c_int;
}

// #define SDL_BUTTON(X)       (1 << ((X)-1))
// #define SDL_BUTTON_LEFT     1
// #define SDL_BUTTON_MIDDLE   2
// #define SDL_BUTTON_RIGHT    3
// #define SDL_BUTTON_X1       4
// #define SDL_BUTTON_X2       5
// #define SDL_BUTTON_LMASK    SDL_BUTTON(SDL_BUTTON_LEFT)
// #define SDL_BUTTON_MMASK    SDL_BUTTON(SDL_BUTTON_MIDDLE)
// #define SDL_BUTTON_RMASK    SDL_BUTTON(SDL_BUTTON_RIGHT)
// #define SDL_BUTTON_X1MASK   SDL_BUTTON(SDL_BUTTON_X1)
// #define SDL_BUTTON_X2MASK   SDL_BUTTON(SDL_BUTTON_X2)
