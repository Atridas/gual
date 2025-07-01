use gual::{Complement, Dual, Metric, geometry2d::Bivector};

use super::{BivectorIt, VectorIt};

use gual::canonical::Metric as CanonicalMetric;

#[test]
fn dual_vector() {
    for v in VectorIt::new(50) {
        assert_eq!(v.right_bulk_dual(), v.canonical_right_bulk_dual());
        assert_eq!(v.left_bulk_dual(), v.canonical_left_bulk_dual());
        assert_eq!(v.right_weight_dual(), v.canonical_right_weight_dual());
        assert_eq!(v.left_weight_dual(), v.canonical_left_weight_dual());
    }
}

#[test]
fn norm_bivector() {
    for b in BivectorIt::new(50) {
        assert_eq!(
            b.left_bulk_dual(),
            Bivector::<i32>::from_bulk(&b.bulk()).left_complement()
        );
        assert_eq!(
            b.right_bulk_dual(),
            Bivector::<i32>::from_bulk(&b.bulk()).right_complement()
        );
        assert_eq!(
            b.left_weight_dual(),
            Bivector::<i32>::from_weight(&b.weight()).left_complement()
        );
        assert_eq!(
            b.right_weight_dual(),
            Bivector::<i32>::from_weight(&b.weight()).right_complement()
        );
    }
}
