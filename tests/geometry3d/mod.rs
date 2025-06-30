use std::marker::PhantomData;

use gual::geometry3d::*;

mod antiwedge;
mod complement;
mod contraction;
mod dot;
mod dual;
mod metric;
mod norm;
mod wedge;

struct ScalarIt {
    s: i32,
    max: i32,
}

struct VectorIt<M> {
    x: i32,
    y: i32,
    z: i32,
    max: i32,
    _metric: PhantomData<M>,
}

struct BivectorIt<M> {
    yz: i32,
    zx: i32,
    xy: i32,
    max: i32,
    _metric: PhantomData<M>,
}

struct TrivectorIt<M> {
    xyz: i32,
    max: i32,
    _metric: PhantomData<M>,
}

impl ScalarIt {
    fn new(max: i32) -> Self {
        Self { s: 0, max }
    }
}

impl<M> VectorIt<M> {
    fn new(max: i32) -> Self {
        Self {
            x: 0,
            y: 0,
            z: 0,
            max,
            _metric: PhantomData,
        }
    }
}

impl<M> BivectorIt<M> {
    fn new(max: i32) -> Self {
        Self {
            yz: 0,
            zx: 0,
            xy: 0,
            max,
            _metric: PhantomData,
        }
    }
}

impl<M> TrivectorIt<M> {
    fn new(max: i32) -> Self {
        Self {
            xyz: 0,
            max,
            _metric: PhantomData,
        }
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

impl<M> Iterator for VectorIt<M> {
    type Item = Vector<i32, M>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.z < self.max {
            if self.y < self.max {
                if self.x < self.max {
                    let x = self.x;
                    self.x += 1;
                    Some(Vector::new(x, self.y, self.z))
                } else {
                    let y = self.y;
                    self.x = 0;
                    self.y += 1;
                    Some(Vector::new(self.x, y, self.z))
                }
            } else {
                let z = self.z;
                self.x = 0;
                self.y = 0;
                self.z += 1;
                Some(Vector::new(self.x, self.y, z))
            }
        } else {
            None
        }
    }
}

impl<M> Iterator for BivectorIt<M> {
    type Item = Bivector<i32, M>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.xy < self.max {
            if self.zx < self.max {
                if self.yz < self.max {
                    let yz = self.yz;
                    self.yz += 1;
                    Some(Bivector::new(yz, self.zx, self.xy))
                } else {
                    let zx = self.zx;
                    self.yz = 0;
                    self.zx += 1;
                    Some(Bivector::new(self.yz, zx, self.xy))
                }
            } else {
                let xy = self.xy;
                self.yz = 0;
                self.zx = 0;
                self.xy += 1;
                Some(Bivector::new(self.yz, self.zx, xy))
            }
        } else {
            None
        }
    }
}

impl<M> Iterator for TrivectorIt<M> {
    type Item = Trivector<i32, M>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.xyz < self.max {
            let xyz = self.xyz;
            self.xyz += 1;
            Some(Trivector::new(xyz))
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
        Vector::new(self.x as f32, self.y as f32, self.z as f32)
    }
}

impl<M> ToF32 for Bivector<i32, M> {
    type Output = Bivector<f32, M>;
    fn to_f32(&self) -> Bivector<f32, M> {
        Bivector::new(self.yz as f32, self.zx as f32, self.xy as f32)
    }
}

impl<M> ToF32 for Trivector<i32, M> {
    type Output = Trivector<f32, M>;
    fn to_f32(&self) -> Trivector<f32, M> {
        Trivector::new(self.xyz as f32)
    }
}
