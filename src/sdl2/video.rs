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

use ffi;
use ffi::stdinc::Uint32;

pub enum SDL_WindowFlags {
    SDL_WINDOW_FULLSCREEN           = ffi::video::SDL_WINDOW_FULLSCREEN,
    SDL_WINDOW_OPENGL               = ffi::video::SDL_WINDOW_OPENGL,
    SDL_WINDOW_SHOWN                = ffi::video::SDL_WINDOW_SHOWN,
    SDL_WINDOW_HIDDEN               = ffi::video::SDL_WINDOW_HIDDEN,
    SDL_WINDOW_BORDERLESS           = ffi::video::SDL_WINDOW_BORDERLESS,
    SDL_WINDOW_RESIZABLE            = ffi::video::SDL_WINDOW_RESIZABLE,
    SDL_WINDOW_MINIMIZED            = ffi::video::SDL_WINDOW_MINIMIZED,
    SDL_WINDOW_MAXIMIZED            = ffi::video::SDL_WINDOW_MAXIMIZED,
    SDL_WINDOW_INPUT_GRABBED        = ffi::video::SDL_WINDOW_INPUT_GRABBED,
    SDL_WINDOW_INPUT_FOCUS          = ffi::video::SDL_WINDOW_INPUT_FOCUS,
    SDL_WINDOW_MOUSE_FOCUS          = ffi::video::SDL_WINDOW_MOUSE_FOCUS,
    SDL_WINDOW_FULLSCREEN_DESKTOP   = ffi::video::SDL_WINDOW_FULLSCREEN_DESKTOP,
    SDL_WINDOW_FOREIGN              = ffi::video::SDL_WINDOW_FOREIGN,
}

impl SDL_WindowFlags {
    pub fn fold_bits(flags: &[SDL_WindowFlags]) -> Uint32 {
        flags.iter().fold(0, |acc, &flag| acc | flag as Uint32)
    }
}

pub struct SDL_Window {
	priv window : *ffi::video::SDL_Window
}

impl SDL_Window{
	#[inline]
	pub fn initWindow(title: &str, x: int, y: int, w: int, h: int, flags: &[SDL_WindowFlags]) -> SDL_Window {
	    let sdlwin = unsafe { ffi::video::SDL_CreateWindow(title.to_c_str().unwrap(), x as i32, y as i32, w as i32, h as i32, SDL_WindowFlags::fold_bits(flags))};
	  	SDL_Window {window: sdlwin}  	
	}
}
