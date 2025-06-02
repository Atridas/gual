use gual::{
    Antiscalar, Complement, Scalar, WedgeProduct,
    geometry2d::{Bivector, Vector},
};

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
