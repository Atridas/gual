use gual::{
    Contraction, Euclidean,
    canonical::{canonical_bulk_contraction, canonical_weight_contraction},
};

use crate::geometry3d::{BivectorIt, TrivectorIt, VectorIt};

#[test]
fn euclidean_bulk_contraction_vector_vector() {
    for v1 in VectorIt::<Euclidean>::new(20) {
        for v2 in VectorIt::<Euclidean>::new(20) {
            assert_eq!(v1.bulk_contraction(&v2), canonical_bulk_contraction(v1, v2));
            assert_eq!(
                v1.weight_contraction(&v2),
                canonical_weight_contraction(v1, v2)
            );
        }
    }
}

#[test]
fn euclidean_bulk_contraction_bivector_vector() {
    for b in BivectorIt::<Euclidean>::new(20) {
        for v in VectorIt::<Euclidean>::new(20) {
            assert_eq!(b.bulk_contraction(&v), canonical_bulk_contraction(b, v));
            assert_eq!(b.weight_contraction(&v), canonical_weight_contraction(b, v));
        }
    }
}

#[test]
fn euclidean_bulk_contraction_bivector_bivector() {
    for b1 in BivectorIt::<Euclidean>::new(20) {
        for b2 in BivectorIt::<Euclidean>::new(20) {
            assert_eq!(b1.bulk_contraction(&b2), canonical_bulk_contraction(b1, b2));
            assert_eq!(
                b1.weight_contraction(&b2),
                canonical_weight_contraction(b1, b2)
            );
        }
    }
}

#[test]
fn euclidean_bulk_contraction_trivector_vector() {
    for t in TrivectorIt::<Euclidean>::new(20) {
        for v in VectorIt::<Euclidean>::new(20) {
            assert_eq!(t.bulk_contraction(&v), canonical_bulk_contraction(t, v));
            assert_eq!(t.weight_contraction(&v), canonical_weight_contraction(t, v));
        }
    }
}

#[test]
fn euclidean_bulk_contraction_trivector_bivector() {
    for t in TrivectorIt::<Euclidean>::new(20) {
        for b in BivectorIt::<Euclidean>::new(20) {
            assert_eq!(t.bulk_contraction(&b), canonical_bulk_contraction(t, b));
            assert_eq!(t.weight_contraction(&b), canonical_weight_contraction(t, b));
        }
    }
}

#[test]
fn euclidean_bulk_contraction_trivector_trivector() {
    for t1 in TrivectorIt::<Euclidean>::new(20) {
        for t2 in TrivectorIt::<Euclidean>::new(20) {
            assert_eq!(t1.bulk_contraction(&t2), canonical_bulk_contraction(t1, t2));
            assert_eq!(
                t1.weight_contraction(&t2),
                canonical_weight_contraction(t1, t2)
            );
        }
    }
}
