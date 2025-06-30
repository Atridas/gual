use gual::{Dot, Epsilon, Metric, Norm};

use crate::geometry2d::ToF32;

use super::{BivectorIt, VectorIt};

#[test]
fn norm_vector() {
    for v in VectorIt::new(50) {
        let v = v.to_f32();
        assert_eq!(v.bulk_norm_squared(), v.bulk().dot(&v.bulk()));
        assert!((v.bulk_norm() - v.bulk_norm_squared().sqrt()).is_near_zero());

        assert_eq!(v.weight_norm_squared(), v.weight().antidot(&v.weight()));
        assert!((v.weight_norm().xy - v.weight_norm_squared().xy.sqrt()).is_near_zero());
    }
}

#[test]
fn norm_bivector() {
    for b in BivectorIt::new(50) {
        let b = b.to_f32();
        assert_eq!(b.bulk_norm_squared(), b.bulk().dot(&b.bulk()));
        assert!((b.bulk_norm() - b.bulk_norm_squared().sqrt()).is_near_zero());

        assert_eq!(b.weight_norm_squared(), b.weight().antidot(&b.weight()));
        assert!((b.weight_norm().xy - b.weight_norm_squared().xy.sqrt()).is_near_zero());
    }
}
