use gual::geometry2d::{Bivector, Vector};

mod antiwedge;
mod complement;
mod contraction;
mod expansion;
mod metric;
mod wedge;

struct ScalarIt {
    s: i32,
    max: i32,
}

struct VectorIt {
    x: i32,
    y: i32,
    max: i32,
}

struct BivectorIt {
    xy: i32,
    max: i32,
}

impl ScalarIt {
    fn new(max: i32) -> Self {
        Self { s: 0, max }
    }
}

impl VectorIt {
    fn new(max: i32) -> Self {
        Self { x: 0, y: 0, max }
    }
}

impl BivectorIt {
    fn new(max: i32) -> Self {
        Self { xy: 0, max }
    }
}

impl Iterator for ScalarIt {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.s < self.max {
            let s = self.s;
            self.s += 1;
            Some(s)
        } else {
            None
        }
    }
}

impl Iterator for VectorIt {
    type Item = Vector<i32>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.y < self.max {
            if self.x < self.max {
                let x = self.x;
                self.x += 1;
                Some(Vector::new(x, self.y))
            } else {
                let y = self.y;
                self.x = 0;
                self.y += 1;
                Some(Vector::new(self.x, y))
            }
        } else {
            None
        }
    }
}

impl Iterator for BivectorIt {
    type Item = Bivector<i32>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.xy < self.max {
            let xy = self.xy;
            self.xy += 1;
            Some(Bivector::new(xy))
        } else {
            None
        }
    }
}
