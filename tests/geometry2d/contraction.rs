use super::{BivectorIt, VectorIt};

use gual::{
    Contraction, canonical::canonical_bulk_contraction, canonical::canonical_weight_contraction,
};

#[test]
fn contraction_vector_vector() {
    for v in VectorIt::new(50) {
        for v2 in VectorIt::new(50) {
            assert_eq!(v.bulk_contraction(&v2), canonical_bulk_contraction(v, v2));
            assert_eq!(
                v.weight_contraction(&v2),
                canonical_weight_contraction(v, v2)
            );
        }
    }
}

#[test]
fn contraction_bivector_vector() {
    for b in BivectorIt::new(50) {
        for v in VectorIt::new(50) {
            assert_eq!(b.bulk_contraction(&v), canonical_bulk_contraction(b, v));
            assert_eq!(b.weight_contraction(&v), canonical_weight_contraction(b, v));
        }
    }
}

#[test]
fn contraction_bivector_bivector() {
    for b in BivectorIt::new(50) {
        for b2 in BivectorIt::new(50) {
            assert_eq!(b.bulk_contraction(&b2), canonical_bulk_contraction(b, b2));
            assert_eq!(
                b.weight_contraction(&b2),
                canonical_weight_contraction(b, b2)
            );
        }
    }
}
