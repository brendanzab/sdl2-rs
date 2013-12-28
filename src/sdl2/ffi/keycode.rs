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

use ffi::scancode::*;
use ffi::stdinc::{Sint32, Uint16};

// SDL_keycode.h

pub type SDL_Keycode = Sint32;

static SDLK_SCANCODE_MASK: SDL_Keycode = 1 << 30;

macro_rules! SDL_SCANCODE_TO_KEYCODE(
    ($x:expr) => ($x as SDL_Keycode | SDLK_SCANCODE_MASK)
)

pub static SDLK_UNKNOWN:            SDL_Keycode = 0;

pub static SDLK_RETURN:             SDL_Keycode = '\r' as SDL_Keycode;
pub static SDLK_ESCAPE:             SDL_Keycode = 27; // '\033' as SDL_Keycode;
pub static SDLK_BACKSPACE:          SDL_Keycode = 8; // '\b' as SDL_Keycode;
pub static SDLK_TAB:                SDL_Keycode = '\t' as SDL_Keycode;
pub static SDLK_SPACE:              SDL_Keycode = ' ' as SDL_Keycode;
pub static SDLK_EXCLAIM:            SDL_Keycode = '!' as SDL_Keycode;
pub static SDLK_QUOTEDBL:           SDL_Keycode = '"' as SDL_Keycode;
pub static SDLK_HASH:               SDL_Keycode = '#' as SDL_Keycode;
pub static SDLK_PERCENT:            SDL_Keycode = '%' as SDL_Keycode;
pub static SDLK_DOLLAR:             SDL_Keycode = '$' as SDL_Keycode;
pub static SDLK_AMPERSAND:          SDL_Keycode = '&' as SDL_Keycode;
pub static SDLK_QUOTE:              SDL_Keycode = '\'' as SDL_Keycode;
pub static SDLK_LEFTPAREN:          SDL_Keycode = '(' as SDL_Keycode;
pub static SDLK_RIGHTPAREN:         SDL_Keycode = ')' as SDL_Keycode;
pub static SDLK_ASTERISK:           SDL_Keycode = '*' as SDL_Keycode;
pub static SDLK_PLUS:               SDL_Keycode = '+' as SDL_Keycode;
pub static SDLK_COMMA:              SDL_Keycode = ',' as SDL_Keycode;
pub static SDLK_MINUS:              SDL_Keycode = '-' as SDL_Keycode;
pub static SDLK_PERIOD:             SDL_Keycode = '.' as SDL_Keycode;
pub static SDLK_SLASH:              SDL_Keycode = '/' as SDL_Keycode;
pub static SDLK_0:                  SDL_Keycode = '0' as SDL_Keycode;
pub static SDLK_1:                  SDL_Keycode = '1' as SDL_Keycode;
pub static SDLK_2:                  SDL_Keycode = '2' as SDL_Keycode;
pub static SDLK_3:                  SDL_Keycode = '3' as SDL_Keycode;
pub static SDLK_4:                  SDL_Keycode = '4' as SDL_Keycode;
pub static SDLK_5:                  SDL_Keycode = '5' as SDL_Keycode;
pub static SDLK_6:                  SDL_Keycode = '6' as SDL_Keycode;
pub static SDLK_7:                  SDL_Keycode = '7' as SDL_Keycode;
pub static SDLK_8:                  SDL_Keycode = '8' as SDL_Keycode;
pub static SDLK_9:                  SDL_Keycode = '9' as SDL_Keycode;
pub static SDLK_COLON:              SDL_Keycode = ':' as SDL_Keycode;
pub static SDLK_SEMICOLON:          SDL_Keycode = ';' as SDL_Keycode;
pub static SDLK_LESS:               SDL_Keycode = '<' as SDL_Keycode;
pub static SDLK_EQUALS:             SDL_Keycode = '=' as SDL_Keycode;
pub static SDLK_GREATER:            SDL_Keycode = '>' as SDL_Keycode;
pub static SDLK_QUESTION:           SDL_Keycode = '?' as SDL_Keycode;
pub static SDLK_AT:                 SDL_Keycode = '@' as SDL_Keycode;

