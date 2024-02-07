#[derive(Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }

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

    pub fn local_region(&self) -> [Point; 9] {
        [
            self.north_west(),
            self.north(),
            self.north_east(),

            self.west(),
            self.clone(),
            self.east(),

            self.south_west(),
            self.south(),
            self.south_east(),
        ]
    }
}
