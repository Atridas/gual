use crate::{bulk_contraction, weight_contraction};

use super::{BivectorIt, VectorIt};

use gual::Contraction;

#[test]
fn contraction_vector_vector() {
    for v in VectorIt::new(50) {
        for v2 in VectorIt::new(50) {
            assert_eq!(v.bulk_contraction(&v2), bulk_contraction(v, v2));
            assert_eq!(v.weight_contraction(&v2), weight_contraction(v, v2));
        }
    }
}

#[test]
fn contraction_bivector_vector() {
    for b in BivectorIt::new(50) {
        for v in VectorIt::new(50) {
            assert_eq!(b.bulk_contraction(&v), bulk_contraction(b, v));
            assert_eq!(b.weight_contraction(&v), weight_contraction(b, v));
        }
    }
}

#[test]
fn contraction_bivector_bivector() {
    for b in BivectorIt::new(50) {
        for b2 in BivectorIt::new(50) {
            assert_eq!(b.bulk_contraction(&b2), bulk_contraction(b, b2));
            assert_eq!(b.weight_contraction(&b2), weight_contraction(b, b2));
        }
    }
}