pub static SDLK_LEFTBRACKET:        SDL_Keycode = '[' as SDL_Keycode;
pub static SDLK_BACKSLASH:          SDL_Keycode = '\\' as SDL_Keycode;
pub static SDLK_RIGHTBRACKET:       SDL_Keycode = ']' as SDL_Keycode;
pub static SDLK_CARET:              SDL_Keycode = '^' as SDL_Keycode;
pub static SDLK_UNDERSCORE:         SDL_Keycode = '_' as SDL_Keycode;
pub static SDLK_BACKQUOTE:          SDL_Keycode = '`' as SDL_Keycode;
pub static SDLK_a:                  SDL_Keycode = 'a' as SDL_Keycode;
pub static SDLK_b:                  SDL_Keycode = 'b' as SDL_Keycode;
pub static SDLK_c:                  SDL_Keycode = 'c' as SDL_Keycode;
pub static SDLK_d:                  SDL_Keycode = 'd' as SDL_Keycode;
pub static SDLK_e:                  SDL_Keycode = 'e' as SDL_Keycode;
pub static SDLK_f:                  SDL_Keycode = 'f' as SDL_Keycode;
pub static SDLK_g:                  SDL_Keycode = 'g' as SDL_Keycode;
pub static SDLK_h:                  SDL_Keycode = 'h' as SDL_Keycode;
pub static SDLK_i:                  SDL_Keycode = 'i' as SDL_Keycode;
pub static SDLK_j:                  SDL_Keycode = 'j' as SDL_Keycode;
pub static SDLK_k:                  SDL_Keycode = 'k' as SDL_Keycode;
pub static SDLK_l:                  SDL_Keycode = 'l' as SDL_Keycode;
pub static SDLK_m:                  SDL_Keycode = 'm' as SDL_Keycode;
pub static SDLK_n:                  SDL_Keycode = 'n' as SDL_Keycode;
pub static SDLK_o:                  SDL_Keycode = 'o' as SDL_Keycode;
pub static SDLK_p:                  SDL_Keycode = 'p' as SDL_Keycode;
pub static SDLK_q:                  SDL_Keycode = 'q' as SDL_Keycode;
pub static SDLK_r:                  SDL_Keycode = 'r' as SDL_Keycode;
pub static SDLK_s:                  SDL_Keycode = 's' as SDL_Keycode;
pub static SDLK_t:                  SDL_Keycode = 't' as SDL_Keycode;
pub static SDLK_u:                  SDL_Keycode = 'u' as SDL_Keycode;
pub static SDLK_v:                  SDL_Keycode = 'v' as SDL_Keycode;
pub static SDLK_w:                  SDL_Keycode = 'w' as SDL_Keycode;
pub static SDLK_x:                  SDL_Keycode = 'x' as SDL_Keycode;
pub static SDLK_y:                  SDL_Keycode = 'y' as SDL_Keycode;
pub static SDLK_z:                  SDL_Keycode = 'z' as SDL_Keycode;

pub static SDLK_CAPSLOCK:           SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_CAPSLOCK);

pub static SDLK_F1:                 SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_F1);
pub static SDLK_F2:                 SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_F2);
pub static SDLK_F3:                 SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_F3);
pub static SDLK_F4:                 SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_F4);
pub static SDLK_F5:                 SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_F5);
pub static SDLK_F6:                 SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_F6);
pub static SDLK_F7:                 SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_F7);
pub static SDLK_F8:                 SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_F8);
pub static SDLK_F9:                 SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_F9);
pub static SDLK_F10:                SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_F10);
pub static SDLK_F11:                SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_F11);
pub static SDLK_F12:                SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_F12);

