use gual::{Antiscalar, KVector, WedgeProduct, geometry4d::*};
use num::traits::ConstOne;

struct ScalarIt {
    s: i32,
    max: i32,
}

struct VectorIt {
    i: i32,
    max: i32,
    total: i32,
}

struct BivectorIt {
    i: i32,
    max: i32,
    total: i32,
}

struct TrivectorIt {
    i: i32,
    max: i32,
    total: i32,
}

struct QuadvectorIt {
    xyzw: i32,
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
            i: 0,
            max,
            total: max * max * max * max,
        }
    }
}

impl BivectorIt {
    fn new(max: i32) -> Self {
        Self {
            i: 0,
            max,
            total: max * max * max * max * max * max,
        }
    }
}

impl TrivectorIt {
    fn new(max: i32) -> Self {
        Self {
            i: 0,
            max,
            total: max * max * max * max,
        }
    }
}

impl QuadvectorIt {
    fn new(max: i32) -> Self {
        Self { xyzw: 0, max }
    }
}

impl Iterator for ScalarIt {
    type Item = Scalar<i32>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.s < self.max {
            let s = self.s;
            self.s += 1;
            Some(Scalar(s))
        } else {
            None
        }
    }
}

impl Iterator for VectorIt {
    type Item = Vector<i32>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.total {
            let mut i = self.i;
            let x = i % self.max;
            i /= self.max;
            let y = i % self.max;
            i /= self.max;
            let z = i % self.max;
            i /= self.max;
            let w = i;
            self.i += 1;
            Some(Vector { x, y, z, w })
        } else {
            None
        }
    }
}

impl Iterator for BivectorIt {
    type Item = Bivector<i32>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.total {
            let mut i = self.i;
            let wx = i % self.max;
            i /= self.max;
            let wy = i % self.max;
            i /= self.max;
            let wz = i % self.max;
            i /= self.max;
            let yz = i % self.max;
            i /= self.max;
            let zx = i % self.max;
            i /= self.max;
            let xy = i;
            self.i += 1;
            Some(Bivector {
                wx,
                wy,
                wz,
                yz,
                zx,
                xy,
            })
        } else {
            None
        }
    }
}

impl Iterator for TrivectorIt {
    type Item = Trivector<i32>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.total {
            let mut i = self.i;
            let wyz = i % self.max;
            i /= self.max;
            let wzx = i % self.max;
            i /= self.max;
            let wxy = i % self.max;
            i /= self.max;
            let zyx = i;
            self.i += 1;
            Some(Trivector { wyz, wzx, wxy, zyx })
        } else {
            None
        }
    }
}

impl Iterator for QuadvectorIt {
    type Item = Quadvector<i32>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.xyzw < self.max {
            let xyzw = self.xyzw;
            self.xyzw += 1;
            Some(Quadvector { xyzw })
        } else {
            None
        }
    }
}

#[test]
fn complement_scalar() {
    let i = Quadvector::<f32>::UNIT_VOLUME;

    let s = Scalar::ONE;
    assert_eq!(s.wedge(&s.right_complement()), i);
    assert_eq!(s.left_complement().wedge(&s), i);
}

#[test]
fn complement_vector() {
    let i = Quadvector::<f32>::UNIT_VOLUME;

    let v = Vector::X;
    assert_eq!(v.wedge(&v.right_complement()), i);
    assert_eq!(v.left_complement().wedge(&v), i);

    let v = Vector::Y;
    assert_eq!(v.wedge(&v.right_complement()), i);
    assert_eq!(v.left_complement().wedge(&v), i);

    let v = Vector::Z;
    assert_eq!(v.wedge(&v.right_complement()), i);
    assert_eq!(v.left_complement().wedge(&v), i);

    let v = Vector::W;
    assert_eq!(v.wedge(&v.right_complement()), i);
    assert_eq!(v.left_complement().wedge(&v), i);
}

#[test]
fn complement_bivector() {
    let i = Quadvector::<f32>::UNIT_VOLUME;

    let b = Bivector::WX;
    assert_eq!(b.wedge(&b.right_complement()), i);
    assert_eq!(b.left_complement().wedge(&b), i);

    let b = Bivector::WY;
    assert_eq!(b.wedge(&b.right_complement()), i);
    assert_eq!(b.left_complement().wedge(&b), i);

    let b = Bivector::WZ;
    assert_eq!(b.wedge(&b.right_complement()), i);
    assert_eq!(b.left_complement().wedge(&b), i);

    let b = Bivector::YZ;
    assert_eq!(b.wedge(&b.right_complement()), i);
    assert_eq!(b.left_complement().wedge(&b), i);

    let b = Bivector::ZX;
    assert_eq!(b.wedge(&b.right_complement()), i);
    assert_eq!(b.left_complement().wedge(&b), i);

    let b = Bivector::XY;
    assert_eq!(b.wedge(&b.right_complement()), i);
    assert_eq!(b.left_complement().wedge(&b), i);
}

#[test]
fn complement_trivector() {
    let i = Quadvector::<f32>::UNIT_VOLUME;

    let t = Trivector::WYZ;
    assert_eq!(t.wedge(&t.right_complement()), i);
    assert_eq!(t.left_complement().wedge(&t), i);

    let t = Trivector::WZX;
    assert_eq!(t.wedge(&t.right_complement()), i);
    assert_eq!(t.left_complement().wedge(&t), i);

    let t = Trivector::WXY;
    assert_eq!(t.wedge(&t.right_complement()), i);
    assert_eq!(t.left_complement().wedge(&t), i);

    let t = Trivector::ZYX;
    assert_eq!(t.wedge(&t.right_complement()), i);
    assert_eq!(t.left_complement().wedge(&t), i);
}

