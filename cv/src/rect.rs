use crate::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Rect {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) width: i32,
    pub(crate) height: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Rect { x, y, width, height }
    }

    pub fn center(p: &Point, width: i32, height: i32) -> Self {
        let x = p.x - (width / 2);
        let y = p.y - (height / 2);
        Rect::new(x, y, width, height)
    }

    pub(crate) fn pack(rect: opencv::core::Rect) -> Self {
        Rect {
            x: rect.x,
            y: rect.y,
            width: rect.width,
            height: rect.height,
        }
    }

    pub(crate) fn unpack(&self) -> opencv::core::Rect {
        opencv::core::Rect_ {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
        }
    }

    pub fn contains(&self, pt: Point) -> bool {
        self.unpack().contains(pt.unpack())
    }

    pub fn center_at(&mut self, pt: Point) {
        self.x = pt.x + self.width / 2;
        self.y = pt.y + self.height / 2;
    }
}
