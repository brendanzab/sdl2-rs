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

#![feature(globs)]
#![feature(macro_rules)]

#![comment = "Bindings and wrapper for SDL2."]
#![crate_id = "github.com/bjz/sdl2-rs#sdl2:0.1"]
#![crate_type = "lib"]

extern crate libc;
extern crate sync;

use std::c_str::CString;
use std::kinds::marker;
use std::str;

/// A flag used when initialising an SDL subsystem.
pub use InitFlags = ffi::SDL_InitFlags;

/// Foreign function bindings and low level types and enumerations for SDL.
pub mod ffi;

pub mod cpuinfo;
pub mod power;
pub mod rect;
pub mod version;

/// Initilise the timer subsystem.
pub static InitTimer: InitFlags = ffi::SDL_INIT_TIMER;

/// Initilise the audio subsystem.
pub static InitAudio: InitFlags = ffi::SDL_INIT_AUDIO;

/// Initilise the video subsystem. This flag implies that
/// `sdl2::InitEvents` was also set.
pub static InitVideo: InitFlags = ffi::SDL_INIT_VIDEO;

/// Initilise the joystick subsystem. This flag implies that
/// `sdl2::InitEvents` was also set.
pub static InitJoystick: InitFlags = ffi::SDL_INIT_JOYSTICK;

/// Initilise the haptic subsystem.
pub static InitHaptic: InitFlags = ffi::SDL_INIT_HAPTIC;

/// Initilise the game controller subsystem. This flag implies that
/// `sdl2::InitJoystick` was also set.
pub static InitGamecontroller: InitFlags = ffi::SDL_INIT_GAMECONTROLLER;

/// Initialise the event subsystem.
pub static InitEvents: InitFlags = ffi::SDL_INIT_EVENTS;

/// Don't catch fatal signals.
pub static InitNoparachute: InitFlags = ffi::SDL_INIT_NOPARACHUTE;

/// Initilise all the subsystems at once.
pub static InitEverything: InitFlags = ffi::SDL_INIT_EVERYTHING;

/// Initialize an SDL context with the specified subsystems. This must always be
/// called on the main platform thread. The underlying SDL library is
/// automatically shut down on exit or failure of the program.
///
/// # Example
///
/// ~~~
/// extern crate native;
/// extern crate sdl2;
/// 
/// #[start]
/// fn start(argc: int, argv: **u8) -> int {
///     native::start(argc, argv, main) // Run SDL on the main thread
/// }
/// 
/// fn main() {
///     let sdl2 = sdl2::init(sdl2::InitVideo|sdl2::InitEvents);
/// }
/// ~~~
#[inline]
pub fn init(flags: InitFlags) -> Result<Sdl,()> {
    use sync::one::{Once, ONCE_INIT};
    static mut INIT: Once = ONCE_INIT;
    let mut is_ok = false;
    unsafe {
        // This will be called only once
        INIT.doit(|| {
            if ffi::SDL_Init(InitFlags::empty()) == 0 {
                is_ok = true;
                // Set the program to close automatically on exiting
                std::rt::at_exit(proc() {
                    ffi::SDL_Quit()
                });
            }
        })
    }
    if is_ok {
        let sdl = Sdl { marker: marker::NoSend };
        sdl.init_subsystem(flags).map(|_| sdl)
    } else {
        Err(())
    }
}

/// An SDL context. This cannot be sent to other tasks, and should only be
/// initialised on the main platform thread. Whilst this might make performing
/// some operations harder, this is to ensure thread safety is enforced
/// statically. The context can be cloned or implicitly copied if need be for
/// convenience, but note that the state of the context – for example, the
/// initialised subsystems – will be shared between the handles.
#[deriving(Clone)]
pub struct Sdl {
    marker: marker::NoSend,
}

/// Subsystem functions
impl Sdl {
    pub fn init_subsystem(&self, flags: InitFlags) -> Result<(),()> {
        if unsafe { ffi::SDL_InitSubSystem(flags) == 0 }
            { Ok(()) } else { Err(()) }
    }

    /// # Example
    ///
    /// ~~~
    /// let sdl2 = sdl2::init(sdl2::InitVideo|sdl2::InitEvents);
    /// let flags = sdl2.was_init(sdl2::InitVideo|sdl2::InitAudio);
    ///
    /// assert!(flags.contains(sdl2::InitVideo));
    /// assert!(!flags.contains(sdl2::InitEvents));
    /// assert!(!flags.contains(sdl2::InitAudio));
    /// ~~~
    pub fn was_init(&self, flags: InitFlags) -> InitFlags {
        unsafe { ffi::SDL_WasInit(flags) }
    }

    pub fn quit_subsystem(&self, flags: InitFlags) {
        unsafe { ffi::SDL_QuitSubSystem(flags) }
    }
}

/// Event handling
impl Sdl {
    pub fn pump_events(&self) {
        unsafe { ffi::events::SDL_PumpEvents() }
    }
}

/// Quit handling
impl Sdl {
    pub fn quit_requested(&self) -> bool {
        unsafe { ffi::quit::SDL_QuitRequested().to_bool() }
    }
}

/// Clipboard handling
impl Sdl {
    /// Store the string in the clipboard, returning `false` if an error occured.
    pub fn set_clipboard_text(&self, text: &CString) -> bool {
        text.with_ref(|buf| unsafe { ffi::clipboard::SDL_SetClipboardText(buf) }) == 0
    }

    /// Returns the text currently stored in the clipboard.
    pub fn get_clipboard_text(&self) -> CString {
        unsafe { CString::new(ffi::clipboard::SDL_GetClipboardText(), true) }
    }

    /// Returns `true` if the clipboard exists and contains a non-empty string.
    pub fn has_clipboard_text(&self) -> bool {
        unsafe { ffi::clipboard::SDL_HasClipboardText().to_bool() }
    }
}
