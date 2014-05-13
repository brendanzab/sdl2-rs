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

use ffi::gesture::SDL_GestureID;
use ffi::joystick::SDL_JoystickID;
use ffi::keyboard::SDL_Keysym;
use ffi::stdinc::{SDL_bool, Sint16, Sint32, Uint8, Uint16, Uint32};
use ffi::touch::{SDL_TouchID, SDL_FingerID};

use std::cast::transmute;
use libc::{c_char, c_float, c_int, c_void};

// SDL_events.h

#[repr(u8)]
pub enum SDL_ButtonState {
    SDL_RELEASED = 0,
    SDL_PRESSED  = 1,
}

bitflags!(flags SDL_EventType: Uint32 {
    // Unused
    static SDL_FIRSTEVENT                   = 0,

    // Application events
    static SDL_QUIT                         = 0x100,

    // Special events for iOS. These should be handled in an `SDL_EventFilter`
    // callback set using the `SDL_SetEventFilter` function because iOS might
    // not allow for enough processing time after the events are delivered.
    static SDL_APP_TERMINATING              = 0x101,
    static SDL_APP_LOWMEMORY                = 0x102,
    static SDL_APP_WILLENTERBACKGROUND      = 0x103,
    static SDL_APP_DIDENTERBACKGROUND       = 0x104,
    static SDL_APP_WILLENTERFOREGROUND      = 0x105,
    static SDL_APP_DIDENTERFOREGROUND       = 0x106,

    // Window events
    static SDL_WINDOWEVENT                  = 0x200,
    static SDL_SYSWMEVENT                   = 0x201,

    // Keyboard events
    static SDL_KEYDOWN                      = 0x300,
    static SDL_KEYUP                        = 0x301,
    static SDL_TEXTEDITING                  = 0x302,
    static SDL_TEXTINPUT                    = 0x303,

    // Mouse events
    static SDL_MOUSEMOTION                  = 0x400,
    static SDL_MOUSEBUTTONDOWN              = 0x401,
    static SDL_MOUSEBUTTONUP                = 0x402,
    static SDL_MOUSEWHEEL                   = 0x403,

    // Joystick events
    static SDL_JOYAXISMOTION                = 0x600,
    static SDL_JOYBALLMOTION                = 0x601,
    static SDL_JOYHATMOTION                 = 0x602,
    static SDL_JOYBUTTONDOWN                = 0x603,
    static SDL_JOYBUTTONUP                  = 0x604,
    static SDL_JOYDEVICEADDED               = 0x605,
    static SDL_JOYDEVICEREMOVED             = 0x606,

    // Game controller events
    static SDL_CONTROLLERAXISMOTION         = 0x650,
    static SDL_CONTROLLERBUTTONDOWN         = 0x651,
    static SDL_CONTROLLERBUTTONUP           = 0x652,
    static SDL_CONTROLLERDEVICEADDED        = 0x653,
    static SDL_CONTROLLERDEVICEREMOVED      = 0x654,
    static SDL_CONTROLLERDEVICEREMAPPED     = 0x655,

    // Touch events
    static SDL_FINGERDOWN                   = 0x700,
    static SDL_FINGERUP                     = 0x701,
    static SDL_FINGERMOTION                 = 0x702,

    // Gesture events
    static SDL_DOLLARGESTURE                = 0x800,
    static SDL_DOLLARRECORD                 = 0x801,
    static SDL_MULTIGESTURE                 = 0x802,

    // Clipboard events
    static SDL_CLIPBOARDUPDATE              = 0x900,

    // Drag and drop events
    static SDL_DROPFILE                     = 0x1000,

    // User defined events start here
    static SDL_USEREVENT                    = 0x8000,

    // The last user event is at `SDL_LASTEVENT.bits - 1`
    static SDL_LASTEVENT                    = 0xFFFF
})

