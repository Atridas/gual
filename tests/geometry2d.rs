use gual::{Antiscalar, AntiwedgeProduct, Complement, Scalar, WedgeProduct, geometry2d::*};

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

#[test]
fn complement_scalar() {
    let i = Bivector::<f32>::UNIT_VOLUME;

    let s = Scalar::<2, f32>::ONE;
    assert_eq!(s.wedge(&s.right_complement()), i);
    assert_eq!(s.left_complement().wedge(&s), i);
}

#[test]
fn complement_vector() {
    let i = Bivector::<f32>::UNIT_VOLUME;

    let v = Vector::X;
    assert_eq!(v.wedge(&v.right_complement()), i);
    assert_eq!(v.left_complement().wedge(&v), i);

    let v = Vector::Y;
    assert_eq!(v.wedge(&v.right_complement()), i);
    assert_eq!(v.left_complement().wedge(&v), i);
}

#[test]
fn complement_bivector() {
    let i = Bivector::<f32>::UNIT_VOLUME;

    let b = Bivector::<f32>::XY;
    assert_eq!(b.wedge(&b.right_complement()), i);
    assert_eq!(b.left_complement().wedge(&b), i);
}

#[test]
fn wedge_scalar_vector() {
    for s in ScalarIt::new(100) {
        for v in VectorIt::new(50) {
            // scalars always commute
            assert_eq!(s.wedge(&v), v.wedge(&s));
        }
    }
}

#[test]
fn wedge_scalar_bivector() {
    for s in ScalarIt::new(100) {
        for b in BivectorIt::new(100) {
            // scalars always commute
            assert_eq!(s.wedge(&b), b.wedge(&s));
        }
    }
}

#[test]
fn wedge_vector_bivector() {
    for v in VectorIt::new(50) {
        for v2 in VectorIt::new(50) {
            // vector - vector anticommute
            assert_eq!(v.wedge(&v2), -v2.wedge(&v));
        }
    }
}

pub fn antiwedge<Lhs, Rhs>(lhs: Lhs, rhs: Rhs) -> <<<Lhs as Complement>::Output as WedgeProduct<<Rhs as Complement>::Output>>::Output as Complement>::Output
where
    Lhs: Complement,
    Rhs: Complement,
    <Lhs as Complement>::Output: WedgeProduct<<Rhs as Complement>::Output>,
    <<Lhs as Complement>::Output as WedgeProduct<<Rhs as Complement>::Output>>::Output: Complement,
{
    lhs.left_complement()
        .wedge(&rhs.left_complement())
        .right_complement()
}

#[test]
fn antiwedge_scalar_bivector() {
    for s in ScalarIt::new(100) {
        let sc: Scalar<2, _> = Scalar::new(s);
        for b in BivectorIt::new(100) {
            // actual implementation matches definition
            assert_eq!(s.antiwedge(&b), antiwedge(sc, b));
            assert_eq!(b.antiwedge(&s), antiwedge(b, sc));
        }
    }
}

#[test]
fn antiwedge_vector_vector() {
    for a in VectorIt::new(50) {
        for b in VectorIt::new(50) {
            // actual implementation matches definition
            assert_eq!(a.antiwedge(&b), antiwedge(a, b));
            assert_eq!(b.antiwedge(&a), antiwedge(b, a));
        }
    }
}

#[test]
fn antiwedge_vector_bivector() {
    for v in VectorIt::new(50) {
        for b in BivectorIt::new(100) {
            // actual implementation matches definition
            assert_eq!(v.antiwedge(&b), antiwedge(v, b));
            assert_eq!(b.antiwedge(&v), antiwedge(b, v));
        }
    }
}

#[test]
fn antiwedge_bivector_bivector() {
    let antiwedge = |a: Bivector<i32>, b: Bivector<i32>| {
        let s = a.left_complement() * b.left_complement();
        Scalar::<2, _>::new(s).right_complement()
    };

    for a in BivectorIt::new(100) {
        for b in BivectorIt::new(100) {
            // actual implementation matches definition
            assert_eq!(a.antiwedge(&b), antiwedge(a, b));
            assert_eq!(b.antiwedge(&a), antiwedge(b, a));
        }
    }
}
