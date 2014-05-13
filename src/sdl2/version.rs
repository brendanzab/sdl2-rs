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

extern crate semver;
extern crate libc;

use ffi;

use libc::c_int;
use std::str::raw::from_c_str;

/// Returns the version of the underlying SDL library.
pub fn get() -> semver::Version {
    let mut version = ffi::version::SDL_version {
        major: 0, minor: 0, patch: 0,
    };
    unsafe { ffi::version::SDL_GetVersion(&mut version) }
    semver::Version {
        major: version.major as uint,
        minor: version.minor as uint,
        patch: version.patch as uint,
        pre:   vec![],
        build: vec![],
    }
}

/// Returns a hash that uniquely identifies the code revision of the underlying
/// SDL library. It is NOT an incrementing number.
pub fn get_revision() -> ~str {
    unsafe { from_c_str(ffi::version::SDL_GetRevision()) }
}

/// Gets a unique revision number of the underlying SDL library. This is an
/// incrementing number that is based on commits to hg.libsdl.org.
pub fn get_revision_number() -> c_int {
    unsafe { ffi::version::SDL_GetRevisionNumber() }
}
