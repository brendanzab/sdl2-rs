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

use ffi::keyboard::SDL_Keysym;
use ffi::stdinc::{SDL_bool, Uint8, Uint16, Sint32, Uint32};
use ffi::touch::{SDL_TouchID, SDL_FingerID};
use std::cast::transmute;
use std::libc::{c_char, c_float, c_int, c_void};

// SDL_events.h

pub static SDL_RELEASED:    Uint8 = 0;
pub static SDL_PRESSED:     Uint8 = 1;

pub type SDL_EventType = Uint32;
pub static SDL_FIRSTEVENT:                  SDL_EventType = 0;
pub static SDL_QUIT:                        SDL_EventType = 0x100;
pub static SDL_APP_TERMINATING:             SDL_EventType = 0x101;
pub static SDL_APP_LOWMEMORY:               SDL_EventType = 0x102;
pub static SDL_APP_WILLENTERBACKGROUND:     SDL_EventType = 0x103;
pub static SDL_APP_DIDENTERBACKGROUND:      SDL_EventType = 0x104;
pub static SDL_APP_WILLENTERFOREGROUND:     SDL_EventType = 0x105;
pub static SDL_APP_DIDENTERFOREGROUND:      SDL_EventType = 0x106;
pub static SDL_WINDOWEVENT:                 SDL_EventType = 0x200;
pub static SDL_KEYDOWN:                     SDL_EventType = 0x300;
pub static SDL_KEYUP:                       SDL_EventType = 0x301;
pub static SDL_TEXTEDITING:                 SDL_EventType = 0x302;
pub static SDL_TEXTINPUT:                   SDL_EventType = 0x303;
pub static SDL_MOUSEMOTION:                 SDL_EventType = 0x400;
pub static SDL_MOUSEBUTTONDOWN:             SDL_EventType = 0x401;
pub static SDL_MOUSEBUTTONUP:               SDL_EventType = 0x402;
pub static SDL_MOUSEWHEEL:                  SDL_EventType = 0x403;
pub static SDL_JOYAXISMOTION:               SDL_EventType = 0x600;
pub static SDL_JOYBALLMOTION:               SDL_EventType = 0x601;
pub static SDL_JOYHATMOTION:                SDL_EventType = 0x602;
pub static SDL_JOYBUTTONDOWN:               SDL_EventType = 0x603;
pub static SDL_JOYBUTTONUP:                 SDL_EventType = 0x604;
pub static SDL_JOYDEVICEADDED:              SDL_EventType = 0x605;
pub static SDL_JOYDEVICEREMOVED:            SDL_EventType = 0x606;
pub static SDL_CONTROLLERAXISMOTION:        SDL_EventType = 0x650;
pub static SDL_CONTROLLERBUTTONDOWN:        SDL_EventType = 0x651;
pub static SDL_CONTROLLERBUTTONUP:          SDL_EventType = 0x652;
pub static SDL_CONTROLLERDEVICEADDED:       SDL_EventType = 0x653;
pub static SDL_CONTROLLERDEVICEREMOVED:     SDL_EventType = 0x654;
pub static SDL_CONTROLLERDEVICEREMAPPED:    SDL_EventType = 0x655;
pub static SDL_FINGERDOWN:                  SDL_EventType = 0x700;
pub static SDL_FINGERUP:                    SDL_EventType = 0x701;
pub static SDL_FINGERMOTION:                SDL_EventType = 0x702;
pub static SDL_DOLLARGESTURE:               SDL_EventType = 0x800;
pub static SDL_DOLLARRECORD:                SDL_EventType = 0x801;
pub static SDL_MULTIGESTURE:                SDL_EventType = 0x802;
pub static SDL_CLIPBOARDUPDATE:             SDL_EventType = 0x900;
pub static SDL_DROPFILE:                    SDL_EventType = 0x1000;
pub static SDL_USEREVENT:                   SDL_EventType = 0x8000;
pub static SDL_LASTEVENT:                   SDL_EventType = 0xFFFF;

