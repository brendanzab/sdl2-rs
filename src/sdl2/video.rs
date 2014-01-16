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
use std::{str, ptr};
use std::libc::{c_int, c_void};
use rect;
use ffi::video::SDL_DisplayMode;

/**
 *  The flags on a window
 *
 *  SDL_GetWindowFlags()
 */
pub enum WindowFlags {
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

impl WindowFlags {
    pub fn fold_bits(flags: &[WindowFlags]) -> Uint32 {
        flags.iter().fold(0, |acc, &flag| acc | flag as Uint32)
    }

    pub fn unfold_bits(bitflags: u32) -> ~[WindowFlags]	{
	    let flags = [SDL_WINDOW_FULLSCREEN,
	        SDL_WINDOW_OPENGL,
	        SDL_WINDOW_SHOWN,
	        SDL_WINDOW_HIDDEN,
	        SDL_WINDOW_BORDERLESS,
	        SDL_WINDOW_RESIZABLE,
	        SDL_WINDOW_MINIMIZED,
	        SDL_WINDOW_MAXIMIZED,
	        SDL_WINDOW_INPUT_GRABBED,
	        SDL_WINDOW_INPUT_FOCUS,
	        SDL_WINDOW_MOUSE_FOCUS,
	        SDL_WINDOW_FULLSCREEN_DESKTOP,
	        SDL_WINDOW_FOREIGN
	    ];

	   flags.iter().filter_map(|&flag| {
	        if bitflags & (flag as u32) != 0 { Some(flag) }
	        else { None }
	    }).collect()
    }

}

/**
 *  Get the number of video drivers compiled into SDL
 *
 *  SDL_GetVideoDriver()
 */
pub fn SDL_GetNumVideoDrivers() -> int{
	unsafe	{
		ffi::video::SDL_GetNumVideoDrivers() as int
	}
}

/**
 *  Get the name of a built in video driver.
 *
 *  The video drivers are presented in the order in which they are
 *        normally checked during initialization.
 *
 *  SDL_GetNumVideoDrivers()
 */
pub fn SDL_GetVideoDriver(index: int) -> ~str {
	unsafe	{
		let drv = ffi::video::SDL_GetVideoDriver(index as c_int);
		str::raw::from_c_str(drv)
	}

}

/**
 *  Initialize the video subsystem, optionally specifying a video driver.
 *
 *  driver_name Initialize a specific driver by name, or NULL for the
 *                     default video driver.
 *
 *  return true on success, false on error
 *
 *  This function initializes the video subsystem; setting up a connection
 *  to the window manager, etc, and determines the available display modes
 *  and pixel formats, but does not initialize a window or graphics mode.
 *
 *  SDL_VideoQuit()
 */
pub fn SDL_VideoInit(driver_name: &str) -> bool	{
	unsafe	{
		ffi::video::SDL_VideoInit(driver_name.to_c_str().unwrap()) == 0
	}
}

/**
 *  Returns the name of the currently initialized video driver.
 *
 *  return The name of the current video driver or NULL if no driver
 *          has been initialized
 *
 *  SDL_GetNumVideoDrivers()
 *  SDL_GetVideoDriver()
 */
pub fn SDL_GetCurrentVideoDriver() -> ~str 	{
	unsafe	{
		let drv = ffi::video::SDL_GetCurrentVideoDriver();
		str::raw::from_c_str(drv)
	}
}

/**
 *  Returns the number of available video displays.
 *
 *  SDL_GetDisplayBounds()
 */
 pub fn SDL_GetNumVideoDisplays() -> int	{
	unsafe	{
		ffi::video::SDL_GetNumVideoDisplays() as int
	}
}
 
