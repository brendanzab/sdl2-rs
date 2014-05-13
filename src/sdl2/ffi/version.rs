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
use libc::{c_char, c_int};

// SDL_version.h

pub struct SDL_version {
    pub major: Uint8,   // major version
    pub minor: Uint8,   // minor version
    pub patch: Uint8,   // update version
}

extern "C" {
    pub fn SDL_GetVersion(ver: *mut SDL_version);
    pub fn SDL_GetRevision() -> *c_char;
    pub fn SDL_GetRevisionNumber() -> c_int;
}
