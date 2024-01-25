pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn north_west(&self) -> Point {
        Point {
            x: self.x - 1,
            y: self.y - 1,
        }
    }

    pub fn north(&self) -> Point {
        Point {
            x: self.x,
            y: self.y - 1,
        }
    }

    pub fn north_east(&self) -> Point {
        Point {
            x: self.x + 1,
            y: self.y - 1,
        }
    }

    pub fn east(&self) -> Point {
        Point {
            x: self.x + 1,
            y: self.y,
        }
    }

    pub fn south_west(&self) -> Point {
        Point {
            x: self.x - 1,
            y: self.y + 1,
        }
    }

    pub  fn south(&self) -> Point {
        Point {
            x: self.x,
            y: self.y + 1,
        }
    }

    pub fn south_east(&self) -> Point {
        Point {
            x: self.x + 1,
            y: self.y + 1,
        }
    }

    pub fn west(&self) -> Point {
        Point {
            x: self.x - 1,
            y: self.y,
        }
    }
}
