use std::libc::c_int;
use ffi::rect::SDL_Rect;


#[deriving(Eq)]
pub struct Point {
    x: int,
    y: int,
}

#[inline]
pub fn ArePointEquals(a: &Point, b: &Point) -> bool {
    if *a == *b { true } else { false }
}

#[inline]
pub fn Point(x: int, y: int) -> Point {
    Point { x: x, y: y}
}

impl Point {
    pub fn new(x: int, y: int) -> Point {
        Point {
            x: x,
            y: y
        }
    }
}

#[deriving(Eq)]
pub struct Rect {
     x: int,
     y: int,
     w: int,
     h: int
}


#[inline]
pub fn IsRectEmpty(r: &Rect) -> bool {
    if (r.w <= 0) || (r.h <= 0) { true } else { false }
}

#[inline]
pub fn AreRectEquals(a: &Rect, b: &Rect) -> bool {
    if *a == *b { true } else { false }
}

#[inline]
pub fn Rect(x: int, y: int, w: int, h: int) -> Rect {
    Rect { x: x, y: y, w: w, h: h }
}

impl Rect {
    pub fn new(x: int, y: int, w: int, h: int) -> Rect {
        Rect {
            x: x,
            y: y,
            w: w,
            h: h
        }
    }

    #[doc(hidden)]
    pub fn wrap(rect : *SDL_Rect) -> Rect {
        unsafe {
            Rect {
                x : (*rect).x as int,
                y : (*rect).y as int,
                w : (*rect).w as int,
                h : (*rect).h as int,
            }
        }
    }

    #[doc(hidden)]
    pub fn unwrap(&self) -> SDL_Rect {
        SDL_Rect {
            x : self.x as c_int,
            y : self.y as c_int,
            w : self.w as c_int,
            h : self.h as c_int,
        }
    }
}

