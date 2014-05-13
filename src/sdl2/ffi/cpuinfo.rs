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
use libc::c_int;

// SDL_cpuinfo.h

extern "C" {
    pub fn SDL_GetCPUCount() -> c_int;
    pub fn SDL_GetCPUCacheLineSize() -> c_int;
    pub fn SDL_HasRDTSC() -> SDL_bool;
    pub fn SDL_HasAltiVec() -> SDL_bool;
    pub fn SDL_HasMMX() -> SDL_bool;
    pub fn SDL_Has3DNow() -> SDL_bool;
    pub fn SDL_HasSSE() -> SDL_bool;
    pub fn SDL_HasSSE2() -> SDL_bool;
    pub fn SDL_HasSSE3() -> SDL_bool;
    pub fn SDL_HasSSE41() -> SDL_bool;
    pub fn SDL_HasSSE42() -> SDL_bool;
}