 /**
 *  Get the name of a display in UTF-8 encoding
 *
 *  return The name of a display, or NULL for an invalid display index.
 *
 *  SDL_GetNumVideoDisplays()
 */   
pub fn SDL_GetDisplayName(displayIndex: c_int) -> ~str	{
	unsafe	{
		let drv = ffi::video::SDL_GetDisplayName(displayIndex as c_int);
		str::raw::from_c_str(drv)
	}
}

/**
 *  Get the desktop area represented by a display, with the primary
 *         display located at 0,0
 *
 *  return 
 *
 *  \sa SDL_GetNumVideoDisplays()
 */
pub fn SDL_GetDisplayBounds(displayIndex: int) -> Option<rect::Rect>	{
	let mut rect = ffi::rect::SDL_Rect {x: 0,y: 0,w: 0,h: 0};
    let c_info = unsafe { ffi::video::SDL_GetDisplayBounds(displayIndex as c_int, &mut rect)};
    if c_info == 0 {
        Some(rect::Rect::wrap(&rect))
    }
    else {
        None
    }
}


pub struct DisplayMode {
    format: Uint32,                 // pixel format
    w: int,                       // width
    h: int,                       // height
    refresh_rate: int,            // refresh rate (or zero for unspecified)
    priv driverdata: *c_void,            // driver-specific data, initialize to 0
}

impl DisplayMode {

    #[doc(hidden)]
    pub fn wrap(mode : *SDL_DisplayMode) -> DisplayMode {
        unsafe {
            DisplayMode {
                format : (*mode).format,
                w : (*mode).w as int,
                h : (*mode).h as int,
                refresh_rate : (*mode).refresh_rate as int,
                driverdata: (*mode).driverdata
            }
        }
    }

    #[doc(hidden)]
    pub fn unwrap(&self) -> SDL_DisplayMode {
        SDL_DisplayMode {
            format : self.format,
            w : self.w as c_int,
            h : self.h as c_int,
            refresh_rate : self.refresh_rate as c_int,
            driverdata: self.driverdata
        }
    }
}


/**
 *  Returns the number of available display modes.
 *
 *  SDL_GetDisplayMode()
 */
pub fn SDL_GetNumDisplayModes(displayIndex: int) -> int	{
	unsafe	{
		ffi::video::SDL_GetNumDisplayModes(displayIndex as c_int) as int
	}	
}

/**
 *  Fill in information about a specific display mode.
 *
 *  note: The display modes are sorted in this priority:
 *        \li bits per pixel -> more colors to fewer colors
 *        \li width -> largest to smallest
 *        \li height -> largest to smallest
 *        \li refresh rate -> highest to lowest
 *
 *  SDL_GetNumDisplayModes()
 */
pub fn SDL_GetDisplayMode(displayIndex: int, modeIndex: int) -> Option<DisplayMode>	{
	let mut mode = SDL_DisplayMode {
            format : 0,
            w : 0,
            h : 0,
            refresh_rate : 0,
            driverdata: ptr::null(),
        };

    let c_info = unsafe { ffi::video::SDL_GetDisplayMode(displayIndex as c_int, modeIndex as c_int, &mut mode)};
    if c_info == 0 {
        Some(DisplayMode::wrap(&mode))
    }
    else {
        None
    }
}


/**
 *  Fill in information about the desktop display mode.
 */
pub fn SDL_GetDesktopDisplayMode(displayIndex: int) -> Option<DisplayMode>	{
	let mut mode = SDL_DisplayMode {
            format : 0,
            w : 0,
            h : 0,
            refresh_rate : 0,
            driverdata: ptr::null(),
        };

    let c_info = unsafe { ffi::video::SDL_GetDesktopDisplayMode(displayIndex as c_int, &mut mode)};
    if c_info == 0 {
        Some(DisplayMode::wrap(&mode))
    }
    else {
        None
    }
}

/**
 *  Fill in information about the current display mode.
 */
pub fn SDL_GetCurrentDisplayMode(displayIndex: int) -> Option<DisplayMode>	{
	let mut mode = SDL_DisplayMode {
            format : 0,
            w : 0,
            h : 0,
            refresh_rate : 0,
            driverdata: ptr::null(),
        };

    let c_info = unsafe { ffi::video::SDL_GetCurrentDisplayMode(displayIndex as c_int, &mut mode)};
    if c_info == 0 {
        Some(DisplayMode::wrap(&mode))
    }
    else {
        None
    }
}

/**
 *  Get the closest match to the requested display mode.
 *
 *  param: displayIndex The index of display from which mode should be queried.
 *  param: mode The desired display mode
 *  param: closest A pointer to a display mode to be filled in with the closest
 *                 match of the available display modes.
 *
 *  return The passed in value \c closest, or NULL if no matching video mode
 *          was available.
 *
 *  The available display modes are scanned, and \c closest is filled in with the
 *  closest mode matching the requested mode and returned.  The mode format and
 *  refresh_rate default to the desktop mode if they are 0.  The modes are
 *  scanned with size being first priority, format being second priority, and
 *  finally checking the refresh_rate.  If all the available modes are too
 *  small, then NULL is returned.
 *
 *  SDL_GetNumDisplayModes()
 *  SDL_GetDisplayMode()
 */   
pub fn SDL_GetClosestDisplayMode(displayIndex: int, mode: &SDL_DisplayMode) -> Option<DisplayMode>	{
	let mut closest = SDL_DisplayMode {
            format : 0,
            w : 0,
            h : 0,
            refresh_rate : 0,
            driverdata: ptr::null(),
        };

    let c_info = unsafe { ffi::video::SDL_GetClosestDisplayMode(displayIndex as c_int, mode, &mut closest)};
    if !c_info.is_null() {
        Some(DisplayMode::wrap(&closest))
    }
    else {
        None
    }
}




/**
 *  The type used to identify a window
 *
 *  \sa SDL_GetWindowData()
 *  \sa SDL_GetWindowGrab()
 *  \sa SDL_HideWindow()
 *  \sa SDL_MaximizeWindow()
 *  \sa SDL_MinimizeWindow()
 *  \sa SDL_RaiseWindow()
 *  \sa SDL_RestoreWindow()
 *  \sa SDL_SetWindowData()
 *  \sa SDL_SetWindowFullscreen()
 *  \sa SDL_SetWindowGrab()
 *  \sa SDL_SetWindowIcon()
 *  \sa SDL_SetWindowBordered()
 *  \sa SDL_ShowWindow()
 */
pub struct Window {
	priv window : *ffi::video::SDL_Window
}

impl Window{
	/**
 *  Create a window with the specified position, dimensions, and flags.
 *
 *  param title The title of the window, in UTF-8 encoding.
 *  param x     The x position of the window, ::SDL_WINDOWPOS_CENTERED, or
 *               ::SDL_WINDOWPOS_UNDEFINED.
 *  param y     The y position of the window, ::SDL_WINDOWPOS_CENTERED, or
 *               ::SDL_WINDOWPOS_UNDEFINED.
 *  param w     The width of the window.
 *  param h     The height of the window.
 *  param flags The flags for the window, a mask of any of the following:
 *               ::SDL_WINDOW_FULLSCREEN,    ::SDL_WINDOW_OPENGL,
 *               ::SDL_WINDOW_HIDDEN,        ::SDL_WINDOW_BORDERLESS,
 *               ::SDL_WINDOW_RESIZABLE,     ::SDL_WINDOW_MAXIMIZED,
 *               ::SDL_WINDOW_MINIMIZED,     ::SDL_WINDOW_INPUT_GRABBED,
 *               ::SDL_WINDOW_ALLOW_HIGHDPI.
 *
 *  return the window created.
 *
 */
	#[inline]
	pub fn create_window(title: &str, x: int, y: int, w: int, h: int, flags: &[WindowFlags]) -> Window {
	    let sdlwin = unsafe { ffi::video::SDL_CreateWindow(title.to_c_str().unwrap(), x as i32, y as i32, w as i32, h as i32, WindowFlags::fold_bits(flags))};
	  	Window {window: sdlwin}  	
	}