pub static SDLK_PRINTSCREEN:        SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_PRINTSCREEN);
pub static SDLK_SCROLLLOCK:         SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_SCROLLLOCK);
pub static SDLK_PAUSE:              SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_PAUSE);
pub static SDLK_INSERT:             SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_INSERT);
pub static SDLK_HOME:               SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_HOME);
pub static SDLK_PAGEUP:             SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_PAGEUP);
pub static SDLK_DELETE:             SDL_Keycode = 127; // '\177' as SDL_Keycode;
pub static SDLK_END:                SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_END);
pub static SDLK_PAGEDOWN:           SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_PAGEDOWN);
pub static SDLK_RIGHT:              SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_RIGHT);
pub static SDLK_LEFT:               SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_LEFT);
pub static SDLK_DOWN:               SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_DOWN);
pub static SDLK_UP:                 SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_UP);

pub static SDLK_NUMLOCKCLEAR:       SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_NUMLOCKCLEAR);
pub static SDLK_KP_DIVIDE:          SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_DIVIDE);
pub static SDLK_KP_MULTIPLY:        SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_MULTIPLY);
pub static SDLK_KP_MINUS:           SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_MINUS);
pub static SDLK_KP_PLUS:            SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_PLUS);
pub static SDLK_KP_ENTER:           SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_ENTER);
pub static SDLK_KP_1:               SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_1);
pub static SDLK_KP_2:               SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_2);
pub static SDLK_KP_3:               SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_3);
pub static SDLK_KP_4:               SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_4);
pub static SDLK_KP_5:               SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_5);
pub static SDLK_KP_6:               SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_6);
pub static SDLK_KP_7:               SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_7);
pub static SDLK_KP_8:               SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_8);
pub static SDLK_KP_9:               SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_9);
pub static SDLK_KP_0:               SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_0);
pub static SDLK_KP_PERIOD:          SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_PERIOD);

pub static SDLK_APPLICATION:        SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_APPLICATION);
pub static SDLK_POWER:              SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_POWER);
pub static SDLK_KP_EQUALS:          SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_EQUALS);
pub static SDLK_F13:                SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_F13);
pub static SDLK_F14:                SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_F14);
pub static SDLK_F15:                SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_F15);
pub static SDLK_F16:                SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_F16);
pub static SDLK_F17:                SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_F17);
pub static SDLK_F18:                SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_F18);
pub static SDLK_F19:                SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_F19);
pub static SDLK_F20:                SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_F20);
pub static SDLK_F21:                SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_F21);
pub static SDLK_F22:                SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_F22);
pub static SDLK_F23:                SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_F23);
pub static SDLK_F24:                SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_F24);
pub static SDLK_EXECUTE:            SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_EXECUTE);
pub static SDLK_HELP:               SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_HELP);
pub static SDLK_MENU:               SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_MENU);
pub static SDLK_SELECT:             SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_SELECT);
pub static SDLK_STOP:               SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_STOP);
pub static SDLK_AGAIN:              SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_AGAIN);
pub static SDLK_UNDO:               SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_UNDO);
pub static SDLK_CUT:                SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_CUT);
pub static SDLK_COPY:               SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_COPY);
pub static SDLK_PASTE:              SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_PASTE);
pub static SDLK_FIND:               SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_FIND);
pub static SDLK_MUTE:               SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_MUTE);
pub static SDLK_VOLUMEUP:           SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_VOLUMEUP);
pub static SDLK_VOLUMEDOWN:         SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_VOLUMEDOWN);
pub static SDLK_KP_COMMA:           SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_COMMA);
pub static SDLK_KP_EQUALSAS400:     SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_EQUALSAS400);

