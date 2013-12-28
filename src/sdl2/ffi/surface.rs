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

use ffi::rect::SDL_Rect;
use ffi::stdinc::{SDL_bool, Uint8, Uint32};
use std::libc::{c_int, c_void};

// SDL_surface.h

pub struct SDL_Surface {
    priv flags: Uint32,
    priv format: *c_void, // *SDL_PixelFormat,
    priv h: c_int,
    priv w: c_int,
    priv pitch: c_int,
    pixels: *c_void,
    userdata: *c_void,
    priv locked: c_int,
    priv lock_data: *c_void,
    priv clip_rect: SDL_Rect,
    priv map: *c_void, // struct *SDL_BlitMap
    priv refcount: c_int,
}

impl SDL_Surface {
    pub fn get_flags(&self) -> Uint32 { self.flags }
    // pub fn get_format(&self) -> *SDL_PixelFormat { self.format }
    pub fn get_h(&self) -> c_int { self.h }
    pub fn get_w(&self) -> c_int { self.w }
    pub fn get_pitch(&self) -> c_int { self.pitch }
    pub fn get_locked(&self) -> c_int { self.locked }
    pub fn get_lock_data(&self) -> *c_void { self.lock_data }
    pub fn get_clip_rect<'a>(&'a self) -> &'a SDL_Rect { &'a self.clip_rect }
}

pub type SDL_blit = extern "C" fn(src: *SDL_Surface, srcrect: *SDL_Rect, dst: *mut SDL_Surface, dstrect: *mut SDL_Rect) -> c_int;

// #define SDL_LoadBMP(file)   SDL_LoadBMP_RW(SDL_RWFromFile(file, "rb"), 1)
// #define SDL_SaveBMP(surface, file) \
//         SDL_SaveBMP_RW(surface, SDL_RWFromFile(file, "wb"), 1)

extern "C" {
    pub fn SDL_CreateRGBSurface(flags: Uint32, width: c_int, height: c_int, depth: c_int, Rmask: Uint32, Gmask: Uint32, Bmask: Uint32, Amask: Uint32) -> *SDL_Surface;
    pub fn SDL_CreateRGBSurfaceFrom(pixels: *c_void, width: c_int, height: c_int, depth: c_int, pitch: c_int, Rmask: Uint32, Gmask: Uint32, Bmask: Uint32, Amask: Uint32) -> *SDL_Surface;
    pub fn SDL_FreeSurface(surface: *mut SDL_Surface);
    // pub fn SDL_SetSurfacePalette(surface: *mut SDL_Surface, palette: *SDL_Palette) -> c_int;
    pub fn SDL_LockSurface(surface: *mut SDL_Surface) -> c_int;
    pub fn SDL_UnlockSurface(surface: *mut SDL_Surface);
    // pub fn SDL_LoadBMP_RW(src: *SDL_RWops, freesrc: c_int) -> *SDL_Surface;
    // pub fn SDL_SaveBMP_RW(surface: *SDL_Surface, dst: *SDL_RWops, freedst: c_int) -> c_int;
    pub fn SDL_SetSurfaceRLE(surface: *mut SDL_Surface, flag: c_int) -> c_int;
    pub fn SDL_SetColorKey(surface: *mut SDL_Surface, flag: c_int, key: Uint32) -> c_int;
    pub fn SDL_GetColorKey(surface: *SDL_Surface, key: *mut Uint32) -> c_int;
    pub fn SDL_SetSurfaceColorMod(surface: *mut SDL_Surface, r: Uint8, g: Uint8, b: Uint8) -> c_int;
    pub fn SDL_GetSurfaceColorMod(surface: *SDL_Surface, r: *mut Uint8, g: *mut Uint8, b: *mut Uint8) -> c_int;
    pub fn SDL_SetSurfaceAlphaMod(surface: *mut SDL_Surface, alpha: Uint8) -> c_int;
    pub fn SDL_GetSurfaceAlphaMod(surface: *SDL_Surface, alpha: *mut Uint8) -> c_int;
    // pub fn SDL_SetSurfaceBlendMode(surface: *mut SDL_Surface, blendMode: SDL_BlendMode) -> c_int;
    // pub fn SDL_GetSurfaceBlendMode(surface: *SDL_Surface, blendMode: *mut SDL_BlendMode) -> c_int;
    pub fn SDL_SetClipRect(surface: *mut SDL_Surface, rect: *SDL_Rect) -> SDL_bool;
    pub fn SDL_GetClipRect(surface: *SDL_Surface, rect: *mut SDL_Rect);
    // pub fn SDL_ConvertSurface(src: *SDL_Surface, fmt: *SDL_PixelFormat, flags: Uint32) -> *SDL_Surface;
    pub fn SDL_ConvertSurfaceFormat(src: *SDL_Surface, pixel_format: Uint32, flags: Uint32) -> *SDL_Surface;
    pub fn SDL_ConvertPixels(width: c_int, height: c_int, src_format: Uint32, src: *c_void, src_pitch: c_int, dst_format: Uint32, dst: *mut c_void, dst_pitch: c_int) -> c_int;
    pub fn SDL_FillRect(dst: *mut SDL_Surface, rect: *SDL_Rect, color: Uint32) -> c_int;
    pub fn SDL_FillRects(dst: *mut SDL_Surface, rects: *SDL_Rect, count: c_int, color: Uint32) -> c_int;
    #[link_name="SDL_UpperBlit"] pub fn SDL_BlitSurface(src: *SDL_Surface, srcrect: *SDL_Rect, dst: *mut SDL_Surface, dstrect: *mut SDL_Rect) -> c_int;
    pub fn SDL_UpperBlit(src: *SDL_Surface, srcrect: *SDL_Rect, dst: *mut SDL_Surface, dstrect: *mut SDL_Rect) -> c_int;
    pub fn SDL_LowerBlit(src: *SDL_Surface, srcrect: *SDL_Rect, dst: *mut SDL_Surface, dstrect: *mut SDL_Rect) -> c_int;
    pub fn SDL_SoftStretch(src: *SDL_Surface, srcrect: *SDL_Rect, dst: *mut SDL_Surface, dstrect: *mut SDL_Rect) -> c_int;
    #[link_name="SDL_UpperBlitScaled"] pub fn SDL_BlitScaled(src: *SDL_Surface, srcrect: *SDL_Rect, dst: *mut SDL_Surface, dstrect: *mut SDL_Rect) -> c_int;
    pub fn SDL_UpperBlitScaled(src: *SDL_Surface, srcrect: *SDL_Rect, dst: *mut SDL_Surface, dstrect: *mut SDL_Rect) -> c_int;
    pub fn SDL_LowerBlitScaled(src: *SDL_Surface, srcrect: *SDL_Rect, dst: *mut SDL_Surface, dstrect: *mut SDL_Rect) -> c_int;
}
