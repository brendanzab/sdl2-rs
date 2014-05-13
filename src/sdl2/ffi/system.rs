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

#[cfg(IPHONEOS)] use ffi::stdinc::SDL_bool;
#[cfg(IPHONEOS)] use libc::{c_int, c_void};
#[cfg(ANDROID)]  use libc::{c_char, c_int, c_void};

// SDL_system.h

#[cfg(IPHONEOS)]
extern "C" {
    pub fn SDL_iPhoneSetAnimationCallback(window: *SDL_Window, interval: c_int, callback: extern "C" fn(*c_void), callbackParam: *c_void) -> c_int;
    pub fn SDL_iPhoneSetEventPump(enabled: SDL_bool);
}

#[cfg(ANDROID)]
bitflags!(flags SDL_EventType: c_int {
    static SDL_ANDROID_EXTERNAL_STORAGE_READ  = 0x01,
    static SDL_ANDROID_EXTERNAL_STORAGE_WRITE = 0x02
})

#[cfg(ANDROID)]
extern "C" {
    pub fn SDL_AndroidGetJNIEnv() -> *c_void;
    pub fn SDL_AndroidGetActivity() -> *c_void;
    pub fn SDL_AndroidGetInternalStoragePath() -> *c_char;
    pub fn SDL_AndroidGetExternalStorageState() -> SDL_ExternalStorage;
    pub fn SDL_AndroidGetExternalStoragePath() -> *c_char;
}
