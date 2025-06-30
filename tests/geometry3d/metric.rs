use num::Zero;

use gual::{
    Complement, Euclidean, Metric, Projective, Scalar, canonical_weight,
    geometry3d::{Bivector, Trivector, Vector},
};

use crate::geometry3d::TrivectorIt;

use super::{BivectorIt, VectorIt};

#[test]
fn euclidean_metric_vector() {
    for v in VectorIt::<Euclidean>::new(50) {
        assert_eq!(v.proper_bulk(), v.bulk());
        assert_eq!(v.proper_weight(), v.weight());

        assert_eq!(v.proper_weight(), canonical_weight(v));
    }
}

#[test]
fn euclidean_metric_bivector() {
    for b in BivectorIt::<Euclidean>::new(50) {
        assert_eq!(b.proper_bulk(), b.bulk());
        assert_eq!(b.proper_weight(), b.weight());

        assert_eq!(b.proper_weight(), canonical_weight(b));
    }
}

#[test]
fn euclidean_metric_trivector() {
    for t in TrivectorIt::new(50) {
        assert_eq!(t.proper_bulk(), t.bulk());
        assert_eq!(t.proper_weight(), t.weight());

        assert_eq!(
            t.proper_weight(),
            Scalar::<3, _>::new(t.left_complement())
                .proper_bulk()
                .right_complement()
        );
    }
}

#[test]
fn projective_metric_vector() {
    for v in VectorIt::<Projective>::new(50) {
        let bulk = v.bulk();
        let weight = v.weight();

        assert_eq!(v.proper_bulk(), Vector::new(bulk.x, bulk.y, 0));
        assert_eq!(v.proper_weight(), Vector::new(0, 0, weight));

        assert_eq!(v.proper_weight(), canonical_weight(v));
    }
}

#[test]
fn projective_metric_bivector() {
    for b in BivectorIt::<Projective>::new(50) {
        let bulk = b.bulk();
        let weight = b.weight();

        assert_eq!(b.proper_bulk(), Bivector::new(0, 0, bulk.xy));
        assert_eq!(b.proper_weight(), Bivector::new(weight.x, weight.y, 0));

        assert_eq!(b.proper_weight(), canonical_weight(b));
    }
}

#[test]
fn projective_metric_trivector() {
    for t in TrivectorIt::<Projective>::new(50) {
        //let bulk : () = t.bulk();
        let weight = t.weight();

        assert!(t.proper_bulk().is_zero());
        assert_eq!(t.proper_weight(), Trivector::new(weight.xy));

        assert_eq!(
            t.proper_weight(),
            Scalar::<3, _, Projective>::new(t.left_complement())
                .proper_bulk()
                .right_complement()
        );
    }
}
