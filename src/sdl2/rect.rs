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

use std::cast::transmute;
use libc::c_int;

use ffi;

#[deriving(Eq)]
pub struct Point {
    pub x: c_int,
    pub y: c_int,
}

#[deriving(Eq)]
pub struct Rect {
    pub x: c_int,
    pub y: c_int,
    pub w: c_int,
    pub h: c_int,
}

impl Rect {
    fn as_sdl_rect<'a>(&'a self) -> &'a ffi::rect::SDL_Rect {
        unsafe { transmute(self) }
    }

    fn as_mut_sdl_rect<'a>(&'a mut self) -> &'a mut ffi::rect::SDL_Rect {
        unsafe { transmute(self) }
    }

    pub fn empty() -> Rect {
        Rect { x: 0, y: 0, w: 0, h: 0 }
    }

    pub fn is_empty(&self) -> bool {
        ffi::rect::SDL_RectEmpty(self.as_sdl_rect()).to_bool()
    }

    pub fn has_intersection(&self, other: &Rect) -> bool {
        unsafe {
            ffi::rect::SDL_HasIntersection(self.as_sdl_rect(),
                                           other.as_sdl_rect()).to_bool()
        }
    }

    pub fn intersect(&mut self, a: &Rect, b: &Rect) {
        unsafe {
            ffi::rect::SDL_IntersectRect(a.as_sdl_rect(), b.as_sdl_rect(),
                                         self.as_mut_sdl_rect());
        }
    }

    pub fn union(&mut self, a: &Rect, b: &Rect) {
        unsafe {
            ffi::rect::SDL_UnionRect(a.as_sdl_rect(), b.as_sdl_rect(),
                                     self.as_mut_sdl_rect());
        }
    }

    pub fn enclose_points(&mut self, points: &[Point], clip: &Rect) -> bool {
        unsafe {
            ffi::rect::SDL_EnclosePoints(transmute(points.as_ptr()),
                                         points.len() as c_int,
                                         clip.as_sdl_rect(),
                                         self.as_mut_sdl_rect()).to_bool()
        }
    }

    pub fn intersect_with_line(&self, a: &mut Point, b: &mut Point) -> bool {
        unsafe {
            ffi::rect::SDL_IntersectRectAndLine(self.as_sdl_rect(),
                                                &mut a.x, &mut a.y,
                                                &mut b.x, &mut b.y).to_bool()
        }
    }
}
