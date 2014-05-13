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

//! CPU feature detection.

use ffi;

/// Returns the number of available CPU cores.
#[inline]
pub fn get_cpu_count() -> uint {
    unsafe { ffi::cpuinfo::SDL_GetCPUCount() as uint }
}

/// Returns the L1 cache size of the CPU.
#[inline]
pub fn get_cpu_cache_line_size() -> uint {
    unsafe { ffi::cpuinfo::SDL_GetCPUCacheLineSize() as uint }
}

/// Returns `true` if the CPU has the RDTSC instruction.
#[inline]
pub fn has_rdtsc() -> bool {
    unsafe { ffi::cpuinfo::SDL_HasRDTSC().to_bool() }
}

/// Returns `true` if the CPU has AltiVec features.
#[inline]
pub fn has_altivec() -> bool {
    unsafe { ffi::cpuinfo::SDL_HasAltiVec().to_bool() }
}

/// Returns `true` if the CPU has MMX features.
#[inline]
pub fn has_mmx() -> bool {
    unsafe { ffi::cpuinfo::SDL_HasMMX().to_bool() }
}

/// Returns `true` if the CPU has 3DNow! features.
#[inline]
pub fn has_3dnow() -> bool {
    unsafe { ffi::cpuinfo::SDL_Has3DNow().to_bool() }
}

/// Returns `true` if the CPU has SSE features.
#[inline]
pub fn has_sse() -> bool {
    unsafe { ffi::cpuinfo::SDL_HasSSE().to_bool() }
}

/// Returns `true` if the CPU has SSE2 features.
#[inline]
pub fn has_sse2() -> bool {
    unsafe { ffi::cpuinfo::SDL_HasSSE2().to_bool() }
}

/// Returns `true` if the CPU has SSE3 features.
#[inline]
pub fn has_sse3() -> bool {
    unsafe { ffi::cpuinfo::SDL_HasSSE3().to_bool() }
}

/// Returns `true` if the CPU has SSE4.1 features.
#[inline]
pub fn has_sse41() -> bool {
    unsafe { ffi::cpuinfo::SDL_HasSSE41().to_bool() }
}

/// Returns `true` if the CPU has SSE4.2 features.
#[inline]
pub fn has_sse42() -> bool {
    unsafe { ffi::cpuinfo::SDL_HasSSE42().to_bool() }
}