impl SDL_EventType {
    /// Create a event type flag representing a user event. The supplied bits
    /// must be in the range of `SDL_USEREVENT.bits(0x8000) <= bits <
    /// SDL_LASTEVENT.bits(0xFFFF)`.
    pub fn new_user_event(bits: Uint32) -> SDL_EventType {
        let event_type = SDL_EventType { bits: bits };
        assert!(event_type.is_user(), "The supplied bits ({:X}) are not in the range of \
                                       static SDL_USEREVENT.bits({:X}) <= bits < SDL_LASTEVENT.bits({:X}).",
                                       bits, SDL_USEREVENT.bits, SDL_LASTEVENT.bits);
        event_type
    }
    pub fn is_window(&self)     -> bool { *self == SDL_WINDOWEVENT                                                                                              }
    pub fn is_key(&self)        -> bool { *self == SDL_KEYDOWN || *self == SDL_KEYUP                                                                            }
    pub fn is_edit(&self)       -> bool { *self == SDL_TEXTEDITING                                                                                              }
    pub fn is_text(&self)       -> bool { *self == SDL_TEXTINPUT                                                                                                }
    pub fn is_motion(&self)     -> bool { *self == SDL_MOUSEMOTION                                                                                              }
    pub fn is_button(&self)     -> bool { *self == SDL_MOUSEBUTTONDOWN || *self == SDL_MOUSEBUTTONUP                                                            }
    pub fn is_wheel(&self)      -> bool { *self == SDL_MOUSEWHEEL                                                                                               }
    pub fn is_jaxis(&self)      -> bool { *self == SDL_JOYAXISMOTION                                                                                            }
    pub fn is_jball(&self)      -> bool { *self == SDL_JOYBALLMOTION                                                                                            }
    pub fn is_jhat(&self)       -> bool { *self == SDL_JOYHATMOTION                                                                                             }
    pub fn is_jbutton(&self)    -> bool { *self == SDL_JOYBUTTONDOWN || *self == SDL_JOYBUTTONUP                                                                }
    pub fn is_jdevice(&self)    -> bool { *self == SDL_JOYDEVICEADDED || *self == SDL_JOYDEVICEREMOVED                                                          }
    pub fn is_caxis(&self)      -> bool { *self == SDL_CONTROLLERAXISMOTION                                                                                     }
    pub fn is_cbutton(&self)    -> bool { *self == SDL_CONTROLLERBUTTONDOWN || *self == SDL_CONTROLLERBUTTONUP                                                  }
    pub fn is_cdevice(&self)    -> bool { *self == SDL_CONTROLLERDEVICEADDED || *self == SDL_CONTROLLERDEVICEREMOVED || *self == SDL_CONTROLLERDEVICEREMAPPED   }
    pub fn is_quit(&self)       -> bool { *self == SDL_QUIT                                                                                                     }
    pub fn is_user(&self)       -> bool { SDL_USEREVENT.bits <= self.bits && self.bits < SDL_LASTEVENT.bits                                                     }
    pub fn is_syswm(&self)      -> bool { *self == SDL_SYSWMEVENT                                                                                               }
    pub fn is_tfinger(&self)    -> bool { *self == SDL_FINGERMOTION || *self == SDL_FINGERDOWN || *self == SDL_FINGERUP                                         }
    pub fn is_mgesture(&self)   -> bool { *self == SDL_MULTIGESTURE                                                                                             }
    pub fn is_dgesture(&self)   -> bool { *self == SDL_DOLLARGESTURE                                                                                            }
    pub fn is_drop_(&self)      -> bool { *self == SDL_DROPFILE                                                                                                 }
}

/// A generic SDL event. We use methods to emulate the behaviour of the union
/// defined in `SDL_events.h`. The `event_type` and `timestamp` fields are
/// common to all data members of the said union, so they are included in this
/// common struct. This means we must reduce the size of the padding to suit.
pub struct SDL_Event {
    /// The event type.
    pub event_type: SDL_EventType,
    /// The timestamp of when the event was triggered.
    pub timestamp:  Uint32,
    /// Some padding that is added to ensure that we to match the size of the
    /// `SDL_Event` union defined in `SDL_events.h`. We want the size of this
    /// field to be equal to be the padding member defined in the union, minus
    /// the size of the event type and timestamp fields, ie.
    /// `sizeof(Uint8[56]) - (sizeof(Uint32) + sizeof(Uint32))`.
    padding: [Uint8, ..(56 * 8) - (32/8 + 32/8)],
}

