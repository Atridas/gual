use gual::{AntiwedgeProduct, Complement, Scalar, canonical::Antiwedge, geometry2d::Bivector};

use super::{BivectorIt, ScalarIt, VectorIt};

#[test]
fn antiwedge_scalar_bivector() {
    for s in ScalarIt::new(100) {
        let sc: Scalar<2, _> = Scalar::new(s);
        for b in BivectorIt::new(100) {
            // actual implementation matches definition
            assert_eq!(s.antiwedge(&b), sc.canonical_antiwedge(&b));
            assert_eq!(b.antiwedge(&s), b.canonical_antiwedge(&sc));
        }
    }
}

#[test]
fn antiwedge_vector_vector() {
    for a in VectorIt::new(50) {
        for b in VectorIt::new(50) {
            // actual implementation matches definition
            assert_eq!(a.antiwedge(&b), a.canonical_antiwedge(&b));
            assert_eq!(b.antiwedge(&a), b.canonical_antiwedge(&a));
        }
    }
}

#[test]
fn antiwedge_vector_bivector() {
    for v in VectorIt::new(50) {
        for b in BivectorIt::new(100) {
            // actual implementation matches definition
            assert_eq!(v.antiwedge(&b), v.canonical_antiwedge(&b));
            assert_eq!(b.antiwedge(&v), b.canonical_antiwedge(&v));
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
