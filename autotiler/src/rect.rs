use crate::point::Point;

#[derive(Clone)]
pub struct Rect {
    pub x: i32,
    pub right: i32,
    pub y: i32,
    pub bottom: i32,
    pub w: i32,
    pub h: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self {
            x,
            y,
            w,
            h,
            right: x + w,
            bottom: y + h,
        }
    }
    pub fn contains(&self, pt: &Point) -> bool {
        pt.x >= self.x &&
            pt.x < self.right &&
            pt.y >= self.y &&
            pt.y < self.bottom
    }
}