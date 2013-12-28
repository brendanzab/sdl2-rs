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
use ffi::stdinc::{SDL_bool, Uint16, Uint32};
use std::libc::{c_char, c_float, c_int, c_void};

// SDL_video.h

pub struct SDL_DisplayMode {
    format: Uint32,                 // pixel format
    w: c_int,                       // width
    h: c_int,                       // height
    refresh_rate: c_int,            // refresh rate (or zero for unspecified)
    driverdata: *c_void,            // driver-specific data, initialize to 0
}

pub struct SDL_Window;

#[repr(C)]
pub enum SDL_WindowFlags {
    SDL_WINDOW_FULLSCREEN           = 0x00000001,
    SDL_WINDOW_OPENGL               = 0x00000002,
    SDL_WINDOW_SHOWN                = 0x00000004,
    SDL_WINDOW_HIDDEN               = 0x00000008,
    SDL_WINDOW_BORDERLESS           = 0x00000010,
    SDL_WINDOW_RESIZABLE            = 0x00000020,
    SDL_WINDOW_MINIMIZED            = 0x00000040,
    SDL_WINDOW_MAXIMIZED            = 0x00000080,
    SDL_WINDOW_INPUT_GRABBED        = 0x00000100,
    SDL_WINDOW_INPUT_FOCUS          = 0x00000200,
    SDL_WINDOW_MOUSE_FOCUS          = 0x00000400,
    SDL_WINDOW_FULLSCREEN_DESKTOP   = 0x00000001 | 0x00001000, // (SDL_WINDOW_FULLSCREEN | 0x00001000),
    SDL_WINDOW_FOREIGN              = 0x00000800,
}

pub static SDL_WINDOWPOS_UNDEFINED_MASK: c_int = 0x1FFF0000;
macro_rules! SDL_WINDOWPOS_UNDEFINED_DISPLAY(($x:expr) => (SDL_WINDOWPOS_UNDEFINED_MASK | $x))
pub static SDL_WINDOWPOS_UNDEFINED: c_int = SDL_WINDOWPOS_UNDEFINED_DISPLAY!(0);
macro_rules! SDL_WINDOWPOS_ISUNDEFINED(($x:expr) => (($x & 0xFFFF0000) == SDL_WINDOWPOS_UNDEFINED_MASK))

pub static SDL_WINDOWPOS_CENTERED_MASK: c_int = 0x2FFF0000;
macro_rules! SDL_WINDOWPOS_CENTERED_DISPLAY(($x:expr) => (SDL_WINDOWPOS_CENTERED_MASK | $x))
pub static SDL_WINDOWPOS_CENTERED: c_int = SDL_WINDOWPOS_CENTERED_DISPLAY!(0);
macro_rules! SDL_WINDOWPOS_ISCENTERED(($x:expr) => (($x & 0xFFFF0000) == SDL_WINDOWPOS_CENTERED_MASK))

#[repr(C)]
pub enum SDL_WindowEventID {
    SDL_WINDOWEVENT_NONE,           // Never used
    SDL_WINDOWEVENT_SHOWN,          // Window has been shown
    SDL_WINDOWEVENT_HIDDEN,         // Window has been hidden
    SDL_WINDOWEVENT_EXPOSED,        // Window has been exposed and should be redrawn
    SDL_WINDOWEVENT_MOVED,          // Window has been moved to data1, data2
    SDL_WINDOWEVENT_RESIZED,        // Window has been resized to data1xdata2
    SDL_WINDOWEVENT_SIZE_CHANGED,   // The window size has changed, either as a result of an API call or through the system or user changing the window size.
    SDL_WINDOWEVENT_MINIMIZED,      // Window has been minimized
    SDL_WINDOWEVENT_MAXIMIZED,      // Window has been maximized
    SDL_WINDOWEVENT_RESTORED,       // Window has been restored to normal size and position
    SDL_WINDOWEVENT_ENTER,          // Window has gained mouse focus
    SDL_WINDOWEVENT_LEAVE,          // Window has lost mouse focus
    SDL_WINDOWEVENT_FOCUS_GAINED,   // Window has gained keyboard focus
    SDL_WINDOWEVENT_FOCUS_LOST,     // Window has lost keyboard focus
    SDL_WINDOWEVENT_CLOSE,          // The window manager requests that the window be closed
}

pub type SDL_GLContext = *c_void;

