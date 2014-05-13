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

use ffi::stdinc::Sint64;
use ffi::touch::SDL_TouchID;

use libc::c_int;

// SDL_gesture.h

pub type SDL_GestureID = Sint64;

extern "C" {
    pub fn SDL_RecordGesture(touchId: SDL_TouchID) -> c_int;
    // pub fn SDL_SaveAllDollarTemplates(src: *SDL_RWops) -> c_int;
    // pub fn SDL_SaveDollarTemplate(gestureId: SDL_GestureID, src: *SDL_RWops) -> c_int;
    // pub fn SDL_LoadDollarTemplates(touchId: SDL_TouchID, src: *SDL_RWops) -> c_int;
}