impl SDL_Event {
    // These event getters check the `event_type` first, meaning they are slower
    // than the unsafe casts, however they should be safe if SDL behaves. :)
    pub fn get_window<'a>(&'a self)         -> Option<&'a SDL_WindowEvent>           { if self.event_type.is_window()   { Some(unsafe { self.window()   }) } else { None } }
    pub fn get_key<'a>(&'a self)            -> Option<&'a SDL_KeyboardEvent>         { if self.event_type.is_key()      { Some(unsafe { self.key()      }) } else { None } }
    pub fn get_edit<'a>(&'a self)           -> Option<&'a SDL_TextEditingEvent>      { if self.event_type.is_edit()     { Some(unsafe { self.edit()     }) } else { None } }
    pub fn get_text<'a>(&'a self)           -> Option<&'a SDL_TextInputEvent>        { if self.event_type.is_text()     { Some(unsafe { self.text()     }) } else { None } }
    pub fn get_motion<'a>(&'a self)         -> Option<&'a SDL_MouseMotionEvent>      { if self.event_type.is_motion()   { Some(unsafe { self.motion()   }) } else { None } }
    pub fn get_button<'a>(&'a self)         -> Option<&'a SDL_MouseButtonEvent>      { if self.event_type.is_button()   { Some(unsafe { self.button()   }) } else { None } }
    pub fn get_wheel<'a>(&'a self)          -> Option<&'a SDL_MouseWheelEvent>       { if self.event_type.is_wheel()    { Some(unsafe { self.wheel()    }) } else { None } }
    pub fn get_jaxis<'a>(&'a self)          -> Option<&'a SDL_JoyAxisEvent>          { if self.event_type.is_jaxis()    { Some(unsafe { self.jaxis()    }) } else { None } }
    pub fn get_jball<'a>(&'a self)          -> Option<&'a SDL_JoyBallEvent>          { if self.event_type.is_jball()    { Some(unsafe { self.jball()    }) } else { None } }
    pub fn get_jhat<'a>(&'a self)           -> Option<&'a SDL_JoyHatEvent>           { if self.event_type.is_jhat()     { Some(unsafe { self.jhat()     }) } else { None } }
    pub fn get_jbutton<'a>(&'a self)        -> Option<&'a SDL_JoyButtonEvent>        { if self.event_type.is_jbutton()  { Some(unsafe { self.jbutton()  }) } else { None } }
    pub fn get_jdevice<'a>(&'a self)        -> Option<&'a SDL_JoyDeviceEvent>        { if self.event_type.is_jdevice()  { Some(unsafe { self.jdevice()  }) } else { None } }
    pub fn get_caxis<'a>(&'a self)          -> Option<&'a SDL_ControllerAxisEvent>   { if self.event_type.is_caxis()    { Some(unsafe { self.caxis()    }) } else { None } }
    pub fn get_cbutton<'a>(&'a self)        -> Option<&'a SDL_ControllerButtonEvent> { if self.event_type.is_cbutton()  { Some(unsafe { self.cbutton()  }) } else { None } }
    pub fn get_cdevice<'a>(&'a self)        -> Option<&'a SDL_ControllerDeviceEvent> { if self.event_type.is_cdevice()  { Some(unsafe { self.cdevice()  }) } else { None } }
    pub fn get_quit<'a>(&'a self)           -> Option<&'a SDL_QuitEvent>             { if self.event_type.is_quit()     { Some(unsafe { self.quit()     }) } else { None } }
    pub fn get_user<'a>(&'a self)           -> Option<&'a SDL_UserEvent>             { if self.event_type.is_user()     { Some(unsafe { self.user()     }) } else { None } }
    pub fn get_syswm<'a>(&'a self)          -> Option<&'a SDL_SysWMEvent>            { if self.event_type.is_syswm()    { Some(unsafe { self.syswm()    }) } else { None } }
    pub fn get_tfinger<'a>(&'a self)        -> Option<&'a SDL_TouchFingerEvent>      { if self.event_type.is_tfinger()  { Some(unsafe { self.tfinger()  }) } else { None } }
    pub fn get_mgesture<'a>(&'a self)       -> Option<&'a SDL_MultiGestureEvent>     { if self.event_type.is_mgesture() { Some(unsafe { self.mgesture() }) } else { None } }
    pub fn get_dgesture<'a>(&'a self)       -> Option<&'a SDL_DollarGestureEvent>    { if self.event_type.is_dgesture() { Some(unsafe { self.dgesture() }) } else { None } }
    pub fn get_drop_<'a>(&'a self)          -> Option<&'a SDL_DropEvent>             { if self.event_type.is_drop_()    { Some(unsafe { self.drop_()    }) } else { None } }

    // Unsafe event casts – make sure you check the `event_type` first!
    pub unsafe fn window<'a>(&'a self)      -> &'a SDL_WindowEvent                   { transmute(self) }
    pub unsafe fn key<'a>(&'a self)         -> &'a SDL_KeyboardEvent                 { transmute(self) }
    pub unsafe fn edit<'a>(&'a self)        -> &'a SDL_TextEditingEvent              { transmute(self) }
    pub unsafe fn text<'a>(&'a self)        -> &'a SDL_TextInputEvent                { transmute(self) }
    pub unsafe fn motion<'a>(&'a self)      -> &'a SDL_MouseMotionEvent              { transmute(self) }
    pub unsafe fn button<'a>(&'a self)      -> &'a SDL_MouseButtonEvent              { transmute(self) }
    pub unsafe fn wheel<'a>(&'a self)       -> &'a SDL_MouseWheelEvent               { transmute(self) }
    pub unsafe fn jaxis<'a>(&'a self)       -> &'a SDL_JoyAxisEvent                  { transmute(self) }
    pub unsafe fn jball<'a>(&'a self)       -> &'a SDL_JoyBallEvent                  { transmute(self) }
    pub unsafe fn jhat<'a>(&'a self)        -> &'a SDL_JoyHatEvent                   { transmute(self) }
    pub unsafe fn jbutton<'a>(&'a self)     -> &'a SDL_JoyButtonEvent                { transmute(self) }
    pub unsafe fn jdevice<'a>(&'a self)     -> &'a SDL_JoyDeviceEvent                { transmute(self) }
    pub unsafe fn caxis<'a>(&'a self)       -> &'a SDL_ControllerAxisEvent           { transmute(self) }
    pub unsafe fn cbutton<'a>(&'a self)     -> &'a SDL_ControllerButtonEvent         { transmute(self) }
    pub unsafe fn cdevice<'a>(&'a self)     -> &'a SDL_ControllerDeviceEvent         { transmute(self) }
    pub unsafe fn quit<'a>(&'a self)        -> &'a SDL_QuitEvent                     { transmute(self) }
    pub unsafe fn user<'a>(&'a self)        -> &'a SDL_UserEvent                     { transmute(self) }
    pub unsafe fn syswm<'a>(&'a self)       -> &'a SDL_SysWMEvent                    { transmute(self) }
    pub unsafe fn tfinger<'a>(&'a self)     -> &'a SDL_TouchFingerEvent              { transmute(self) }
    pub unsafe fn mgesture<'a>(&'a self)    -> &'a SDL_MultiGestureEvent             { transmute(self) }
    pub unsafe fn dgesture<'a>(&'a self)    -> &'a SDL_DollarGestureEvent            { transmute(self) }
    pub unsafe fn drop_<'a>(&'a self)       -> &'a SDL_DropEvent                     { transmute(self) }
}