	/**
	 *  Create an SDL window from an existing native window.
	 *
	 *  param data A pointer to driver-dependent window creation data
	 *
	 *  return The window created.
	 *
	 */
	pub fn create_window_from(data: *c_void) -> Window {
	    let sdlwin = unsafe { ffi::video::SDL_CreateWindowFrom(data)};
	  	Window {window: sdlwin}  	
	}

	/**
	 *  Get a window from a stored ID, or NULL if it doesn't exist.
	 */
	pub fn get_window_from_id(id: u32) -> Window {
	    let sdlwin = unsafe { ffi::video::SDL_GetWindowFromID(id as Uint32)};
	  	Window {window: sdlwin}  	
	}



	/**
	 *  Get the display index associated with a window.
	 *
	 *  return the display index of the display containing the center of the
	 *          window, or None if an error occurs.
	 */
	pub fn get_window_display_index(&self) -> Option<int>	{
		let c_info = unsafe	{
			ffi::video::SDL_GetWindowDisplayIndex(self.window) as int
		};
	    if c_info != -1 {
	        Some(c_info)
	    }
	    else {
	        None
	    }
	}

	/**
	 *  Set the display mode used when a fullscreen window is visible.
	 *
	 *  By default the window's dimensions and the desktop format and refresh rate
	 *  are used.
	 *  param mode The mode to use, or NULL for the default mode.
	 *
	 *  return true on success, or false if setting the display mode failed.
	 *
	 */
	pub fn set_window_display_mode(&self, mode: &DisplayMode) -> bool	{
		unsafe	{
			ffi::video::SDL_SetWindowDisplayMode(self.window, &mode.unwrap()) == 0
		}
	}

