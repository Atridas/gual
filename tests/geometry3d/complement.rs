use gual::{
    Antiscalar, Complement, Scalar, WedgeProduct,
    geometry3d::{Bivector, Trivector, Vector},
};
use num::traits::ConstOne;

#[test]
fn complement_scalar() {
    let i = Trivector::<f32>::UNIT_VOLUME;

    let s = Scalar::<3, f32>::ONE;
    assert_eq!(s.wedge(&s.right_complement()), i);
    assert_eq!(s.left_complement().wedge(&s), i);
}

#[test]
fn complement_vector() {
    let i = Trivector::<f32>::UNIT_VOLUME;

    let v = Vector::X;
    assert_eq!(v.wedge(&v.right_complement()), i);
    assert_eq!(v.left_complement().wedge(&v), i);

    let v = Vector::Y;
    assert_eq!(v.wedge(&v.right_complement()), i);
    assert_eq!(v.left_complement().wedge(&v), i);

    let v = Vector::Z;
    assert_eq!(v.wedge(&v.right_complement()), i);
    assert_eq!(v.left_complement().wedge(&v), i);
}

#[test]
fn complement_bivector() {
    let i = Trivector::<f32>::UNIT_VOLUME;

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
    let i = Trivector::<f32>::UNIT_VOLUME;

    let t = Trivector::<f32>::XYZ;
    assert_eq!(t.wedge(&t.right_complement()), i);
    assert_eq!(t.left_complement().wedge(&t), i);
}