// There is no need for this struct because the fields are already covered by
// the fields in the `SDL_Event` struct.
/* 
pub struct SDL_CommonEvent {
    event_type: SDL_EventType,
    timestamp:  Uint32,
}
*/

pub struct SDL_WindowEvent {
    /// The event type. Should be `SDL_WINDOWEVENT`.
    pub event_type: SDL_EventType,
    /// The timestamp of when the event was triggered.
    pub timestamp:  Uint32,
    /// The associated window.
    pub windowID:   Uint32,
    pub event:      Uint8,
    padding1:       Uint8,
    padding2:       Uint8,
    padding3:       Uint8,
    pub data1:      Sint32,
    pub data2:      Sint32,
}

pub struct SDL_KeyboardEvent {
    /// The event type. Should be `SDL_KEYDOWN | SDL_KEYUP`.
    pub event_type: SDL_EventType,
    /// The timestamp of when the event was triggered.
    pub timestamp:  Uint32,
    /// The window with keyboard focus, if any.
    pub windowID:   Uint32,
    pub state:      SDL_ButtonState,
    pub repeat:     Uint8,
    padding2:       Uint8,
    padding3:       Uint8,
    pub keysym:     SDL_Keysym,
}

pub static SDL_TEXTEDITINGEVENT_TEXT_SIZE: uint = 32;