	/**
	 *  brief Fill in information about the display mode used when a fullscreen
	 *         window is visible.
	 *
	 */
	pub fn get_window_display_mode(&self) -> Option<DisplayMode>	{
		let mut mode = SDL_DisplayMode {
	            format : 0,
	            w : 0,
	            h : 0,
	            refresh_rate : 0,
	            driverdata: ptr::null(),
	        };

	    let c_info = unsafe { ffi::video::SDL_GetWindowDisplayMode(self.window, &mut mode)};
	    if c_info == 0 {
	        Some(DisplayMode::wrap(&mode))
	    }
	    else {
	        None
	    }
    }

    /**
	 *  Get the numeric ID of a window, for logging purposes.
	 */
	pub fn Ggt_window_id(&self) -> u32	{
		unsafe	{
			ffi::video::SDL_GetWindowID(self.window) as u32
		}
	}

	/**
	 *  Get the window flags.
	 */
	pub fn get_window_flags(&self) -> ~[WindowFlags]	{
	    let c_flags = unsafe { ffi::video::SDL_GetWindowFlags(self.window)};
	  	WindowFlags::unfold_bits(c_flags as u32)	
	}

	/**
	 *  Set the title of a window, in UTF-8 format.
	 *
	 */
	pub fn set_window_title(&self, title: &str)	{
		unsafe	{
			ffi::video::SDL_SetWindowTitle(self.window, title.to_c_str().unwrap());
		}
	}

	/**
	 *  Get the title of a window, in UTF-8 format.
	 *
	 */	
    pub fn get_window_title(&self) -> ~str	{		
		unsafe	{
			str::raw::from_c_str(ffi::video::SDL_GetWindowTitle(self.window))
		}
    }

	/**
	 *  Set the position of a window.
	 *
	 *  param window   The window to reposition.
	 *  param x        The x coordinate of the window, ::SDL_WINDOWPOS_CENTERED, or
	                    ::SDL_WINDOWPOS_UNDEFINED.
	 *  param y        The y coordinate of the window, ::SDL_WINDOWPOS_CENTERED, or
	                    ::SDL_WINDOWPOS_UNDEFINED.
	 *
	 *  note The window coordinate origin is the upper left of the display.
	 *
	 */    
    pub fn set_window_position(&self, x: int, y: int)	{
		unsafe	{
			ffi::video::SDL_SetWindowPosition(self.window, x as c_int, y as c_int);
		}
    }

    /**
	 *  Get the position of a window.
	 *
	 *  return         x position,  y position
	 *
	 */
    pub fn get_window_position(&self) -> (int, int)	{
		let (mut x, mut y): (c_int, c_int) = (0,0);
		unsafe	{
			ffi::video::SDL_GetWindowPosition(self.window, &mut x, &mut y);
		}
		(x as int, y as int)
    }

    /**
	 *  Set the size of a window's client area.
	 *
	 *  param w        The width of the window, must be >0
	 *  param h        The height of the window, must be >0
	 *
	 *  note You can't change the size of a fullscreen window, it automatically
	 *        matches the size of the display mode.
	 *
	 */
    pub fn set_window_size(&self, w: int, h: int)	{
 		unsafe	{
			ffi::video::SDL_SetWindowSize(self.window, w as c_int, h as c_int);
		}   	
    }

    /**
	 *  Get the size of a window's client area.
	 *
	 *  param window   The window to query.
	 *  return         the width, the height
	 *
	 */
    pub fn get_window_size(&self) -> (int, int)	{
		let (mut w, mut h): (c_int, c_int) = (0,0);
		unsafe	{
			ffi::video::SDL_GetWindowSize(self.window, &mut w, &mut h);
		}
		(w as int, h as int)
    }

