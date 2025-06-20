use gual::{Euclidean, WedgeProduct};

use crate::geometry3d::ScalarIt;

type VectorIt = crate::geometry3d::VectorIt<Euclidean>;
type BivectorIt = crate::geometry3d::BivectorIt<Euclidean>;
type TrivectorIt = crate::geometry3d::TrivectorIt<Euclidean>;

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
        for b in BivectorIt::new(20) {
            // scalars always commute
            assert_eq!(s.wedge(&b), b.wedge(&s));
        }
    }
}

#[test]
fn wedge_scalar_trivector() {
    for s in ScalarIt::new(100) {
        for b in TrivectorIt::new(100) {
            // scalars always commute
            assert_eq!(s.wedge(&b), b.wedge(&s));
        }
    }
}

#[test]
fn wedge_vector_vector() {
    for v in VectorIt::new(20) {
        for v2 in VectorIt::new(20) {
            // vector - vector anticommute
            assert_eq!(v.wedge(&v2), -v2.wedge(&v));
        }
    }
}

#[test]
fn wedge_vector_bivector() {
    for v in VectorIt::new(20) {
        for b in BivectorIt::new(20) {
            // bivectors always commute
            assert_eq!(v.wedge(&b), b.wedge(&v));
        }
    }
}
