use gual::geometry3d::*;

mod antiwedge;
mod complement;
mod wedge;

struct ScalarIt {
    s: i32,
    max: i32,
}

struct VectorIt {
    x: i32,
    y: i32,
    z: i32,
    max: i32,
}

struct BivectorIt {
    yz: i32,
    zx: i32,
    xy: i32,
    max: i32,
}

struct TrivectorIt {
    xyz: i32,
    max: i32,
}

impl ScalarIt {
    fn new(max: i32) -> Self {
        Self { s: 0, max }
    }
}

impl VectorIt {
    fn new(max: i32) -> Self {
        Self {
            x: 0,
            y: 0,
            z: 0,
            max,
        }
    }
}

impl BivectorIt {
    fn new(max: i32) -> Self {
        Self {
            yz: 0,
            zx: 0,
            xy: 0,
            max,
        }
    }
}

impl TrivectorIt {
    fn new(max: i32) -> Self {
        Self { xyz: 0, max }
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

impl Iterator for BivectorIt {
    type Item = Bivector<i32>;
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

impl Iterator for TrivectorIt {
    type Item = Trivector<i32>;
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
