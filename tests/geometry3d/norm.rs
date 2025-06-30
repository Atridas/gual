use gual::{Dot, Epsilon, Euclidean, Metric, Norm, Projective};

use crate::geometry3d::{ToF32, TrivectorIt};

use super::{BivectorIt, VectorIt};

#[test]
fn euclidean_norm_vector() {
    for v in VectorIt::<Euclidean>::new(50) {
        let v = v.to_f32();
        assert_eq!(v.bulk_norm_squared(), v.bulk().dot(&v.bulk()));
        assert!((v.bulk_norm() - v.bulk_norm_squared().sqrt()).is_near_zero());

        assert_eq!(v.weight_norm_squared(), v.weight().antidot(&v.weight()));
        assert!((v.weight_norm().xyz - v.weight_norm_squared().xyz.sqrt()).is_near_zero());
    }
}

#[test]
fn euclidean_norm_bivector() {
    for b in BivectorIt::<Euclidean>::new(50) {
        let b = b.to_f32();
        assert_eq!(b.bulk_norm_squared(), b.bulk().dot(&b.bulk()));
        assert!((b.bulk_norm() - b.bulk_norm_squared().sqrt()).is_near_zero());

        assert_eq!(b.weight_norm_squared(), b.weight().antidot(&b.weight()));
        assert!((b.weight_norm().xyz - b.weight_norm_squared().xyz.sqrt()).is_near_zero());
    }
}

#[test]
fn euclidean_norm_trivector() {
    for t in TrivectorIt::<Euclidean>::new(50) {
        let t = t.to_f32();
        assert_eq!(t.bulk_norm_squared(), t.bulk().dot(&t.bulk()));
        assert!((t.bulk_norm() - t.bulk_norm_squared().sqrt()).is_near_zero());

        assert_eq!(t.weight_norm_squared(), t.weight().antidot(&t.weight()));
        assert!((t.weight_norm().xyz - t.weight_norm_squared().xyz.sqrt()).is_near_zero());
    }
}

#[test]
fn projective_norm_vector() {
    for v in VectorIt::<Projective>::new(50) {
        let v = v.to_f32();
        assert_eq!(v.bulk_norm_squared(), v.bulk().dot(&v.bulk()));
        assert!((v.bulk_norm() - v.bulk_norm_squared().sqrt()).is_near_zero());

        assert_eq!(
            v.weight_norm_squared(),
            v.proper_weight().antidot(&v.proper_weight())
        );
        assert!((v.weight_norm().xyz - v.weight_norm_squared().xyz.sqrt()).is_near_zero());
    }
}

#[test]
fn projective_norm_bivector() {
    for b in BivectorIt::<Projective>::new(50) {
        let b = b.to_f32();
        assert_eq!(b.bulk_norm_squared(), b.bulk().dot(&b.bulk()));
        assert!((b.bulk_norm() - b.bulk_norm_squared().sqrt()).is_near_zero());

        assert_eq!(
            b.weight_norm_squared(),
            b.proper_weight().antidot(&b.proper_weight())
        );
        assert!((b.weight_norm().xyz - b.weight_norm_squared().xyz.sqrt()).is_near_zero());
    }
}

#[test]
fn projective_norm_trivector() {
    for t in TrivectorIt::<Projective>::new(50) {
        let t = t.to_f32();
        assert_eq!(t.bulk_norm_squared(), t.proper_bulk().dot(&t.proper_bulk()));
        assert!((t.bulk_norm() - t.bulk_norm_squared().sqrt()).is_near_zero());

        assert_eq!(
            t.weight_norm_squared(),
            t.proper_weight().antidot(&t.proper_weight())
        );
        assert!((t.weight_norm().xyz - t.weight_norm_squared().xyz.sqrt()).is_near_zero());
    }
}
