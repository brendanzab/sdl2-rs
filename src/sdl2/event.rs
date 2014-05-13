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

use ffi::event::*;

pub type EventStamped = (u32, Event);

pub struct PressedState {
    Pressed,
    Released,
}

pub enum Event {
    AppTerminating,
    AppLowMemory,
    AppWillEnterBackground,
    AppDidEnterBackground,
    AppWillEnterForeground,
    AppDidEnterForeground,
    WindowEvent,
    Keyboard,
    TextEditing,
    TextInput,
    MouseMotion,
    MouseButton,
    MouseWheel,
    JoyAxis,
    JoyBall,
    JoyHat,
    JoyButton,
    JoyDevice,
    ControllerAxis,
    ControllerButton,
    ControllerDevice,
    TouchFinger,
    MultiGesture,
    DollarGesture,
    Drop,
    Quit,
    User,
    SysWm,
}

impl Event {
    fn from_sdl_event_data(event_type: Uint32, data: SDL_Event) -> Event {
    }

    fn from_sdl_window_event(data: SDL_WindowEvent) -> Event {
         WindowEvent
    }

    fn from_sdl_keyboard(data: SDL_KeyboardEvent) -> Event {
         Keyboard
    }

    fn from_sdl_text_editing(data: SDL_TextEditingEvent) -> Event {
         TextEditing
    }

    fn from_sdl_text_input(data: SDL_TextInputEvent) -> Event {
         TextInput
    }

    fn from_sdl_mouse_motion(data: SDL_MouseMotionEvent) -> Event {
         MouseMotion
    }

    fn from_sdl_mouse_button(data: SDL_MouseButtonEvent) -> Event {
         MouseButton
    }

    fn from_sdl_mouse_wheel(data: SDL_MouseWheelEvent) -> Event {
         MouseWheel
    }

    fn from_sdl_joy_axis(data: SDL_JoyAxisEvent) -> Event {
         JoyAxis
    }

    fn from_sdl_joy_ball(data: SDL_JoyBallEvent) -> Event {
         JoyBall
    }

    fn from_sdl_joy_hat(data: SDL_JoyHatEvent) -> Event {
         JoyHat
    }

    fn from_sdl_joy_button(data: SDL_JoyButtonEvent) -> Event {
         JoyButton
    }

    fn from_sdl_joy_device(data: SDL_JoyDeviceEvent) -> Event {
         JoyDevice
    }

    fn from_sdl_controller_axis(data: SDL_ControllerAxisEvent) -> Event {
         ControllerAxis
    }

    fn from_sdl_controller_button(data: SDL_ControllerButtonEvent) -> Event {
         ControllerButton
    }

    fn from_sdl_controller_device(data: SDL_ControllerDeviceEvent) -> Event {
         ControllerDevice
    }

    fn from_sdl_touch_finger(data: SDL_TouchFingerEvent) -> Event {
         TouchFinger
    }

    fn from_sdl_multi_gesture(data: SDL_MultiGestureEvent) -> Event {
         MultiGesture
    }

    fn from_sdl_dollar_gesture(data: SDL_DollarGestureEvent) -> Event {
         DollarGesture
    }

    fn from_sdl_drop(data: SDL_DropEvent) -> Event {
         Drop
    }

    fn from_sdl_user(data: SDL_UserEvent) -> Event {
         User
    }

    fn from_sdl_syswm(data: SDL_SysWMEvent) -> Event {
         SysWM
    }
}

fn make_event(SDL_Event { timestamp, event_type, data }: SDL_Event) -> EventStamped {
    static LAST: Uint32 = SDL_LASTEVENT - 1;
    let event = match event_type {
        SDL_QUIT                        => Quit,
        SDL_APP_TERMINATING             => AppTerminating,
        SDL_APP_LOWMEMORY               => AppLowMemory,
        SDL_APP_WILLENTERBACKGROUND     => AppWillEnterBackground,
        SDL_APP_DIDENTERBACKGROUND      => AppDidEnterBackground,
        SDL_APP_WILLENTERFOREGROUND     => AppWillEnterForeground,
        SDL_APP_DIDENTERFOREGROUND      => AppDidEnterForeground,
        SDL_WINDOWEVENT                 => WindowEvent,
        SDL_KEYDOWN                     => Keyboard,
        SDL_KEYUP                       => Keyboard,
        SDL_TEXTEDITING                 => TextEditing,
        SDL_TEXTINPUT                   => TextInput,
        SDL_MOUSEMOTION                 => MouseMotion,
        SDL_MOUSEBUTTONDOWN             => MouseButton,
        SDL_MOUSEBUTTONUP               => MouseButton,
        SDL_MOUSEWHEEL                  => MouseWheel,
        SDL_JOYAXISMOTION               => JoyAxis,
        SDL_JOYBALLMOTION               => JoyBall,
        SDL_JOYHATMOTION                => JoyHat,
        SDL_JOYBUTTONDOWN               => JoyButton,
        SDL_JOYBUTTONUP                 => JoyButton,
        SDL_JOYDEVICEADDED              => JoyDevice,
        SDL_JOYDEVICEREMOVED            => JoyDevice,
        SDL_CONTROLLERAXISMOTION        => ControllerAxis,
        SDL_CONTROLLERBUTTONDOWN        => ControllerButton,
        SDL_CONTROLLERBUTTONUP          => ControllerButton,
        SDL_CONTROLLERDEVICEADDED       => ControllerDevice,
        SDL_CONTROLLERDEVICEREMOVED     => ControllerDevice,
        SDL_CONTROLLERDEVICEREMAPPED    => ControllerDevice,
        SDL_FINGERDOWN                  => TouchFinger,
        SDL_FINGERUP                    => TouchFinger,
        SDL_FINGERMOTION                => TouchFinger,
        SDL_DOLLARGESTURE               => DollarGesture,
        // SDL_DOLLARRECORD                => ...,
        SDL_MULTIGESTURE                => MultiGesture,
        SDL_CLIPBOARDUPDATE             => CLIPBOARDUPDATE,
        SDL_DROPFILE                    => Drop,
        SDL_USEREVENT .. LAST           => UserEvent,
        _                               => unreachable!(),
    };
    (timestamp, event)
}