pub struct SDL_CommonEvent {
    event_type: Uint32,
    timestamp:  Uint32,
}

pub struct SDL_WindowEvent {
    event_type: Uint32,
    timestamp:  Uint32,
    windowID:   Uint32,
    event:      Uint8,
    padding1:   Uint8,
    padding2:   Uint8,
    padding3:   Uint8,
    data1:      Sint32,
    data2:      Sint32,
}

pub struct SDL_KeyboardEvent {
    event_type: Uint32,
    timestamp:  Uint32,
    windowID:   Uint32,
    state:      Uint8,
    repeat:     Uint8,
    padding2:   Uint8,
    padding3:   Uint8,
    keysym:     SDL_Keysym,
}

pub static SDL_TEXTEDITINGEVENT_TEXT_SIZE: uint = 32;

pub struct SDL_TextEditingEvent {
    event_type: Uint32,
    timestamp:  Uint32,
    windowID:   Uint32,
    text:       [c_char, ..SDL_TEXTEDITINGEVENT_TEXT_SIZE],
    start:      Sint32,
    length:     Sint32,
}

pub static SDL_TEXTINPUTEVENT_TEXT_SIZE: uint = 32;

pub struct SDL_TextInputEvent {
    event_type: Uint32,
    timestamp:  Uint32,
    windowID:   Uint32,
    text:       [c_char, ..SDL_TEXTINPUTEVENT_TEXT_SIZE],
}

pub struct SDL_MouseMotionEvent {
    event_type: Uint32,
    timestamp:  Uint32,
    windowID:   Uint32,
    which:      Uint32,
    state:      Uint32,
    x:          Sint32,
    y:          Sint32,
    xrel:       Sint32,
    yrel:       Sint32,
}

pub struct SDL_MouseButtonEvent {
    event_type: Uint32,
    timestamp:  Uint32,
    windowID:   Uint32,
    which:      Uint32,
    button:     Uint8,
    state:      Uint8,
    padding1:   Uint8,
    padding2:   Uint8,
    x:          Sint32,
    y:          Sint32,
}

pub struct SDL_MouseWheelEvent {
    event_type: Uint32,
    timestamp:  Uint32,
    windowID:   Uint32,
    which:      Uint32,
    x:          Sint32,
    y:          Sint32,
}

// pub struct SDL_JoyAxisEvent {
//     event_type: Uint32,
//     timestamp:  Uint32,
//     which:      SDL_JoystickID,
//     axis:       Uint8,
//     padding1:   Uint8,
//     padding2:   Uint8,
//     padding3:   Uint8,
//     value:      Sint16,
//     padding4:   Uint16,
// }

// pub struct SDL_JoyBallEvent {
//     event_type: Uint32,
//     timestamp:  Uint32,
//     which:      SDL_JoystickID,
//     ball:       Uint8,
//     padding1:   Uint8,
//     padding2:   Uint8,
//     padding3:   Uint8,
//     xrel:       Sint16,
//     yrel:       Sint16,
// }

// pub struct SDL_JoyHatEvent {
//     event_type: Uint32,
//     timestamp:  Uint32,
//     which:      SDL_JoystickID,
//     hat:        Uint8,
//     value:      Uint8,
//     padding1:   Uint8,
//     padding2:   Uint8,
// }

// pub struct SDL_JoyButtonEvent {
//     event_type: Uint32,
//     timestamp:  Uint32,
//     which:      SDL_JoystickID,
//     button:     Uint8,
//     state:      Uint8,
//     padding1:   Uint8,
//     padding2:   Uint8,
// }

// pub struct SDL_JoyDeviceEvent {
//     event_type: Uint32,
//     timestamp:  Uint32,
//     which:      Sint32,
// }

// pub struct SDL_ControllerAxisEvent {
//     event_type: Uint32,
//     timestamp:  Uint32,
//     which:      SDL_JoystickID,
//     axis:       Uint8,
//     padding1:   Uint8,
//     padding2:   Uint8,
//     padding3:   Uint8,
//     value:      Sint16,
//     padding4:   Uint16,
// }

