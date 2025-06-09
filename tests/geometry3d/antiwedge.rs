use gual::{AntiwedgeProduct, Complement, Scalar, geometry3d::Trivector};

use crate::{antiwedge, geometry3d::TrivectorIt};

use super::{BivectorIt, ScalarIt, VectorIt};

#[test]
fn antiwedge_vector_bivector() {
    for v in VectorIt::new(20) {
        for b in BivectorIt::new(20) {
            // actual implementation matches definition
            assert_eq!(v.antiwedge(&b), antiwedge(v, b));
            assert_eq!(b.antiwedge(&v), antiwedge(b, v));
        }
    }
}

#[test]
fn antiwedge_vector_trivector() {
    for v in VectorIt::new(20) {
        for t in TrivectorIt::new(100) {
            // actual implementation matches definition
            assert_eq!(v.antiwedge(&t), antiwedge(v, t));
            assert_eq!(t.antiwedge(&v), antiwedge(t, v));
        }
    }
}

#[test]
fn antiwedge_bivector_bivector() {
    for b1 in BivectorIt::new(20) {
        for b2 in BivectorIt::new(20) {
            // actual implementation matches definition
            assert_eq!(b1.antiwedge(&b2), antiwedge(b1, b2));
            assert_eq!(b2.antiwedge(&b1), antiwedge(b2, b1));
        }
    }
}

#[test]
fn antiwedge_bivector_trivector() {
    for b in BivectorIt::new(20) {
        for t in TrivectorIt::new(100) {
            // actual implementation matches definition
            assert_eq!(b.antiwedge(&t), antiwedge(b, t));
            assert_eq!(t.antiwedge(&b), antiwedge(t, b));
        }
    }
}

#[test]
fn antiwedge_trivector_trivector() {
    let antiwedge = |a: Trivector<i32>, b: Trivector<i32>| {
        let s = a.left_complement() * b.left_complement();
        Scalar::<3, _>::new(s).right_complement()
    };

    for t1 in TrivectorIt::new(100) {
        for t2 in TrivectorIt::new(100) {
            // actual implementation matches definition
            assert_eq!(t1.antiwedge(&t2), antiwedge(t1, t2));
            assert_eq!(t2.antiwedge(&t1), antiwedge(t2, t1));
        }
    }
}

#[test]
fn antiwedge_scalar_trivector() {
    for s in ScalarIt::new(100) {
        let sc: Scalar<3, _> = Scalar::new(s);
        for t in TrivectorIt::new(100) {
            // actual implementation matches definition
            assert_eq!(s.antiwedge(&t), antiwedge(sc, t));
            assert_eq!(t.antiwedge(&s), antiwedge(t, sc));
        }
    }
}
