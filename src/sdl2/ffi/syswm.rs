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

use ffi::stdinc::SDL_bool;
use ffi::version::SDL_version;
use ffi::video::SDL_Window;
use libc::c_int;

// SDL_syswm.h

#[repr(C)]
pub enum SDL_SYSWM_TYPE {
    SDL_SYSWM_UNKNOWN,
    SDL_SYSWM_WINDOWS,
    SDL_SYSWM_X11,
    SDL_SYSWM_DIRECTFB,
    SDL_SYSWM_COCOA,
    SDL_SYSWM_UIKIT,
}

pub struct SDL_SysWMmsg {
    pub version:    SDL_version,
    pub subsystem:  SDL_SYSWM_TYPE,
    pub dummy_msg:  c_int,  // union { ... } msg;
}

pub struct SDL_SysWMinfo {
    pub version:    SDL_version,
    pub subsystem:  SDL_SYSWM_TYPE,
    pub dummy_info: c_int, // union { ... } info;
}

extern "C" {
    pub fn SDL_GetWindowWMInfo(window: *SDL_Window, info: *SDL_SysWMinfo) -> SDL_bool;
}