#[repr(C)]
pub enum SDL_GLattr {
    SDL_GL_RED_SIZE,
    SDL_GL_GREEN_SIZE,
    SDL_GL_BLUE_SIZE,
    SDL_GL_ALPHA_SIZE,
    SDL_GL_BUFFER_SIZE,
    SDL_GL_DOUBLEBUFFER,
    SDL_GL_DEPTH_SIZE,
    SDL_GL_STENCIL_SIZE,
    SDL_GL_ACCUM_RED_SIZE,
    SDL_GL_ACCUM_GREEN_SIZE,
    SDL_GL_ACCUM_BLUE_SIZE,
    SDL_GL_ACCUM_ALPHA_SIZE,
    SDL_GL_STEREO,
    SDL_GL_MULTISAMPLEBUFFERS,
    SDL_GL_MULTISAMPLESAMPLES,
    SDL_GL_ACCELERATED_VISUAL,
    SDL_GL_RETAINED_BACKING,
    SDL_GL_CONTEXT_MAJOR_VERSION,
    SDL_GL_CONTEXT_MINOR_VERSION,
    SDL_GL_CONTEXT_EGL,
    SDL_GL_CONTEXT_FLAGS,
    SDL_GL_CONTEXT_PROFILE_MASK,
    SDL_GL_SHARE_WITH_CURRENT_CONTEXT,
}

#[repr(C)]
pub enum SDL_GLprofile {
    SDL_GL_CONTEXT_PROFILE_CORE           = 0x0001,
    SDL_GL_CONTEXT_PROFILE_COMPATIBILITY  = 0x0002,
    SDL_GL_CONTEXT_PROFILE_ES             = 0x0004,
}

#[repr(C)]
pub enum SDL_GLcontextFlag {
    SDL_GL_CONTEXT_DEBUG_FLAG              = 0x0001,
    SDL_GL_CONTEXT_FORWARD_COMPATIBLE_FLAG = 0x0002,
    SDL_GL_CONTEXT_ROBUST_ACCESS_FLAG      = 0x0004,
    SDL_GL_CONTEXT_RESET_ISOLATION_FLAG    = 0x0008,
}