// pub struct SDL_ControllerButtonEvent {
//     event_type: Uint32,
//     timestamp:  Uint32,
//     which:      SDL_JoystickID,
//     button:     Uint8,
//     state:      Uint8,
//     padding1:   Uint8,
//     padding2:   Uint8,
// }

pub struct SDL_ControllerDeviceEvent {
    event_type: Uint32,
    timestamp:  Uint32,
    which:      Sint32,
}

pub struct SDL_TouchFingerEvent {
    event_type: Uint32,
    timestamp:  Uint32,
    touchId:    SDL_TouchID,
    fingerId:   SDL_FingerID,
    x:          c_float,
    y:          c_float,
    dx:         c_float,
    dy:         c_float,
    pressure:   c_float,
}

pub struct SDL_MultiGestureEvent {
    event_type: Uint32,
    timestamp:  Uint32,
    touchId:    SDL_TouchID,
    dTheta:     c_float,
    dDist:      c_float,
    x:          c_float,
    y:          c_float,
    numFingers: Uint16,
    padding:    Uint16,
}

// pub struct SDL_DollarGestureEvent {
//     event_type: Uint32,
//     timestamp:  Uint32,
//     touchId:    SDL_TouchID,
//     gestureId:  SDL_GestureID,
//     numFingers: Uint32,
//     error:      c_float,
//     x:          c_float,
//     y:          c_float,
// }

pub struct SDL_DropEvent {
    event_type: Uint32,
    timestamp:  Uint32,
    file:       *c_char,
}

pub struct SDL_QuitEvent {
    event_type: Uint32,
    timestamp:  Uint32,
}

pub struct SDL_OSEvent {
    event_type: Uint32,
    timestamp:  Uint32,
}

pub struct SDL_UserEvent {
    event_type: Uint32,
    timestamp:  Uint32,
    windowID:   Uint32,
    code:       Sint32,
    data1:      *c_void,
    data2:      *c_void,
}

pub enum SDL_SysWMmsg {}

pub struct SDL_SysWMEvent {
    event_type: Uint32,
    timestamp:  Uint32,
    msg:        *SDL_SysWMmsg,
}

pub struct SDL_Event {
    priv data: [Uint8, ..56],
}

