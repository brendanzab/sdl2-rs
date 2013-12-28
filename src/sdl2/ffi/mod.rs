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

use ffi::stdinc::Uint32;
use std::libc::c_int;

// Linking

#[nolink]
#[cfg(mac_framework)]
#[link(name = "SDL2", kind = "framework")]
extern { }

#[nolink]
#[cfg(not(mac_framework))]
#[link(name = "SDL2")]
extern { }

#[cfg(target_os="win32")]
#[cfg(target_os="linux")]
#[cfg(target_os="freebsd")]
#[link(name = "SDL2")]
extern { }

// SDL.h

pub static SDL_INIT_TIMER:          Uint32 = 0x00000001;
pub static SDL_INIT_AUDIO:          Uint32 = 0x00000010;
pub static SDL_INIT_VIDEO:          Uint32 = 0x00000020;
pub static SDL_INIT_JOYSTICK:       Uint32 = 0x00000200;
pub static SDL_INIT_HAPTIC:         Uint32 = 0x00001000;
pub static SDL_INIT_GAMECONTROLLER: Uint32 = 0x00002000;
pub static SDL_INIT_EVENTS:         Uint32 = 0x00004000;
pub static SDL_INIT_NOPARACHUTE:    Uint32 = 0x00100000;
pub static SDL_INIT_EVERYTHING:     Uint32 = SDL_INIT_TIMER
                                           | SDL_INIT_AUDIO
                                           | SDL_INIT_VIDEO
                                           | SDL_INIT_EVENTS
                                           | SDL_INIT_JOYSTICK
                                           | SDL_INIT_HAPTIC
                                           | SDL_INIT_GAMECONTROLLER;

extern "C" {
    pub fn SDL_Init(flags: Uint32) -> c_int;
    pub fn SDL_InitSubSystem(flags: Uint32) -> c_int;
    pub fn SDL_QuitSubSystem(flags: Uint32);
    pub fn SDL_WasInit(flags: Uint32) -> Uint32;
    pub fn SDL_Quit();
}

// skipped              // SDL_assert.h
// skipped              // SDL_atomic.h
// TODO                 // SDL_audio.h
// TODO                 // SDL_blendmode.h
pub mod clipboard;      // SDL_clipboard.h
// skipped              // SDL_config.h
pub mod cpuinfo;        // SDL_cpuinfo.h
// skipped              // SDL_endian.h
pub mod error;          // SDL_error.h
pub mod events;         // SDL_events.h
// TODO                 // SDL_gamecontroller.h
// TODO                 // SDL_gesture.h
// TODO                 // SDL_haptic.h
pub mod hints;          // SDL_hints.h
// TODO                 // SDL_joystick.h
pub mod keyboard;       // SDL_keyboard.h
pub mod keycode;        // SDL_keycode.h
// skipped              // SDL_loadso.h
// skipped              // SDL_log.h
// skipped              // SDL_main.h
pub mod messagebox;     // SDL_messagebox.h
pub mod mouse;          // SDL_mouse.h
// skipped              // SDL_mutex.h
// skipped              // SDL_name.h
// skipped              // SDL_opengl.h
// skipped              // SDL_opengles.h
// skipped              // SDL_opengles2.h
// TODO                 // SDL_pixels.h
// skipped              // SDL_platform.h
pub mod power;          // SDL_power.h
pub mod quit;           // SDL_quit.h
pub mod rect;           // SDL_rect.h
// TODO                 // SDL_render.h
// skipped              // SDL_revision.h
// TODO                 // SDL_rwops.h
pub mod scancode;       // SDL_scancode.h
// TODO                 // SDL_shape.h
pub mod stdinc;         // SDL_stdinc.h
pub mod surface;        // SDL_surface.h
// TODO                 // SDL_system.h
pub mod syswm;          // SDL_syswm.h
// skipped              // SDL_test.h
// skipped              // SDL_test_assert.h
// skipped              // SDL_test_common.h
// skipped              // SDL_test_compare.h
// skipped              // SDL_test_crc32.h
// skipped              // SDL_test_font.h
// skipped              // SDL_test_fuzzer.h
// skipped              // SDL_test_harness.h
// skipped              // SDL_test_images.h
// skipped              // SDL_test_log.h
// skipped              // SDL_test_md5.h
// skipped              // SDL_test_random.h
// skipped              // SDL_thread.h
pub mod timer;          // SDL_timer.h
pub mod touch;          // SDL_touch.h
// skipped              // SDL_types.h
pub mod version;        // SDL_version.h
pub mod video;          // SDL_video.h
