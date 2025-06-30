use gual::geometry2d::{Bivector, Vector};

mod antiwedge;
mod complement;
mod contraction;
mod dot;
mod dual;
mod expansion;
mod metric;
mod norm;
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

trait ToF32 {
    type Output;
    fn to_f32(&self) -> Self::Output;
}

impl<M> ToF32 for Vector<i32, M> {
    type Output = Vector<f32, M>;
    fn to_f32(&self) -> Vector<f32, M> {
        Vector::new(self.x as f32, self.y as f32)
    }
}

impl<M> ToF32 for Bivector<i32, M> {
    type Output = Bivector<f32, M>;
    fn to_f32(&self) -> Bivector<f32, M> {
        Bivector::new(self.xy as f32)
    }
}
