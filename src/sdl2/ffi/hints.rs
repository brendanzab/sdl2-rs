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

use ffi::stdinc::SDL_bool;
use std::libc::{c_char, c_void};

// SDL_hints.h

pub static SDL_HINT_FRAMEBUFFER_ACCELERATION:               &'static str = &'static "SDL_FRAMEBUFFER_ACCELERATION";
pub static SDL_HINT_RENDER_DRIVER:                          &'static str = &'static "SDL_RENDER_DRIVER";
pub static SDL_HINT_RENDER_OPENGL_SHADERS:                  &'static str = &'static "SDL_RENDER_OPENGL_SHADERS";
pub static SDL_HINT_RENDER_SCALE_QUALITY:                   &'static str = &'static "SDL_RENDER_SCALE_QUALITY";
pub static SDL_HINT_RENDER_VSYNC:                           &'static str = &'static "SDL_RENDER_VSYNC";
pub static SDL_HINT_VIDEO_X11_XVIDMODE:                     &'static str = &'static "SDL_VIDEO_X11_XVIDMODE";
pub static SDL_HINT_VIDEO_X11_XINERAMA:                     &'static str = &'static "SDL_VIDEO_X11_XINERAMA";
pub static SDL_HINT_VIDEO_X11_XRANDR:                       &'static str = &'static "SDL_VIDEO_X11_XRANDR";
pub static SDL_HINT_GRAB_KEYBOARD:                          &'static str = &'static "SDL_GRAB_KEYBOARD";
pub static SDL_HINT_VIDEO_MINIMIZE_ON_FOCUS_LOSS:           &'static str = &'static "SDL_VIDEO_MINIMIZE_ON_FOCUS_LOSS";
pub static SDL_HINT_IDLE_TIMER_DISABLED:                    &'static str = &'static "SDL_IOS_IDLE_TIMER_DISABLED";
pub static SDL_HINT_ORIENTATIONS:                           &'static str = &'static "SDL_IOS_ORIENTATIONS";
pub static SDL_HINT_XINPUT_ENABLED:                         &'static str = &'static "SDL_XINPUT_ENABLED";
pub static SDL_HINT_GAMECONTROLLERCONFIG:                   &'static str = &'static "SDL_GAMECONTROLLERCONFIG";
pub static SDL_HINT_JOYSTICK_ALLOW_BACKGROUND_EVENTS:       &'static str = &'static "SDL_JOYSTICK_ALLOW_BACKGROUND_EVENTS";
pub static SDL_HINT_ALLOW_TOPMOST:                          &'static str = &'static "SDL_ALLOW_TOPMOST";
pub static SDL_HINT_TIMER_RESOLUTION:                       &'static str = &'static "SDL_TIMER_RESOLUTION";

#[repr(C)]
pub enum SDL_HintPriority {
    SDL_HINT_DEFAULT,
    SDL_HINT_NORMAL,
    SDL_HINT_OVERRIDE,
}

pub type SDL_HintCallback = extern "C" fn(userdata: *c_void, name: *c_char, oldValue: *c_char, newValue: *c_char);

extern "C" {
    pub fn SDL_SetHintWithPriority(name: *c_char, value: *c_char, priority: SDL_HintPriority) -> SDL_bool;
    pub fn SDL_SetHint(name: *c_char, value: *c_char) -> SDL_bool;
    pub fn SDL_GetHint(name: *c_char) -> *c_char;
    pub fn SDL_AddHintCallback(name: *c_char, callback: SDL_HintCallback, userdata: *c_void);
    pub fn SDL_DelHintCallback(name: *c_char, callback: SDL_HintCallback, userdata: *c_void);
    pub fn SDL_ClearHints();
}
