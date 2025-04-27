use gual::{
    Antiscalar, AntiwedgeProduct, Bivector3D, KVector, Scalar3D, Trivector3D, Vector3D,
    WedgeProduct, antiwedge_reference,
};
use num::traits::ConstOne;

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
    type Item = Scalar3D<i32>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.s < self.max {
            let s = self.s;
            self.s += 1;
            Some(Scalar3D(s))
        } else {
            None
        }
    }
}

impl Iterator for VectorIt {
    type Item = Vector3D<i32>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.z < self.max {
            if self.y < self.max {
                if self.x < self.max {
                    let x = self.x;
                    self.x += 1;
                    Some(Vector3D {
                        x,
                        y: self.y,
                        z: self.z,
                    })
                } else {
                    let y = self.y;
                    self.x = 0;
                    self.y += 1;
                    Some(Vector3D {
                        x: self.x,
                        y,
                        z: self.z,
                    })
                }
            } else {
                let z = self.z;
                self.x = 0;
                self.y = 0;
                self.z += 1;
                Some(Vector3D {
                    x: self.x,
                    y: self.y,
                    z,
                })
            }
        } else {
            None
        }
    }
}

impl Iterator for BivectorIt {
    type Item = Bivector3D<i32>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.xy < self.max {
            if self.zx < self.max {
                if self.yz < self.max {
                    let yz = self.yz;
                    self.yz += 1;
                    Some(Bivector3D {
                        yz,
                        zx: self.zx,
                        xy: self.xy,
                    })
                } else {
                    let zx = self.zx;
                    self.yz = 0;
                    self.zx += 1;
                    Some(Bivector3D {
                        yz: self.yz,
                        zx,
                        xy: self.xy,
                    })
                }
            } else {
                let xy = self.xy;
                self.yz = 0;
                self.zx = 0;
                self.xy += 1;
                Some(Bivector3D {
                    yz: self.yz,
                    zx: self.zx,
                    xy,
                })
            }
        } else {
            None
        }
    }
}

impl Iterator for TrivectorIt {
    type Item = Trivector3D<i32>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.xyz < self.max {
            let xyz = self.xyz;
            self.xyz += 1;
            Some(Trivector3D { xyz })
        } else {
            None
        }
    }
}

#[test]
fn complement_scalar() {
    let i: Trivector3D<i32> = Trivector3D::UNIT_VOLUME;

    let s = Scalar3D::ONE;
    assert_eq!(s.wedge(s.right_complement()), i);
    assert_eq!(s.left_complement().wedge(s), i);
}

#[test]
fn complement_vector() {
    let i: Trivector3D<i32> = Trivector3D::UNIT_VOLUME;

    let v = Vector3D::X;
    assert_eq!(v.wedge(v.right_complement()), i);
    assert_eq!(v.left_complement().wedge(v), i);

    let v = Vector3D::Y;
    assert_eq!(v.wedge(v.right_complement()), i);
    assert_eq!(v.left_complement().wedge(v), i);

    let v = Vector3D::Z;
    assert_eq!(v.wedge(v.right_complement()), i);
    assert_eq!(v.left_complement().wedge(v), i);
}

#[test]
fn complement_bivector() {
    let i: Trivector3D<i32> = Trivector3D::UNIT_VOLUME;

    let b = Bivector3D::YZ;
    assert_eq!(b.wedge(b.right_complement()), i);
    assert_eq!(b.left_complement().wedge(b), i);

    let b = Bivector3D::ZX;
    assert_eq!(b.wedge(b.right_complement()), i);
    assert_eq!(b.left_complement().wedge(b), i);

    let b = Bivector3D::XY;
    assert_eq!(b.wedge(b.right_complement()), i);
    assert_eq!(b.left_complement().wedge(b), i);
}

#[test]
fn complement_trivector() {
    let i: Trivector3D<i32> = Trivector3D::UNIT_VOLUME;

    let t = Trivector3D::XYZ;
    assert_eq!(t.wedge(t.right_complement()), i);
    assert_eq!(t.left_complement().wedge(t), i);
}

#[test]
fn wedge_scalar_scalar() {
    for s in ScalarIt::new(100) {
        for s2 in ScalarIt::new(100) {
            // scalars always commute
            assert_eq!(s.wedge(s2), s2.wedge(s));
        }
    }
}

#[test]
fn wedge_scalar_vector() {
    for s in ScalarIt::new(100) {
        for v in VectorIt::new(20) {
            // scalars always commute
            assert_eq!(s.wedge(v), v.wedge(s));
        }
    }
}

#[test]
fn wedge_scalar_bivector() {
    for s in ScalarIt::new(100) {
        for b in BivectorIt::new(20) {
            // scalars always commute
            assert_eq!(s.wedge(b), b.wedge(s));
        }
    }
}

#[test]
fn wedge_vector_vector() {
    for v in VectorIt::new(20) {
        for v2 in VectorIt::new(20) {
            // vector - vector anticommute
            assert_eq!(v.wedge(v2), -v2.wedge(v));
        }
    }
}

#[test]
fn wedge_vector_bivector() {
    for v in VectorIt::new(20) {
        for b in BivectorIt::new(20) {
            // bivectors always commute
            assert_eq!(v.wedge(b), b.wedge(v));
        }
    }
}

#[test]
fn antiwedge_scalar_trivector() {
    for s in ScalarIt::new(100) {
        for t in TrivectorIt::new(100) {
            // actual implementation matches definition
            assert_eq!(s.antiwedge(t), antiwedge_reference(s, t));
            assert_eq!(t.antiwedge(s), antiwedge_reference(t, s));
        }
    }
}

#[test]
fn antiwedge_vector_bivector() {
    for v in VectorIt::new(20) {
        for b in BivectorIt::new(20) {
            // actual implementation matches definition
            assert_eq!(v.antiwedge(b), antiwedge_reference(v, b));
            assert_eq!(b.antiwedge(v), antiwedge_reference(b, v));
        }
    }
}

#[test]
fn antiwedge_vector_trivector() {
    for v in VectorIt::new(20) {
        for t in TrivectorIt::new(100) {
            // actual implementation matches definition
            assert_eq!(v.antiwedge(t), antiwedge_reference(v, t));
            assert_eq!(t.antiwedge(v), antiwedge_reference(t, v));
        }
    }
}

#[test]
fn antiwedge_bivector_bivector() {
    for b1 in BivectorIt::new(20) {
        for b2 in BivectorIt::new(20) {
            // actual implementation matches definition
            assert_eq!(b1.antiwedge(b2), antiwedge_reference(b1, b2));
            assert_eq!(b2.antiwedge(b1), antiwedge_reference(b2, b1));
        }
    }
}

#[test]
fn antiwedge_bivector_trivector() {
    for b in BivectorIt::new(20) {
        for t in TrivectorIt::new(100) {
            // actual implementation matches definition
            assert_eq!(b.antiwedge(t), antiwedge_reference(b, t));
            assert_eq!(t.antiwedge(b), antiwedge_reference(t, b));
        }
    }
}

#[test]
fn antiwedge_trivector_trivector() {
    for t1 in TrivectorIt::new(100) {
        for t2 in TrivectorIt::new(100) {
            // actual implementation matches definition
            assert_eq!(t1.antiwedge(t2), antiwedge_reference(t1, t2));
            assert_eq!(t2.antiwedge(t1), antiwedge_reference(t2, t1));
        }
    }
}
