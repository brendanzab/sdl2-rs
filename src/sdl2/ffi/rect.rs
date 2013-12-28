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

use ffi::stdinc::{SDL_bool, SDL_TRUE, SDL_FALSE};
use std::libc::c_int;

// SDL_rect.h

pub struct SDL_Point {
    x: c_int,
    y: c_int,
}

#[deriving(Eq)]
pub struct SDL_Rect {
    x: c_int, y: c_int,
    w: c_int, h: c_int,
}

#[inline]
pub fn SDL_RectEmpty(r: &SDL_Rect) -> SDL_bool {
    if (r.w <= 0) || (r.h <= 0) { SDL_TRUE } else { SDL_FALSE }
}

#[inline]
pub fn SDL_RectEquals(a: &SDL_Rect, b: &SDL_Rect) -> SDL_bool {
    if *a == *b { SDL_TRUE } else { SDL_FALSE }
}

extern "C" {
    pub fn SDL_HasIntersection(A: *SDL_Rect, B: *SDL_Rect) -> SDL_bool;
    pub fn SDL_IntersectRect(A: *SDL_Rect, B: *SDL_Rect, result: *mut SDL_Rect) -> SDL_bool;
    pub fn SDL_UnionRect(A: *SDL_Rect, B: *SDL_Rect, result: *mut SDL_Rect);
    pub fn SDL_EnclosePoints(points: *SDL_Point, count: c_int, clip: *SDL_Rect, result: *mut SDL_Rect) -> SDL_bool;
    pub fn SDL_IntersectRectAndLine(r: *SDL_Rect, X1: *c_int, Y1: *c_int, X2: *c_int, Y2: *c_int) -> SDL_bool;
}
