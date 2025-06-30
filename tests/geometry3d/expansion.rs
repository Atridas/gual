use gual::{
    Euclidean, Expansion, Projective,
    canonical::{canonical_bulk_expansion, canonical_weight_expansion},
};

use crate::geometry3d::{BivectorIt, TrivectorIt, VectorIt};

#[test]
fn euclidean_bulk_expansion_vector_vector() {
    for v1 in VectorIt::<Euclidean>::new(20) {
        for v2 in VectorIt::<Euclidean>::new(20) {
            assert_eq!(v1.bulk_expansion(&v2), canonical_bulk_expansion(v1, v2));
            assert_eq!(v1.weight_expansion(&v2), canonical_weight_expansion(v1, v2));
        }
    }
}

#[test]
fn euclidean_bulk_expansion_vector_bivector() {
    for v in VectorIt::<Euclidean>::new(20) {
        for b in BivectorIt::<Euclidean>::new(20) {
            assert_eq!(v.bulk_expansion(&b), canonical_bulk_expansion(v, b));
            assert_eq!(v.weight_expansion(&b), canonical_weight_expansion(v, b));
        }
    }
}

#[test]
fn euclidean_bulk_expansion_bivector_bivector() {
    for b1 in BivectorIt::<Euclidean>::new(20) {
        for b2 in BivectorIt::<Euclidean>::new(20) {
            assert_eq!(b1.bulk_expansion(&b2), canonical_bulk_expansion(b1, b2));
            assert_eq!(b1.weight_expansion(&b2), canonical_weight_expansion(b1, b2));
        }
    }
}

#[test]
fn euclidean_bulk_expansion_trivector_trivector() {
    for t1 in TrivectorIt::<Euclidean>::new(20) {
        for t2 in TrivectorIt::<Euclidean>::new(20) {
            assert_eq!(t1.bulk_expansion(&t2), canonical_bulk_expansion(t1, t2));
            assert_eq!(t1.weight_expansion(&t2), canonical_weight_expansion(t1, t2));
        }
    }
}

#[test]
fn projective_bulk_expansion_vector_vector() {
    for v1 in VectorIt::<Projective>::new(20) {
        for v2 in VectorIt::<Projective>::new(20) {
            assert_eq!(v1.bulk_expansion(&v2), canonical_bulk_expansion(v1, v2));
            assert_eq!(v1.weight_expansion(&v2), canonical_weight_expansion(v1, v2));
        }
    }
}

#[test]
fn projective_bulk_expansion_vector_bivector() {
    for v in VectorIt::<Projective>::new(20) {
        for b in BivectorIt::<Projective>::new(20) {
            assert_eq!(v.bulk_expansion(&b), canonical_bulk_expansion(v, b));
            assert_eq!(v.weight_expansion(&b), canonical_weight_expansion(v, b));
        }
    }
}

#[test]
fn projective_bulk_expansion_bivector_bivector() {
    for b1 in BivectorIt::<Projective>::new(20) {
        for b2 in BivectorIt::<Projective>::new(20) {
            assert_eq!(b1.bulk_expansion(&b2), canonical_bulk_expansion(b1, b2));
            assert_eq!(b1.weight_expansion(&b2), canonical_weight_expansion(b1, b2));
        }
    }
}

#[test]
fn projective_bulk_expansion_trivector_trivector() {
    for t1 in TrivectorIt::<Projective>::new(20) {
        for t2 in TrivectorIt::<Projective>::new(20) {
            assert_eq!(t1.bulk_expansion(&t2), canonical_bulk_expansion(t1, t2));
            assert_eq!(t1.weight_expansion(&t2), canonical_weight_expansion(t1, t2));
        }
    }
}
