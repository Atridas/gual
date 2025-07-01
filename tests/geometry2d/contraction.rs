use super::{BivectorIt, VectorIt};

use gual::Contraction;
use gual::canonical::Contraction as CanonicalContraction;

#[test]
fn contraction_vector_vector() {
    for v in VectorIt::new(50) {
        for v2 in VectorIt::new(50) {
            assert_eq!(v.bulk_contraction(&v2), v.canonical_bulk_contraction(&v2));
            assert_eq!(
                v.weight_contraction(&v2),
                v.canonical_weight_contraction(&v2)
            );
        }
    }
}

#[test]
fn contraction_bivector_vector() {
    for b in BivectorIt::new(50) {
        for v in VectorIt::new(50) {
            assert_eq!(b.bulk_contraction(&v), b.canonical_bulk_contraction(&v));
            assert_eq!(b.weight_contraction(&v), b.canonical_weight_contraction(&v));
        }
    }
}

#[test]
fn contraction_bivector_bivector() {
    for b in BivectorIt::new(50) {
        for b2 in BivectorIt::new(50) {
            assert_eq!(b.bulk_contraction(&b2), b.canonical_bulk_contraction(&b2));
            assert_eq!(
                b.weight_contraction(&b2),
                b.canonical_weight_contraction(&b2)
            );
        }
    }
}