pub struct SDL_TextEditingEvent {
    /// The event type. Should be `SDL_TEXTEDITING`.
    pub event_type: SDL_EventType,
    /// The timestamp of when the event was triggered.
    pub timestamp:  Uint32,
    /// The window with keyboard focus, if any.
    pub windowID:   Uint32,
    pub text:       [c_char, ..SDL_TEXTEDITINGEVENT_TEXT_SIZE],
    pub start:      Sint32,
    pub length:     Sint32,
}

pub static SDL_TEXTINPUTEVENT_TEXT_SIZE: uint = 32;

pub struct SDL_TextInputEvent {
    /// The event type. Should be `SDL_TEXTINPUT`.
    pub event_type: SDL_EventType,
    /// The timestamp of when the event was triggered.
    pub timestamp:  Uint32,
    /// The window with keyboard focus, if any.
    pub windowID:   Uint32,
    pub text:       [c_char, ..SDL_TEXTINPUTEVENT_TEXT_SIZE],
}

pub struct SDL_MouseMotionEvent {
    /// The event type. Should be `SDL_MOUSEMOTION`.
    pub event_type: SDL_EventType,
    /// The timestamp of when the event was triggered.
    pub timestamp:  Uint32,
    /// The window with mouse focus, if any.
    pub windowID:   Uint32,
    pub which:      Uint32,
    pub state:      Uint32,
    pub x:          Sint32,
    pub y:          Sint32,
    pub xrel:       Sint32,
    pub yrel:       Sint32,
}

pub struct SDL_MouseButtonEvent {
    /// The event type. Should be `SDL_MOUSEBUTTONDOWN | SDL_MOUSEBUTTONUP`.
    pub event_type: SDL_EventType,
    /// The timestamp of when the event was triggered.
    pub timestamp:  Uint32,
    /// The window with mouse focus, if any.
    pub windowID:   Uint32,
    pub which:      Uint32,
    pub button:     Uint8,
    pub state:      SDL_ButtonState,
    padding1:       Uint8,
    padding2:       Uint8,
    pub x:          Sint32,
    pub y:          Sint32,
}

pub struct SDL_MouseWheelEvent {
    /// The event type. Should be `SDL_MOUSEWHEEL`.
    pub event_type: SDL_EventType,
    /// The timestamp of when the event was triggered.
    pub timestamp:  Uint32,
    /// The window with mouse focus, if any.
    pub windowID:   Uint32,
    pub which:      Uint32,
    pub x:          Sint32,
    pub y:          Sint32,
}

pub struct SDL_JoyAxisEvent {
    /// The event type. Should be `SDL_JOYAXISMOTION`.
    pub event_type: SDL_EventType,
    /// The timestamp of when the event was triggered.
    pub timestamp:  Uint32,
    pub which:      SDL_JoystickID,
    pub axis:       Uint8,
    padding1:       Uint8,
    padding2:       Uint8,
    padding3:       Uint8,
    pub value:      Sint16,
    padding4:       Uint16,
}

pub struct SDL_JoyBallEvent {
    /// The event type. Should be `SDL_JOYBALLMOTION`.
    pub event_type: SDL_EventType,
    /// The timestamp of when the event was triggered.
    pub timestamp:  Uint32,
    pub which:      SDL_JoystickID,
    pub ball:       Uint8,
    padding1:       Uint8,
    padding2:       Uint8,
    padding3:       Uint8,
    pub xrel:       Sint16,
    pub yrel:       Sint16,
}

