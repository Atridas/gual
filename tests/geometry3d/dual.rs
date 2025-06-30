use gual::{
    Complement, Dual, Euclidean, Metric, Projective,
    canonical::{
        canonical_left_bulk_dual, canonical_left_weight_dual, canonical_right_bulk_dual,
        canonical_right_weight_dual,
    },
    geometry3d::Trivector,
};

use crate::geometry3d::{BivectorIt, TrivectorIt, VectorIt};

#[test]
fn dual_euclidean_vector() {
    for v in VectorIt::<Euclidean>::new(100) {
        assert_eq!(v.right_bulk_dual(), canonical_right_bulk_dual(v));
        assert_eq!(v.left_bulk_dual(), canonical_left_bulk_dual(v));
        assert_eq!(v.right_weight_dual(), canonical_right_weight_dual(v));
        assert_eq!(v.left_weight_dual(), canonical_left_weight_dual(v));
    }
}

#[test]
fn dual_euclidean_bivector() {
    for b in BivectorIt::<Euclidean>::new(100) {
        assert_eq!(b.right_bulk_dual(), canonical_right_bulk_dual(b));
        assert_eq!(b.left_bulk_dual(), canonical_left_bulk_dual(b));
        assert_eq!(b.right_weight_dual(), canonical_right_weight_dual(b));
        assert_eq!(b.left_weight_dual(), canonical_left_weight_dual(b));
    }
}

#[test]
fn dual_euclidean_trivector() {
    for v in TrivectorIt::<Euclidean>::new(100) {
        assert_eq!(
            v.left_bulk_dual(),
            Trivector::<i32, Euclidean>::from_bulk(&v.bulk()).left_complement()
        );
        assert_eq!(
            v.right_bulk_dual(),
            Trivector::<i32, Euclidean>::from_bulk(&v.bulk()).right_complement()
        );
        assert_eq!(
            v.left_weight_dual(),
            Trivector::<i32, Euclidean>::from_weight(&v.weight()).left_complement()
        );
        assert_eq!(
            v.right_weight_dual(),
            Trivector::<i32, Euclidean>::from_weight(&v.weight()).right_complement()
        );
    }
}

#[test]
fn dual_projective_vector() {
    for v in VectorIt::<Projective>::new(100) {
        assert_eq!(v.right_bulk_dual(), canonical_right_bulk_dual(v));
        assert_eq!(v.left_bulk_dual(), canonical_left_bulk_dual(v));
        assert_eq!(v.right_weight_dual(), canonical_right_weight_dual(v));
        assert_eq!(v.left_weight_dual(), canonical_left_weight_dual(v));
    }
}

#[test]
fn dual_projective_bivector() {
    for b in BivectorIt::<Projective>::new(100) {
        assert_eq!(b.right_bulk_dual(), canonical_right_bulk_dual(b));
        assert_eq!(b.left_bulk_dual(), canonical_left_bulk_dual(b));
        assert_eq!(b.right_weight_dual(), canonical_right_weight_dual(b));
        assert_eq!(b.left_weight_dual(), canonical_left_weight_dual(b));
    }
}

#[test]
fn dual_projective_trivector() {
    for v in TrivectorIt::<Projective>::new(100) {
        assert_eq!(
            v.left_bulk_dual(),
            Trivector::<i32, Projective>::from_bulk(&v.bulk()).left_complement()
        );
        assert_eq!(
            v.right_bulk_dual(),
            Trivector::<i32, Projective>::from_bulk(&v.bulk()).right_complement()
        );
        assert_eq!(
            v.left_weight_dual(),
            Trivector::<i32, Projective>::from_weight(&v.weight()).left_complement()
        );
        assert_eq!(
            v.right_weight_dual(),
            Trivector::<i32, Projective>::from_weight(&v.weight()).right_complement()
        );
    }
}
