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

use ffi::stdinc::{SDL_bool, Uint8, Uint16, Uint32};
use libc::{c_char, c_float, c_int};

// SDL_pixels.h

pub static SDL_ALPHA_OPAQUE: Uint8 = 255;
pub static SDL_ALPHA_TRANSPARENT: Uint8 = 0;

#[repr(u32)]
#[deriving(Eq)]
pub enum SDL_PixelType {
    SDL_PIXELTYPE_UNKNOWN,
    SDL_PIXELTYPE_INDEX1,
    SDL_PIXELTYPE_INDEX4,
    SDL_PIXELTYPE_INDEX8,
    SDL_PIXELTYPE_PACKED8,
    SDL_PIXELTYPE_PACKED16,
    SDL_PIXELTYPE_PACKED32,
    SDL_PIXELTYPE_ARRAYU8,
    SDL_PIXELTYPE_ARRAYU16,
    SDL_PIXELTYPE_ARRAYU32,
    SDL_PIXELTYPE_ARRAYF16,
    SDL_PIXELTYPE_ARRAYF32,
}

#[repr(u32)]
#[deriving(Eq)]
pub enum SDL_BitmapOrder {
    SDL_BITMAPORDER_NONE,
    SDL_BITMAPORDER_4321,
    SDL_BITMAPORDER_1234,
}

#[repr(u32)]
#[deriving(Eq)]
pub enum SDL_PackedOrder {
    SDL_PACKEDORDER_NONE,
    SDL_PACKEDORDER_XRGB,
    SDL_PACKEDORDER_RGBX,
    SDL_PACKEDORDER_ARGB,
    SDL_PACKEDORDER_RGBA,
    SDL_PACKEDORDER_XBGR,
    SDL_PACKEDORDER_BGRX,
    SDL_PACKEDORDER_ABGR,
    SDL_PACKEDORDER_BGRA,
}

#[repr(u32)]
#[deriving(Eq)]
pub enum SDL_ArrayOrder {
    SDL_ARRAYORDER_NONE,
    SDL_ARRAYORDER_RGB,
    SDL_ARRAYORDER_RGBA,
    SDL_ARRAYORDER_ARGB,
    SDL_ARRAYORDER_BGR,
    SDL_ARRAYORDER_BGRA,
    SDL_ARRAYORDER_ABGR,
}

#[repr(u32)]
#[deriving(Eq)]
pub enum SDL_PackedLayout {
    SDL_PACKEDLAYOUT_NONE,
    SDL_PACKEDLAYOUT_332,
    SDL_PACKEDLAYOUT_4444,
    SDL_PACKEDLAYOUT_1555,
    SDL_PACKEDLAYOUT_5551,
    SDL_PACKEDLAYOUT_565,
    SDL_PACKEDLAYOUT_8888,
    SDL_PACKEDLAYOUT_2101010,
    SDL_PACKEDLAYOUT_1010102,
}

// from SDL_stdinc.h
macro_rules! SDL_FOURCC(
    ($A:expr, $B:expr, $C:expr, $D:expr) => (
        ((($A as Uint8) as Uint32) << 0) |
        ((($B as Uint8) as Uint32) << 8) |
        ((($C as Uint8) as Uint32) << 16) |
        ((($D as Uint8) as Uint32) << 24)
    )
)

macro_rules! SDL_DEFINE_PIXELFOURCC(
    ($A:expr, $B:expr, $C:expr, $D:expr) => (SDL_FOURCC!($A, $B, $C, $D))
)

macro_rules! SDL_DEFINE_PIXELFORMAT(
    ($ty:expr, $order:expr, $layout:expr, $bits:expr, $bytes:expr) => (
        (1 << 28) |
        ($ty      as Uint32 << 24) |
        ($order   as Uint32 << 20) |
        ($layout  as Uint32 << 16) |
        ($bits    as Uint32 << 8)  |
        ($bytes   as Uint32 << 0)
    )
)

macro_rules! SDL_PIXELFLAG    (($X:expr) => (($X >> 28) & 0x0F))
macro_rules! SDL_PIXELTYPE    (($X:expr) => (($X >> 24) & 0x0F))
macro_rules! SDL_PIXELORDER   (($X:expr) => (($X >> 20) & 0x0F))
macro_rules! SDL_PIXELLAYOUT  (($X:expr) => (($X >> 16) & 0x0F))
macro_rules! SDL_BITSPERPIXEL (($X:expr) => (($X >>  8) & 0xFF))

macro_rules! SDL_ISPIXELFORMAT_FOURCC(
    ($format:expr) => ({
        let format = $format;
        format && (SDL_PIXELFLAG!(format) != 1)
    })
)

macro_rules! SDL_BYTESPERPIXEL(
    ($X:expr) => ({
        let x = $X;
        if SDL_ISPIXELFORMAT_FOURCC!(x) {
            if (x == SDL_PIXELFORMAT_YUY2)
            || (x == SDL_PIXELFORMAT_UYVY)
            || (x == SDL_PIXELFORMAT_YVYU) { 2 } else { 1 }
        } else {
            (x >> 0) & 0xFF
        }
    })
)