pub struct SDL_JoyHatEvent {
    /// The event type. Should be `SDL_JOYHATMOTION`.
    pub event_type: SDL_EventType,
    /// The timestamp of when the event was triggered.
    pub timestamp:  Uint32,
    pub which:      SDL_JoystickID,
    pub hat:        Uint8,
    pub value:      Uint8,
    padding1:       Uint8,
    padding2:       Uint8,
}

pub struct SDL_JoyButtonEvent {
    /// The event type. Should be `SDL_JOYBUTTONDOWN | SDL_JOYBUTTONUP`.
    pub event_type: SDL_EventType,
    /// The timestamp of when the event was triggered.
    pub timestamp:  Uint32,
    pub which:      SDL_JoystickID,
    pub button:     Uint8,
    pub state:      SDL_ButtonState,
    padding1:       Uint8,
    padding2:       Uint8,
}

pub struct SDL_JoyDeviceEvent {
    /// The event type. Should be `SDL_JOYDEVICEADDED | SDL_JOYDEVICEREMOVED`.
    pub event_type: SDL_EventType,
    /// The timestamp of when the event was triggered.
    pub timestamp:  Uint32,
    pub which:      Sint32,
}

pub struct SDL_ControllerAxisEvent {
    /// The event type. Should be `SDL_CONTROLLERAXISMOTION`.
    pub event_type: SDL_EventType,
    /// The timestamp of when the event was triggered.
    pub timestamp:  Uint32,
    pub which:      SDL_JoystickID,
    pub axis:       Uint8,
    padding1:       Uint8,
    padding2:       Uint8,
    padding3:       Uint8,
    pub value:      Sint16,
    padding4:       Uint16,
}

pub struct SDL_ControllerButtonEvent {
    /// The event type. Should be `SDL_CONTROLLERBUTTONDOWN | SDL_CONTROLLERBUTTONUP`.
    pub event_type: SDL_EventType,
    /// The timestamp of when the event was triggered.
    pub timestamp:  Uint32,
    pub which:      SDL_JoystickID,
    pub button:     Uint8,
    pub state:      SDL_ButtonState,
    padding1:       Uint8,
    padding2:       Uint8,
}

pub struct SDL_ControllerDeviceEvent {
    /// The event type. Should be `SDL_CONTROLLERDEVICEADDED | SDL_CONTROLLERDEVICEREMOVED | SDL_CONTROLLERDEVICEREMAPPED`.
    pub event_type: SDL_EventType,
    /// The timestamp of when the event was triggered.
    pub timestamp:  Uint32,
    pub which:      Sint32,
}

pub struct SDL_TouchFingerEvent {
    /// The event type. Should be `SDL_FINGERMOTION | SDL_FINGERDOWN | SDL_FINGERUP`.
    pub event_type: SDL_EventType,
    /// The timestamp of when the event was triggered.
    pub timestamp:  Uint32,
    pub touchId:    SDL_TouchID,
    pub fingerId:   SDL_FingerID,
    pub x:          c_float,
    pub y:          c_float,
    pub dx:         c_float,
    pub dy:         c_float,
    pub pressure:   c_float,
}

pub struct SDL_MultiGestureEvent {
    /// The event type. Should be `SDL_MULTIGESTURE`.
    pub event_type: SDL_EventType,
    /// The timestamp of when the event was triggered.
    pub timestamp:  Uint32,
    pub touchId:    SDL_TouchID,
    pub dTheta:     c_float,
    pub dDist:      c_float,
    pub x:          c_float,
    pub y:          c_float,
    pub numFingers: Uint16,
    padding:         Uint16,
}

pub struct SDL_DollarGestureEvent {
    /// The event type. Should be `SDL_DOLLARGESTURE`.
    pub event_type: SDL_EventType,
    /// The timestamp of when the event was triggered.
    pub timestamp:  Uint32,
    pub touchId:    SDL_TouchID,
    pub gestureId:  SDL_GestureID,
    pub numFingers: Uint32,
    pub error:      c_float,
    pub x:          c_float,
    pub y:          c_float,
}