pub static SDLK_ALTERASE:           SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_ALTERASE);
pub static SDLK_SYSREQ:             SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_SYSREQ);
pub static SDLK_CANCEL:             SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_CANCEL);
pub static SDLK_CLEAR:              SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_CLEAR);
pub static SDLK_PRIOR:              SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_PRIOR);
pub static SDLK_RETURN2:            SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_RETURN2);
pub static SDLK_SEPARATOR:          SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_SEPARATOR);
pub static SDLK_OUT:                SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_OUT);
pub static SDLK_OPER:               SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_OPER);
pub static SDLK_CLEARAGAIN:         SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_CLEARAGAIN);
pub static SDLK_CRSEL:              SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_CRSEL);
pub static SDLK_EXSEL:              SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_EXSEL);

pub static SDLK_KP_00:              SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_00);
pub static SDLK_KP_000:             SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_000);
pub static SDLK_THOUSANDSSEPARATOR: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_THOUSANDSSEPARATOR);
pub static SDLK_DECIMALSEPARATOR:   SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_DECIMALSEPARATOR);
pub static SDLK_CURRENCYUNIT:       SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_CURRENCYUNIT);
pub static SDLK_CURRENCYSUBUNIT:    SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_CURRENCYSUBUNIT);
pub static SDLK_KP_LEFTPAREN:       SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_LEFTPAREN);
pub static SDLK_KP_RIGHTPAREN:      SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_RIGHTPAREN);
pub static SDLK_KP_LEFTBRACE:       SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_LEFTBRACE);
pub static SDLK_KP_RIGHTBRACE:      SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_RIGHTBRACE);
pub static SDLK_KP_TAB:             SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_TAB);
pub static SDLK_KP_BACKSPACE:       SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_BACKSPACE);
pub static SDLK_KP_A:               SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_A);
pub static SDLK_KP_B:               SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_B);
pub static SDLK_KP_C:               SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_C);
pub static SDLK_KP_D:               SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_D);
pub static SDLK_KP_E:               SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_E);
pub static SDLK_KP_F:               SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_F);
pub static SDLK_KP_XOR:             SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_XOR);
pub static SDLK_KP_POWER:           SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_POWER);
pub static SDLK_KP_PERCENT:         SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_PERCENT);
pub static SDLK_KP_LESS:            SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_LESS);
pub static SDLK_KP_GREATER:         SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_GREATER);
pub static SDLK_KP_AMPERSAND:       SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_AMPERSAND);
pub static SDLK_KP_DBLAMPERSAND:    SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_DBLAMPERSAND);
pub static SDLK_KP_VERTICALBAR:     SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_VERTICALBAR);
pub static SDLK_KP_DBLVERTICALBAR:  SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_DBLVERTICALBAR);
pub static SDLK_KP_COLON:           SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_COLON);
pub static SDLK_KP_HASH:            SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_HASH);
pub static SDLK_KP_SPACE:           SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_SPACE);
pub static SDLK_KP_AT:              SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_AT);
pub static SDLK_KP_EXCLAM:          SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_EXCLAM);
pub static SDLK_KP_MEMSTORE:        SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_MEMSTORE);
pub static SDLK_KP_MEMRECALL:       SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_MEMRECALL);
pub static SDLK_KP_MEMCLEAR:        SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_MEMCLEAR);
pub static SDLK_KP_MEMADD:          SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_MEMADD);
pub static SDLK_KP_MEMSUBTRACT:     SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_MEMSUBTRACT);
pub static SDLK_KP_MEMMULTIPLY:     SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_MEMMULTIPLY);
pub static SDLK_KP_MEMDIVIDE:       SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_MEMDIVIDE);
pub static SDLK_KP_PLUSMINUS:       SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_PLUSMINUS);
pub static SDLK_KP_CLEAR:           SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_CLEAR);
pub static SDLK_KP_CLEARENTRY:      SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_CLEARENTRY);
pub static SDLK_KP_BINARY:          SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_BINARY);
pub static SDLK_KP_OCTAL:           SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_OCTAL);
pub static SDLK_KP_DECIMAL:         SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_DECIMAL);
pub static SDLK_KP_HEXADECIMAL:     SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KP_HEXADECIMAL);