/* Function prototypes */
extern "C" {
    pub fn SDL_GetNumVideoDrivers() -> c_int;
    pub fn SDL_GetVideoDriver(index: c_int) -> *c_char;
    pub fn SDL_VideoInit(driver_name: *c_char) -> c_int;
    pub fn SDL_VideoQuit();
    pub fn SDL_GetCurrentVideoDriver() -> *c_char;
    pub fn SDL_GetNumVideoDisplays() -> c_int;
    pub fn SDL_GetDisplayName(displayIndex: c_int) -> *c_char;
    pub fn SDL_GetDisplayBounds(displayIndex: c_int, rect: *mut SDL_Rect) -> c_int;
    pub fn SDL_GetNumDisplayModes(displayIndex: c_int) -> c_int;
    pub fn SDL_GetDisplayMode(displayIndex: c_int, modeIndex: c_int, mode: *mut SDL_DisplayMode) -> c_int;
    pub fn SDL_GetDesktopDisplayMode(displayIndex: c_int, mode: *mut SDL_DisplayMode) -> c_int;
    pub fn SDL_GetCurrentDisplayMode(displayIndex: c_int, mode: *mut SDL_DisplayMode) -> c_int;
    pub fn SDL_GetClosestDisplayMode(displayIndex: c_int, mode: *SDL_DisplayMode, closest: *mut SDL_DisplayMode) -> *SDL_DisplayMode;
    pub fn SDL_GetWindowDisplayIndex(window: *SDL_Window) -> c_int;
    pub fn SDL_SetWindowDisplayMode(window: *SDL_Window, mode: *SDL_DisplayMode) -> c_int;
    pub fn SDL_GetWindowDisplayMode(window: *SDL_Window, mode: *mut SDL_DisplayMode) -> c_int;
    pub fn SDL_GetWindowPixelFormat(window: *SDL_Window) -> Uint32;
    pub fn SDL_CreateWindow(title: *c_char, x: c_int, y: c_int, w: c_int, h: c_int, flags: Uint32) -> *SDL_Window;
    pub fn SDL_CreateWindowFrom(data: *c_void) -> *SDL_Window;
    pub fn SDL_GetWindowID(window: *SDL_Window) -> Uint32;
    pub fn SDL_GetWindowFromID(id: Uint32) -> *SDL_Window;
    pub fn SDL_GetWindowFlags(window: *SDL_Window) -> Uint32;
    pub fn SDL_SetWindowTitle(window: *SDL_Window, title: *c_char);
    pub fn SDL_GetWindowTitle(window: *SDL_Window) -> *c_char;
    // pub fn SDL_SetWindowIcon(window: *SDL_Window, icon: *SDL_Surface);
    pub fn SDL_SetWindowData(window: *SDL_Window, name: *c_char, userdata: *c_void) -> *c_void;
    pub fn SDL_GetWindowData(window: *SDL_Window, name: *c_char) -> *c_void;
    pub fn SDL_SetWindowPosition(window: *SDL_Window, x: c_int, y: c_int);
    pub fn SDL_GetWindowPosition(window: *SDL_Window, x: *mut c_int, y: *mut c_int);
    pub fn SDL_SetWindowSize(window: *SDL_Window, w: c_int, h: c_int);
    pub fn SDL_GetWindowSize(window: *SDL_Window, w: *mut c_int, h: *mut c_int);
    pub fn SDL_SetWindowMinimumSize(window: *SDL_Window, min_w: c_int, min_h: c_int);
    pub fn SDL_GetWindowMinimumSize(window: *SDL_Window, w: *mut c_int, h: *mut c_int);
    pub fn SDL_SetWindowMaximumSize(window: *SDL_Window, max_w: c_int, max_h: c_int);
    pub fn SDL_GetWindowMaximumSize(window: *SDL_Window, w: *mut c_int, h: *mut c_int);
    pub fn SDL_SetWindowBordered(window: *SDL_Window, bordered: SDL_bool);
    pub fn SDL_ShowWindow(window: *SDL_Window);
    pub fn SDL_HideWindow(window: *SDL_Window);
    pub fn SDL_RaiseWindow(window: *SDL_Window);
    pub fn SDL_MaximizeWindow(window: *SDL_Window);
    pub fn SDL_MinimizeWindow(window: *SDL_Window);
    pub fn SDL_RestoreWindow(window: *SDL_Window);
    pub fn SDL_SetWindowFullscreen(window: *SDL_Window, flags: Uint32) -> c_int;
    // pub fn SDL_GetWindowSurface(window: *SDL_Window) -> *SDL_Surface;
    pub fn SDL_UpdateWindowSurface(window: *SDL_Window) -> c_int;
    pub fn SDL_UpdateWindowSurfaceRects(window: *SDL_Window, rects: *SDL_Rect, numrects: c_int) -> c_int;
    pub fn SDL_SetWindowGrab(window: *SDL_Window, grabbed: SDL_bool);
    pub fn SDL_GetWindowGrab(window: *SDL_Window) -> SDL_bool;
    pub fn SDL_SetWindowBrightness(window: *SDL_Window, brightness: c_float) -> c_int;
    pub fn SDL_GetWindowBrightness(window: *SDL_Window) -> c_float;
    pub fn SDL_SetWindowGammaRamp(window: *SDL_Window, red: *Uint16, green: *Uint16, blue: *Uint16) -> c_int;
    pub fn SDL_GetWindowGammaRamp(window: *SDL_Window, red: *mut Uint16, green: *mut Uint16, blue: *mut Uint16) -> c_int;
    pub fn SDL_DestroyWindow(window: *SDL_Window);
    pub fn SDL_IsScreenSaverEnabled() -> SDL_bool;
    pub fn SDL_EnableScreenSaver();
    pub fn SDL_DisableScreenSaver();
    pub fn SDL_GL_LoadLibrary(path: *c_char) -> c_int;
    pub fn SDL_GL_GetProcAddress(glproc: *c_char) -> *c_void;
    pub fn SDL_GL_UnloadLibrary();
    pub fn SDL_GL_ExtensionSupported(extension: *c_char) -> SDL_bool;
    pub fn SDL_GL_SetAttribute(attr: SDL_GLattr, value: c_int) -> c_int;
    pub fn SDL_GL_GetAttribute(attr: SDL_GLattr, value: *mut c_int) -> c_int;
    pub fn SDL_GL_CreateContext(window: *SDL_Window) -> SDL_GLContext;
    pub fn SDL_GL_MakeCurrent(window: *SDL_Window, context: SDL_GLContext) -> c_int;
    pub fn SDL_GL_GetCurrentWindow() -> *SDL_Window;
    pub fn SDL_GL_GetCurrentContext() -> SDL_GLContext;
    pub fn SDL_GL_SetSwapInterval(interval: c_int) -> c_int;
    pub fn SDL_GL_GetSwapInterval() -> c_int;
    pub fn SDL_GL_SwapWindow(window: *SDL_Window);
    pub fn SDL_GL_DeleteContext(context: SDL_GLContext);
}
