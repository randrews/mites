use crate::mite::Point;

pub type Size = (u32, u32);

pub struct World {
    bitmap: Vec<bool>,
    size: Size,
}

impl World {
    pub fn new(size: Size) -> Self {
        let len = (size.0 * size.1) as usize;
        let mut bitmap = Vec::with_capacity(len);
        bitmap.resize(len, false);
        Self {
            bitmap,
            size,
        }
    }

    pub fn flip(&mut self, coord: Point) {
        let n = (coord.0 + self.size.0 * coord.1) as usize;
        self.bitmap[n] = !self.bitmap[n]
    }

    pub fn query(&self, coord: Point) -> bool {
        let n = (coord.0 + self.size.0 * coord.1) as usize;
        self.bitmap[n]
    }
}