	/**
	 *  Set the minimum size of a window's client area.
	 *
	 *  param min_w     The minimum width of the window, must be >0
	 *  param min_h     The minimum height of the window, must be >0
	 *
	 *  note You can't change the minimum size of a fullscreen window, it
	 *        automatically matches the size of the display mode.
	 *
	 */
    pub fn set_window_minimum_size(&self, min_w: int, min_h: int)	{
 		unsafe	{
			ffi::video::SDL_SetWindowMinimumSize(self.window, min_w as c_int, min_h as c_int);
		}   	
    }

    /**
	 *  Get the minimum size of a window's client area.
	 *
	 *  return       the minimum width, the minimum height
	 *
	 */
    pub fn get_window_minimum_size(&self) -> (int, int) {
		let (mut min_w, mut min_h): (c_int, c_int) = (0,0);
		unsafe	{
			ffi::video::SDL_GetWindowMinimumSize(self.window, &mut min_w, &mut min_h);
		}
		(min_w as int, min_h as int)
    }

	/**
	 *  Set the maximum size of a window's client area.
	 *
	 *  param max_w     The maximum width of the window, must be >0
	 *  param max_h     The maximum height of the window, must be >0
	 *
	 *  note You can't change the maximum size of a fullscreen window, it
	 *        automatically matches the size of the display mode.
	 */
    pub fn set_window_maximum_size(&self, max_w: int, max_h: int)	{
 		unsafe	{
			ffi::video::SDL_SetWindowMaximumSize(self.window, max_w as c_int, max_h as c_int);
		}   	
    }

    /**
	 *  Get the maximum size of a window's client area.
	 *
	 *  param w        Pointer to variable for storing the maximum width, may be NULL
	 *  param h        Pointer to variable for storing the maximum height, may be NULL
	 *
	 */
    pub fn get_window_maximum_size(&self)  -> (int, int)	{
		let (mut max_w, mut max_h): (c_int, c_int) = (0,0);
		unsafe	{
			ffi::video::SDL_GetWindowMaximumSize(self.window, &mut max_w, &mut max_h);
		}
		(max_w as int, max_h as int)
    }

	/**
	 *  Set the border state of a window.
	 *
	 *  This will add or remove the window's SDL_WINDOW_BORDERLESS flag and
	 *  add or remove the border from the actual window. This is a no-op if the
	 *  window's border already matches the requested state.
	 *
	 *  param bordered false to remove border, true to add border.
	 *
	 *  note You can't change the border state of a fullscreen window.
	 *
	 */
    pub fn set_window_bordered(&self, bordered: bool)	{
    	if bordered	{
	 		unsafe	{
				ffi::video::SDL_SetWindowBordered(self.window, ffi::stdinc::SDL_TRUE);
			}   	
    	}
    	else	{
	 		unsafe	{
				ffi::video::SDL_SetWindowBordered(self.window, ffi::stdinc::SDL_FALSE);
			}   	
    	}
    }

	/**
	 *  Show a window.
	 *
	 */    
    pub fn show_window(&self)	{
 		unsafe	{
			ffi::video::SDL_ShowWindow(self.window);
		}   	
    }

	/**
	 *  Hide a window.
	 *
	 */    
    pub fn hide_window(&self)	{
 		unsafe	{
			ffi::video::SDL_HideWindow(self.window);
		}   	
    }

    /**
	 *  Raise a window above other windows and set the input focus.
	 */
    pub fn raise_window(&self)	{
 		unsafe	{
			ffi::video::SDL_RaiseWindow(self.window);
		}   	
    }

	/**
	 *  Make a window as large as possible.
	 *
	 */    
    pub fn maximize_window(&self)	{
 		unsafe	{
			ffi::video::SDL_MaximizeWindow(self.window);
		}   	
    }

	/**
	 *  Minimize a window to an iconic representation.
	 *
	 */    
    pub fn minimize_window(&self)	{
 		unsafe	{
			ffi::video::SDL_MinimizeWindow(self.window);
		}   	
    }

	/**
	 *  Restore the size and position of a minimized or maximized window.
	 *
	 */    
    pub fn restore_window(&self)	{
 		unsafe	{
			ffi::video::SDL_RestoreWindow(self.window);
		}   	
    }
}

impl Drop for Window
{
    fn drop(&mut self)
    {
            unsafe
            {
            	ffi::video::SDL_DestroyWindow(self.window);
            }
    }
}
