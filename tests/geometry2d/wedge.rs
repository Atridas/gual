use gual::WedgeProduct;

use super::{BivectorIt, ScalarIt, VectorIt};

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