pub struct SDL_DropEvent {
    /// The event type. Should be `SDL_DROPFILE`.
    pub event_type: SDL_EventType,
    /// The timestamp of when the event was triggered.
    pub timestamp:  Uint32,
    pub file:       *c_char,
}

pub struct SDL_QuitEvent {
    /// The event type. Should be `SDL_QUIT`.
    pub event_type: SDL_EventType,
    /// The timestamp of when the event was triggered.
    pub timestamp:  Uint32,
}

pub struct SDL_OSEvent {
    /// The event type. Should be `SDL_QUIT`.
    pub event_type: SDL_EventType,
    /// The timestamp of when the event was triggered.
    pub timestamp:  Uint32,
}

pub struct SDL_UserEvent {
    /// The event type. Should be `SDL_USEREVENT <= event_type < SDL_LASTEVENT`.
    pub event_type: SDL_EventType,
    /// The timestamp of when the event was triggered.
    pub timestamp:  Uint32,
    /// The associated window, if any.
    pub windowID:   Uint32,
    pub code:       Sint32,
    pub data1:      *c_void,
    pub data2:      *c_void,
}

pub enum SDL_SysWMmsg {}

pub struct SDL_SysWMEvent {
    /// The event type. Should be `SDL_SYSWMEVENT`.
    pub event_type: SDL_EventType,
    /// The timestamp of when the event was triggered.
    pub timestamp:  Uint32,
    pub msg:        *SDL_SysWMmsg,
}

#[repr(C)]
pub enum SDL_eventaction {
    SDL_ADDEVENT,
    SDL_PEEKEVENT,
    SDL_GETEVENT,
}

pub type SDL_EventFilter = extern "C" fn(userdata: *c_void, event: *SDL_Event) -> c_int;

#[repr(C)]
pub enum SDL_EventStateType {
    SDL_QUERY   = -1 as c_int,
    SDL_IGNORE  =  0 as c_int,
    SDL_ENABLE  =  1 as c_int,
}

pub static SDL_DISABLE: SDL_EventStateType = SDL_IGNORE;

#[inline]
pub unsafe fn SDL_GetEventState(event_type: SDL_EventType) -> Uint8 {
    SDL_EventState(event_type, SDL_QUERY)
}

extern "C" {
    pub fn SDL_PumpEvents();
    pub fn SDL_PeepEvents(events: *SDL_Event, numevents: c_int, action: SDL_eventaction, minType: SDL_EventType, maxType: SDL_EventType) -> c_int;
    pub fn SDL_HasEvent(event_type: SDL_EventType) -> SDL_bool;
    pub fn SDL_HasEvents(minType: SDL_EventType, maxType: SDL_EventType) -> SDL_bool;
    pub fn SDL_FlushEvent(event_type: SDL_EventType);
    pub fn SDL_FlushEvents(minType: SDL_EventType, maxType: SDL_EventType);
    pub fn SDL_PollEvent(event: *SDL_Event) -> c_int;
    pub fn SDL_WaitEvent(event: *SDL_Event) -> c_int;
    pub fn SDL_WaitEventTimeout(event: *SDL_Event, timeout: c_int) -> c_int;
    pub fn SDL_PushEvent(event: *SDL_Event) -> c_int;
    pub fn SDL_SetEventFilter(filter: SDL_EventFilter, userdata: *c_void);
    pub fn SDL_GetEventFilter(filter: *SDL_EventFilter, userdata: **c_void) -> SDL_bool;
    pub fn SDL_AddEventWatch(filter: SDL_EventFilter, userdata: *c_void);
    pub fn SDL_DelEventWatch(filter: SDL_EventFilter, userdata: *c_void);
    pub fn SDL_FilterEvents(filter: SDL_EventFilter, userdata: *c_void);
    pub fn SDL_EventState(event_type: SDL_EventType, state: SDL_EventStateType) -> Uint8;
    pub fn SDL_RegisterEvents(numevents: c_int) -> SDL_EventType;
}