macro_rules! SDL_ISPIXELFORMAT_INDEXED(
    ($format:expr) => ({
        let format = $format;
        !SDL_ISPIXELFORMAT_FOURCC!(format) && (
            (SDL_PIXELTYPE!(format) == SDL_PIXELTYPE_INDEX1) ||
            (SDL_PIXELTYPE!(format) == SDL_PIXELTYPE_INDEX4) ||
            (SDL_PIXELTYPE!(format) == SDL_PIXELTYPE_INDEX8)
        )
    })
)

macro_rules! SDL_ISPIXELFORMAT_ALPHA(
    ($format:expr) => ({
        let format = $format;
        !SDL_ISPIXELFORMAT_FOURCC!(format) && (
            (SDL_PIXELORDER!(format) == SDL_PACKEDORDER_ARGB) ||
            (SDL_PIXELORDER!(format) == SDL_PACKEDORDER_RGBA) ||
            (SDL_PIXELORDER!(format) == SDL_PACKEDORDER_ABGR) ||
            (SDL_PIXELORDER!(format) == SDL_PACKEDORDER_BGRA)
        )
    })
)

#[repr(u32)]
#[deriving(Eq)]
// enum SDL_PixelFormat {
pub enum SDL_PixelFormat_ {
    SDL_PIXELFORMAT_UNKNOWN,
    SDL_PIXELFORMAT_INDEX1LSB   = SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_INDEX1, SDL_BITMAPORDER_4321, 0, 1, 0),
    SDL_PIXELFORMAT_INDEX1MSB   = SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_INDEX1, SDL_BITMAPORDER_1234, 0, 1, 0),
    SDL_PIXELFORMAT_INDEX4LSB   = SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_INDEX4, SDL_BITMAPORDER_4321, 0, 4, 0),
    SDL_PIXELFORMAT_INDEX4MSB   = SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_INDEX4, SDL_BITMAPORDER_1234, 0, 4, 0),
    SDL_PIXELFORMAT_INDEX8      = SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_INDEX8, 0, 0, 8, 1),
    SDL_PIXELFORMAT_RGB332      = SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_PACKED8, SDL_PACKEDORDER_XRGB, SDL_PACKEDLAYOUT_332, 8, 1),
    SDL_PIXELFORMAT_RGB444      = SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_XRGB, SDL_PACKEDLAYOUT_4444, 12, 2),
    SDL_PIXELFORMAT_RGB555      = SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_XRGB, SDL_PACKEDLAYOUT_1555, 15, 2),
    SDL_PIXELFORMAT_BGR555      = SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_XBGR, SDL_PACKEDLAYOUT_1555, 15, 2),
    SDL_PIXELFORMAT_ARGB4444    = SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_ARGB, SDL_PACKEDLAYOUT_4444, 16, 2),
    SDL_PIXELFORMAT_RGBA4444    = SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_RGBA, SDL_PACKEDLAYOUT_4444, 16, 2),
    SDL_PIXELFORMAT_ABGR4444    = SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_ABGR, SDL_PACKEDLAYOUT_4444, 16, 2),
    SDL_PIXELFORMAT_BGRA4444    = SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_BGRA, SDL_PACKEDLAYOUT_4444, 16, 2),
    SDL_PIXELFORMAT_ARGB1555    = SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_ARGB, SDL_PACKEDLAYOUT_1555, 16, 2),
    SDL_PIXELFORMAT_RGBA5551    = SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_RGBA, SDL_PACKEDLAYOUT_5551, 16, 2),
    SDL_PIXELFORMAT_ABGR1555    = SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_ABGR, SDL_PACKEDLAYOUT_1555, 16, 2),
    SDL_PIXELFORMAT_BGRA5551    = SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_BGRA, SDL_PACKEDLAYOUT_5551, 16, 2),
    SDL_PIXELFORMAT_RGB565      = SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_XRGB, SDL_PACKEDLAYOUT_565, 16, 2),
    SDL_PIXELFORMAT_BGR565      = SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_XBGR, SDL_PACKEDLAYOUT_565, 16, 2),
    SDL_PIXELFORMAT_RGB24       = SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_ARRAYU8, SDL_ARRAYORDER_RGB, 0, 24, 3),
    SDL_PIXELFORMAT_BGR24       = SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_ARRAYU8, SDL_ARRAYORDER_BGR, 0, 24, 3),
    SDL_PIXELFORMAT_RGB888      = SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_PACKED32, SDL_PACKEDORDER_XRGB, SDL_PACKEDLAYOUT_8888, 24, 4),
    SDL_PIXELFORMAT_RGBX8888    = SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_PACKED32, SDL_PACKEDORDER_RGBX, SDL_PACKEDLAYOUT_8888, 24, 4),
    SDL_PIXELFORMAT_BGR888      = SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_PACKED32, SDL_PACKEDORDER_XBGR, SDL_PACKEDLAYOUT_8888, 24, 4),
    SDL_PIXELFORMAT_BGRX8888    = SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_PACKED32, SDL_PACKEDORDER_BGRX, SDL_PACKEDLAYOUT_8888, 24, 4),
    SDL_PIXELFORMAT_ARGB8888    = SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_PACKED32, SDL_PACKEDORDER_ARGB, SDL_PACKEDLAYOUT_8888, 32, 4),
    SDL_PIXELFORMAT_RGBA8888    = SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_PACKED32, SDL_PACKEDORDER_RGBA, SDL_PACKEDLAYOUT_8888, 32, 4),
    SDL_PIXELFORMAT_ABGR8888    = SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_PACKED32, SDL_PACKEDORDER_ABGR, SDL_PACKEDLAYOUT_8888, 32, 4),
    SDL_PIXELFORMAT_BGRA8888    = SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_PACKED32, SDL_PACKEDORDER_BGRA, SDL_PACKEDLAYOUT_8888, 32, 4),
    SDL_PIXELFORMAT_ARGB2101010 = SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_PACKED32, SDL_PACKEDORDER_ARGB, SDL_PACKEDLAYOUT_2101010, 32, 4),

    SDL_PIXELFORMAT_YV12        = SDL_DEFINE_PIXELFOURCC!('Y', 'V', '1', '2'),
    SDL_PIXELFORMAT_IYUV        = SDL_DEFINE_PIXELFOURCC!('I', 'Y', 'U', 'V'),
    SDL_PIXELFORMAT_YUY2        = SDL_DEFINE_PIXELFOURCC!('Y', 'U', 'Y', '2'),
    SDL_PIXELFORMAT_UYVY        = SDL_DEFINE_PIXELFOURCC!('U', 'Y', 'V', 'Y'),
    SDL_PIXELFORMAT_YVYU        = SDL_DEFINE_PIXELFOURCC!('Y', 'V', 'Y', 'U'),
}