pub static SDLK_LCTRL:              SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_LCTRL);
pub static SDLK_LSHIFT:             SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_LSHIFT);
pub static SDLK_LALT:               SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_LALT);
pub static SDLK_LGUI:               SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_LGUI);
pub static SDLK_RCTRL:              SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_RCTRL);
pub static SDLK_RSHIFT:             SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_RSHIFT);
pub static SDLK_RALT:               SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_RALT);
pub static SDLK_RGUI:               SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_RGUI);

pub static SDLK_MODE:               SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_MODE);

pub static SDLK_AUDIONEXT:          SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_AUDIONEXT);
pub static SDLK_AUDIOPREV:          SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_AUDIOPREV);
pub static SDLK_AUDIOSTOP:          SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_AUDIOSTOP);
pub static SDLK_AUDIOPLAY:          SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_AUDIOPLAY);
pub static SDLK_AUDIOMUTE:          SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_AUDIOMUTE);
pub static SDLK_MEDIASELECT:        SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_MEDIASELECT);
pub static SDLK_WWW:                SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_WWW);
pub static SDLK_MAIL:               SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_MAIL);
pub static SDLK_CALCULATOR:         SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_CALCULATOR);
pub static SDLK_COMPUTER:           SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_COMPUTER);
pub static SDLK_AC_SEARCH:          SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_AC_SEARCH);
pub static SDLK_AC_HOME:            SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_AC_HOME);
pub static SDLK_AC_BACK:            SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_AC_BACK);
pub static SDLK_AC_FORWARD:         SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_AC_FORWARD);
pub static SDLK_AC_STOP:            SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_AC_STOP);
pub static SDLK_AC_REFRESH:         SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_AC_REFRESH);
pub static SDLK_AC_BOOKMARKS:       SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_AC_BOOKMARKS);

pub static SDLK_BRIGHTNESSDOWN:     SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_BRIGHTNESSDOWN);
pub static SDLK_BRIGHTNESSUP:       SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_BRIGHTNESSUP);
pub static SDLK_DISPLAYSWITCH:      SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_DISPLAYSWITCH);
pub static SDLK_KBDILLUMTOGGLE:     SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KBDILLUMTOGGLE);
pub static SDLK_KBDILLUMDOWN:       SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KBDILLUMDOWN);
pub static SDLK_KBDILLUMUP:         SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_KBDILLUMUP);
pub static SDLK_EJECT:              SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_EJECT);
pub static SDLK_SLEEP:              SDL_Keycode = SDL_SCANCODE_TO_KEYCODE!(SDL_SCANCODE_SLEEP);

pub type SDL_Keymod = Uint16;
pub static KMOD_NONE:       Uint16 = 0x0000;
pub static KMOD_LSHIFT:     Uint16 = 0x0001;
pub static KMOD_RSHIFT:     Uint16 = 0x0002;
pub static KMOD_LCTRL:      Uint16 = 0x0040;
pub static KMOD_RCTRL:      Uint16 = 0x0080;
pub static KMOD_LALT:       Uint16 = 0x0100;
pub static KMOD_RALT:       Uint16 = 0x0200;
pub static KMOD_LGUI:       Uint16 = 0x0400;
pub static KMOD_RGUI:       Uint16 = 0x0800;
pub static KMOD_NUM:        Uint16 = 0x1000;
pub static KMOD_CAPS:       Uint16 = 0x2000;
pub static KMOD_MODE:       Uint16 = 0x4000;
pub static KMOD_RESERVED:   Uint16 = 0x8000;

pub static KMOD_CTRL:       Uint16 = KMOD_LCTRL  | KMOD_RCTRL;
pub static KMOD_SHIFT:      Uint16 = KMOD_LSHIFT | KMOD_RSHIFT;
pub static KMOD_ALT:        Uint16 = KMOD_LALT   | KMOD_RALT;
pub static KMOD_GUI:        Uint16 = KMOD_LGUI   | KMOD_RGUI;
