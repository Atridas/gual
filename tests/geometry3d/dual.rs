use gual::{
    Complement, Dual, Metric, Projective,
    geometry3d::{Bivector, Trivector, Vector},
};

use crate::geometry3d::{BivectorIt, TrivectorIt, VectorIt};

#[test]
fn dual_vector() {
    for v in VectorIt::<Projective>::new(100) {
        assert_eq!(
            v.left_bulk_dual(),
            Vector::from_bulk(&v.bulk()).left_complement()
        );
        assert_eq!(
            v.right_bulk_dual(),
            Vector::from_bulk(&v.bulk()).right_complement()
        );
        assert_eq!(
            v.left_weight_dual(),
            Vector::from_weight(&v.weight()).left_complement()
        );
        assert_eq!(
            v.right_weight_dual(),
            Vector::from_weight(&v.weight()).right_complement()
        );
    }
}

#[test]
fn dual_bivector() {
    for v in BivectorIt::<Projective>::new(100) {
        assert_eq!(
            v.left_bulk_dual(),
            Bivector::from_bulk(&v.bulk()).left_complement()
        );
        assert_eq!(
            v.right_bulk_dual(),
            Bivector::from_bulk(&v.bulk()).right_complement()
        );
        assert_eq!(
            v.left_weight_dual(),
            Bivector::from_weight(&v.weight()).left_complement()
        );
        assert_eq!(
            v.right_weight_dual(),
            Bivector::from_weight(&v.weight()).right_complement()
        );
    }
}

#[test]
fn dual_trivector() {
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
