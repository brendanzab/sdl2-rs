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

use ffi::events::{SDL_PEEKEVENT, SDL_QUIT};
use ffi::events::{SDL_PumpEvents, SDL_PeepEvents};
use ffi::stdinc::{SDL_bool, Uint32};
use std::ptr::null;

// SDL_quit.h

#[inline]
pub unsafe fn SDL_QuitRequested() -> SDL_bool {
    SDL_PumpEvents();
    SDL_bool::from_bool(
        SDL_PeepEvents(null(), 0, SDL_PEEKEVENT, SDL_QUIT as Uint32,
                       SDL_QUIT as Uint32) > 0
    )
}
