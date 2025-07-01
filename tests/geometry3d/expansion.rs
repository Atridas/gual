use gual::canonical::Expansion as CanonicalExpansion;
use gual::{Euclidean, Expansion, Projective};

use crate::geometry3d::{BivectorIt, TrivectorIt, VectorIt};

#[test]
fn euclidean_bulk_expansion_vector_vector() {
    for v1 in VectorIt::<Euclidean>::new(20) {
        for v2 in VectorIt::<Euclidean>::new(20) {
            assert_eq!(v1.bulk_expansion(&v2), v1.canonical_bulk_expansion(&v2));
            assert_eq!(v1.weight_expansion(&v2), v1.canonical_weight_expansion(&v2));
        }
    }
}

#[test]
fn euclidean_bulk_expansion_vector_bivector() {
    for v in VectorIt::<Euclidean>::new(20) {
        for b in BivectorIt::<Euclidean>::new(20) {
            assert_eq!(v.bulk_expansion(&b), v.canonical_bulk_expansion(&b));
            assert_eq!(v.weight_expansion(&b), v.canonical_weight_expansion(&b));
        }
    }
}

#[test]
fn euclidean_bulk_expansion_bivector_bivector() {
    for b1 in BivectorIt::<Euclidean>::new(20) {
        for b2 in BivectorIt::<Euclidean>::new(20) {
            assert_eq!(b1.bulk_expansion(&b2), b1.canonical_bulk_expansion(&b2));
            assert_eq!(b1.weight_expansion(&b2), b1.canonical_weight_expansion(&b2));
        }
    }
}

#[test]
fn euclidean_bulk_expansion_trivector_trivector() {
    for t1 in TrivectorIt::<Euclidean>::new(20) {
        for t2 in TrivectorIt::<Euclidean>::new(20) {
            assert_eq!(t1.bulk_expansion(&t2), t1.canonical_bulk_expansion(&t2));
            assert_eq!(t1.weight_expansion(&t2), t1.canonical_weight_expansion(&t2));
        }
    }
}

#[test]
fn projective_bulk_expansion_vector_vector() {
    for v1 in VectorIt::<Projective>::new(20) {
        for v2 in VectorIt::<Projective>::new(20) {
            assert_eq!(v1.bulk_expansion(&v2), v1.canonical_bulk_expansion(&v2));
            assert_eq!(v1.weight_expansion(&v2), v1.canonical_weight_expansion(&v2));
        }
    }
}

#[test]
fn projective_bulk_expansion_vector_bivector() {
    for v in VectorIt::<Projective>::new(20) {
        for b in BivectorIt::<Projective>::new(20) {
            assert_eq!(v.bulk_expansion(&b), v.canonical_bulk_expansion(&b));
            assert_eq!(v.weight_expansion(&b), v.canonical_weight_expansion(&b));
        }
    }
}

#[test]
fn projective_bulk_expansion_bivector_bivector() {
    for b1 in BivectorIt::<Projective>::new(20) {
        for b2 in BivectorIt::<Projective>::new(20) {
            assert_eq!(b1.bulk_expansion(&b2), b1.canonical_bulk_expansion(&b2));
            assert_eq!(b1.weight_expansion(&b2), b1.canonical_weight_expansion(&b2));
        }
    }
}

#[test]
fn projective_bulk_expansion_trivector_trivector() {
    for t1 in TrivectorIt::<Projective>::new(20) {
        for t2 in TrivectorIt::<Projective>::new(20) {
            assert_eq!(t1.bulk_expansion(&t2), t1.canonical_bulk_expansion(&t2));
            assert_eq!(t1.weight_expansion(&t2), t1.canonical_weight_expansion(&t2));
        }
    }
}