impl SDL_Event {
    pub unsafe fn as_common<'a>(&'a self)      -> &'a SDL_CommonEvent            { transmute(&'a self.data) }
    pub unsafe fn as_window<'a>(&'a self)      -> &'a SDL_WindowEvent            { transmute(&'a self.data) }
    pub unsafe fn as_key<'a>(&'a self)         -> &'a SDL_KeyboardEvent          { transmute(&'a self.data) }
    pub unsafe fn as_edit<'a>(&'a self)        -> &'a SDL_TextEditingEvent       { transmute(&'a self.data) }
    pub unsafe fn as_text<'a>(&'a self)        -> &'a SDL_TextInputEvent         { transmute(&'a self.data) }
    pub unsafe fn as_motion<'a>(&'a self)      -> &'a SDL_MouseMotionEvent       { transmute(&'a self.data) }
    pub unsafe fn as_button<'a>(&'a self)      -> &'a SDL_MouseButtonEvent       { transmute(&'a self.data) }
    pub unsafe fn as_wheel<'a>(&'a self)       -> &'a SDL_MouseWheelEvent        { transmute(&'a self.data) }
 // pub unsafe fn as_jaxis<'a>(&'a self)       -> &'a SDL_JoyAxisEvent           { transmute(&'a self.data) }
 // pub unsafe fn as_jball<'a>(&'a self)       -> &'a SDL_JoyBallEvent           { transmute(&'a self.data) }
 // pub unsafe fn as_jhat<'a>(&'a self)        -> &'a SDL_JoyHatEvent            { transmute(&'a self.data) }
 // pub unsafe fn as_jbutton<'a>(&'a self)     -> &'a SDL_JoyButtonEvent         { transmute(&'a self.data) }
 // pub unsafe fn as_jdevice<'a>(&'a self)     -> &'a SDL_JoyDeviceEvent         { transmute(&'a self.data) }
 // pub unsafe fn as_caxis<'a>(&'a self)       -> &'a SDL_ControllerAxisEvent    { transmute(&'a self.data) }
 // pub unsafe fn as_cbutton<'a>(&'a self)     -> &'a SDL_ControllerButtonEvent  { transmute(&'a self.data) }
 // pub unsafe fn as_cdevice<'a>(&'a self)     -> &'a SDL_ControllerDeviceEvent  { transmute(&'a self.data) }
    pub unsafe fn as_quit<'a>(&'a self)        -> &'a SDL_QuitEvent              { transmute(&'a self.data) }
    pub unsafe fn as_user<'a>(&'a self)        -> &'a SDL_UserEvent              { transmute(&'a self.data) }
    pub unsafe fn as_syswm<'a>(&'a self)       -> &'a SDL_SysWMEvent             { transmute(&'a self.data) }
    pub unsafe fn as_tfinger<'a>(&'a self)     -> &'a SDL_TouchFingerEvent       { transmute(&'a self.data) }
    pub unsafe fn as_mgesture<'a>(&'a self)    -> &'a SDL_MultiGestureEvent      { transmute(&'a self.data) }
 // pub unsafe fn as_dgesture<'a>(&'a self)    -> &'a SDL_DollarGestureEvent     { transmute(&'a self.data) }
    pub unsafe fn as_drop<'a>(&'a self)        -> &'a SDL_DropEvent              { transmute(&'a self.data) }
}

#[repr(C)]
pub enum SDL_eventaction {
    SDL_ADDEVENT,
    SDL_PEEKEVENT,
    SDL_GETEVENT,
}

pub type SDL_EventFilter = extern "C" fn(userdata: *c_void, event: *SDL_Event) -> c_int;

pub static SDL_QUERY:   c_int = -1;
pub static SDL_IGNORE:  c_int =  0;
pub static SDL_DISABLE: c_int =  0;
pub static SDL_ENABLE:  c_int =  1;

#[inline]
pub unsafe fn SDL_GetEventState(event_type: Uint32) -> Uint8 {
    SDL_EventState(event_type, SDL_QUERY)
}

extern "C" {
    pub fn SDL_PumpEvents();
    pub fn SDL_PeepEvents(events: *SDL_Event, numevents: c_int, action: SDL_eventaction, minType: Uint32, maxType: Uint32) -> c_int;
    pub fn SDL_HasEvent(event_type: Uint32) -> SDL_bool;
    pub fn SDL_HasEvents(minType: Uint32, maxType: Uint32) -> SDL_bool;
    pub fn SDL_FlushEvent(event_type: Uint32);
    pub fn SDL_FlushEvents(minType: Uint32, maxType: Uint32);
    pub fn SDL_PollEvent(event: *SDL_Event) -> c_int;
    pub fn SDL_WaitEvent(event: *SDL_Event) -> c_int;
    pub fn SDL_WaitEventTimeout(event: *SDL_Event, timeout: c_int) -> c_int;
    pub fn SDL_PushEvent(event: *SDL_Event) -> c_int;
    pub fn SDL_SetEventFilter(filter: SDL_EventFilter, userdata: *c_void);
    pub fn SDL_GetEventFilter(filter: *SDL_EventFilter, userdata: **c_void) -> SDL_bool;
    pub fn SDL_AddEventWatch(filter: SDL_EventFilter, userdata: *c_void);
    pub fn SDL_DelEventWatch(filter: SDL_EventFilter, userdata: *c_void);
    pub fn SDL_FilterEvents(filter: SDL_EventFilter, userdata: *c_void);
    pub fn SDL_EventState(event_type: Uint32, state: c_int) -> Uint8;
    pub fn SDL_RegisterEvents(numevents: c_int) -> Uint32;
}