pub struct SDL_Color {
    pub r: Uint8,
    pub g: Uint8,
    pub b: Uint8,
    pub a: Uint8,
}

pub type SDL_Colour = SDL_Color;

pub struct SDL_Palette {
    pub ncolors: int,
    pub colors: *SDL_Color,
    pub version: Uint32,
    pub refcount: int,
}

pub struct SDL_PixelFormat {
    pub format: Uint32,
    pub palette: SDL_Palette,
    pub BitsPerPixel: Uint8,
    pub BytesPerPixel: Uint8,
    pub padding: [Uint8, ..2],
    pub Rmask: Uint32,
    pub Gmask: Uint32,
    pub Bmask: Uint32,
    pub Amask: Uint32,
    pub Rloss: Uint8,
    pub Gloss: Uint8,
    pub Bloss: Uint8,
    pub Aloss: Uint8,
    pub Rshift: Uint8,
    pub Gshift: Uint8,
    pub Bshift: Uint8,
    pub Ashift: Uint8,
    pub refcount: c_int,
    pub next: *SDL_PixelFormat,
}

extern "C" {
    fn SDL_GetPixelFormatName(format: Uint32) -> *c_char;
    fn SDL_PixelFormatEnumToMasks(format: Uint32, bpp: *c_int, Rmask: *mut Uint32, Gmask: *mut Uint32, Bmask: *mut Uint32, Amask: *mut Uint32) -> SDL_bool;
    fn SDL_MasksToPixelFormatEnum(bpp: c_int, Rmask: Uint32, Gmask: Uint32, Bmask: Uint32, Amask: Uint32) -> Uint32;
    fn SDL_AllocFormat(pixel_format: Uint32) -> *SDL_PixelFormat;
    fn SDL_FreeFormat(format: *mut SDL_PixelFormat);
    fn SDL_AllocPalette(ncolors: c_int) -> *SDL_Palette;
    fn SDL_SetPixelFormatPalette(format: *mut SDL_PixelFormat, palette: *SDL_Palette) -> c_int;
    fn SDL_SetPaletteColors(palette: *mut SDL_Palette, colors: *SDL_Color, firstcolor: c_int, ncolors: c_int) -> c_int;
    fn SDL_FreePalette(palette: *mut SDL_Palette);
    fn SDL_MapRGB(format: *SDL_PixelFormat, r: Uint8, g: Uint8, b: Uint8) -> Uint32;
    fn SDL_MapRGBA(format: *SDL_PixelFormat, r: Uint8, g: Uint8, b: Uint8, a: Uint8) -> Uint32;
    fn SDL_GetRGB(pixel: Uint32, format: *SDL_PixelFormat, r: *mut Uint8, g: *mut Uint8, b: *mut Uint8);
    fn SDL_GetRGBA(pixel: Uint32, format: *SDL_PixelFormat, r: *mut Uint8, g: *mut Uint8, b: *mut Uint8, a: *mut Uint8);
    fn SDL_CalculateGammaRamp(gamma: c_float, ramp: *mut Uint16);
}
