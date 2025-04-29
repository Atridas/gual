use gual::{
    Antiscalar, AntiwedgeProduct, KVector, WedgeProduct, antiwedge_reference, geometry2d::*,
};
use num::traits::ConstOne;

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
        if self.y < self.max {
            if self.x < self.max {
                let x = self.x;
                self.x += 1;
                Some(Vector { x, y: self.y })
            } else {
                let y = self.y;
                self.x = 0;
                self.y += 1;
                Some(Vector { x: self.x, y })
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
            Some(Bivector { xy })
        } else {
            None
        }
    }
}

#[test]
fn complement_scalar() {
    let i = Bivector::<f32>::UNIT_VOLUME;

    let s = Scalar::ONE;
    assert_eq!(s.wedge(s.right_complement()), i);
    assert_eq!(s.left_complement().wedge(s), i);
}

#[test]
fn complement_vector() {
    let i = Bivector::<f32>::UNIT_VOLUME;

    let v = Vector::X;
    assert_eq!(v.wedge(v.right_complement()), i);
    assert_eq!(v.left_complement().wedge(v), i);

    let v = Vector::Y;
    assert_eq!(v.wedge(v.right_complement()), i);
    assert_eq!(v.left_complement().wedge(v), i);
}

#[test]
fn complement_bivector() {
    let i = Bivector::<f32>::UNIT_VOLUME;

    let b = Bivector::XY;
    assert_eq!(b.wedge(b.right_complement()), i);
    assert_eq!(b.left_complement().wedge(b), i);
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
        for v in VectorIt::new(50) {
            // scalars always commute
            assert_eq!(s.wedge(v), v.wedge(s));
        }
    }
}

#[test]
fn wedge_scalar_bivector() {
    for s in ScalarIt::new(100) {
        for b in BivectorIt::new(100) {
            // scalars always commute
            assert_eq!(s.wedge(b), b.wedge(s));
        }
    }
}

#[test]
fn wedge_vector_bivector() {
    for v in VectorIt::new(50) {
        for v2 in VectorIt::new(50) {
            // vector - vector anticommute
            assert_eq!(v.wedge(v2), -v2.wedge(v));
        }
    }
}

#[test]
fn antiwedge_scalar_bivector() {
    for s in ScalarIt::new(100) {
        for b in BivectorIt::new(100) {
            // actual implementation matches definition
            assert_eq!(s.antiwedge(b), antiwedge_reference(s, b));
            assert_eq!(b.antiwedge(s), antiwedge_reference(b, s));
        }
    }
}

#[test]
fn antiwedge_vector_vector() {
    for a in VectorIt::new(50) {
        for b in VectorIt::new(50) {
            // actual implementation matches definition
            assert_eq!(a.antiwedge(b), antiwedge_reference(a, b));
            assert_eq!(b.antiwedge(a), antiwedge_reference(b, a));
        }
    }
}

#[test]
fn antiwedge_vector_bivector() {
    for v in VectorIt::new(50) {
        for b in BivectorIt::new(100) {
            // actual implementation matches definition
            assert_eq!(v.antiwedge(b), antiwedge_reference(v, b));
            assert_eq!(b.antiwedge(v), antiwedge_reference(b, v));
        }
    }
}

#[test]
fn antiwedge_bivector_bivector() {
    for a in BivectorIt::new(100) {
        for b in BivectorIt::new(100) {
            // actual implementation matches definition
            assert_eq!(a.antiwedge(b), antiwedge_reference(a, b));
            assert_eq!(b.antiwedge(a), antiwedge_reference(b, a));
        }
    }
}