#[test]
fn complement_quadvector() {
    let i = Quadvector::<f32>::UNIT_VOLUME;

    let q = Quadvector::XYZW;
    assert_eq!(q.wedge(&q.right_complement()), i);
    assert_eq!(q.left_complement().wedge(&q), i);
}

#[test]
fn wedge_scalar_scalar() {
    for s in ScalarIt::new(100) {
        for s2 in ScalarIt::new(100) {
            // scalars always commute
            assert_eq!(s.wedge(&s2), s2.wedge(&s));
        }
    }
}

#[test]
fn wedge_scalar_vector() {
    for s in ScalarIt::new(100) {
        for v in VectorIt::new(20) {
            // scalars always commute
            assert_eq!(s.wedge(&v), v.wedge(&s));
        }
    }
}

#[test]
fn wedge_scalar_bivector() {
    for s in ScalarIt::new(100) {
        for b in BivectorIt::new(10) {
            // scalars always commute
            assert_eq!(s.wedge(&b), b.wedge(&s));
        }
    }
}

#[test]
fn wedge_scalar_trivector() {
    for s in ScalarIt::new(100) {
        for t in TrivectorIt::new(20) {
            // scalars always commute
            assert_eq!(s.wedge(&t), t.wedge(&s));
        }
    }
}

#[test]
fn wedge_scalar_quadvector() {
    for s in ScalarIt::new(100) {
        for q in QuadvectorIt::new(100) {
            // scalars always commute
            assert_eq!(s.wedge(&q), q.wedge(&s));
        }
    }
}

#[test]
fn wedge_vector_vector() {
    for v in VectorIt::new(10) {
        for v2 in VectorIt::new(10) {
            // vector - vector anticommute
            assert_eq!(v.wedge(&v2), -v2.wedge(&v));
        }
    }
}

#[test]
fn wedge_vector_bivector() {
    for v in VectorIt::new(7) {
        for b in BivectorIt::new(5) {
            // bivectors always commute
            assert_eq!(v.wedge(&b), b.wedge(&v));
        }
    }
}

#[test]
fn wedge_vector_trivector() {
    for v in VectorIt::new(10) {
        for t in TrivectorIt::new(10) {
            // vectors and trivectors anticommute
            assert_eq!(v.wedge(&t), -t.wedge(&v));
        }
    }
}

// #[test]
// fn antiwedge_scalar_quadvector() {
//     for s in ScalarIt::new(100) {
//         for q in QuadvectorIt::new(100) {
//             // actual implementation matches definition
//             assert_eq!(s.antiwedge(&q), canonical_antiwedge(s, q));
//             assert_eq!(q.antiwedge(&s), canonical_antiwedge(q, s));
//         }
//     }
// }

// #[test]
// fn antiwedge_vector_trivector() {
//     for v in VectorIt::new(10) {
//         for t in TrivectorIt::new(10) {
//             // actual implementation matches definition
//             assert_eq!(v.antiwedge(&t), canonical_antiwedge(v, t));
//             assert_eq!(t.antiwedge(&v), canonical_antiwedge(t, v));
//         }
//     }
// }

// #[test]
// fn antiwedge_vector_quadvector() {
//     for v in VectorIt::new(10) {
//         for q in QuadvectorIt::new(10) {
//             // actual implementation matches definition
//             assert_eq!(v.antiwedge(&q), canonical_antiwedge(v, q));
//             assert_eq!(q.antiwedge(&v), canonical_antiwedge(q, v));
//         }
//     }
// }

// #[test]
// fn antiwedge_bivector_bivector() {
//     for b1 in BivectorIt::new(5) {
//         for b2 in BivectorIt::new(5) {
//             // actual implementation matches definition
//             assert_eq!(b1.antiwedge(&b2), canonical_antiwedge(b1, b2));
//             assert_eq!(b2.antiwedge(&b1), canonical_antiwedge(b2, b1));
//         }
//     }
// }

// #[test]
// fn antiwedge_bivector_trivector() {
//     for b in BivectorIt::new(5) {
//         for t in TrivectorIt::new(10) {
//             // actual implementation matches definition
//             assert_eq!(b.antiwedge(&t), canonical_antiwedge(b, t));
//             assert_eq!(t.antiwedge(&b), canonical_antiwedge(t, b));
//         }
//     }
// }

// #[test]
// fn antiwedge_bivector_quadvector() {
//     for b in VectorIt::new(20) {
//         for q in QuadvectorIt::new(100) {
//             // actual implementation matches definition
//             assert_eq!(b.antiwedge(&q), canonical_antiwedge(b, q));
//             assert_eq!(q.antiwedge(&b), canonical_antiwedge(q, b));
//         }
//     }
// }

// #[test]
// fn antiwedge_trivector_quadvector() {
//     for q1 in QuadvectorIt::new(100) {
//         for q2 in QuadvectorIt::new(100) {
//             // actual implementation matches definition
//             assert_eq!(q1.antiwedge(&q2), canonical_antiwedge(q1, q2));
//             assert_eq!(q2.antiwedge(&q1), canonical_antiwedge(q2, q1));
//         }
//     }
// }
